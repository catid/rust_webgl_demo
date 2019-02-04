use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web;
use webgl_rendering_context::{
    WebGLRenderingContext as WebGL,
    WebGLUniformLocation,
    WebGLBuffer,
    WebGLShader,
    WebGLProgram,
};
use stdweb::web::{
    IEventTarget,
    IHtmlElement,
    IParentNode,
    document,
    //window,
    TypedArray,
};
use stdweb::web::html_element::CanvasElement;
use std::rc::Rc;

/*
    WebGL Context with Left-Handed Projection Matrix
*/
pub struct Context {
    canvas: CanvasElement,
    webgl: WebGL,
    width: i32,
    height: i32,
    projection_matrix: nalgebra_glm::TMat4<f32>,
}

impl Context {
    pub fn new(element_id: &str) -> Self {
        let canvas : CanvasElement = document().query_selector(&element_id).unwrap().unwrap().try_into().unwrap();
        let webgl : WebGL = canvas.get_context().unwrap();

        webgl.enable(WebGL::CULL_FACE);
        webgl.enable(WebGL::DEPTH_TEST);

        let thiz = Self {
            canvas: canvas,
            webgl: webgl,
            width: 0,
            height: 0,
            projection_matrix: nalgebra_glm::TMat4::identity(),
        };
        thiz.resize();
        thiz.clear();
        thiz
    }

    // Following the guide here:
    // https://webglfundamentals.org/webgl/lessons/webgl-anti-patterns.html
    // We should not react to the resize event to update the canvas size.
    // Instead we should check the canvas size each 
    pub fn resize(&mut self) {
        let width = self.canvas.offset_width() as i32;
        let height = self.canvas.offset_height() as i32;

        if width != self.width || height != self.height {
            self.canvas.set_width(width as u32);
            self.canvas.set_height(height as u32);

            self.webgl.viewport(0, 0, width, height);

            const fov : f32 = 45.0_f32;
            const near : f32 = 0.0_f32;
            const far : f32 = 100.0_f32;

            self.projection_matrix = nalgebra_glm::perspective_fov_lh_zo(
                fov.to_radians(),
                width as f32,
                height as f32,
                near,
                far,
            );

            self.width = width;
            self.height = height;
        }
    }

    pub fn clear(&self) {
        self.webgl.clear_color(1.0, 0.0, 0.0, 1.0);
        self.webgl.clear(WebGL::COLOR_BUFFER_BIT | WebGL::DEPTH_BUFFER_BIT);
    }
}

/*
    WebGL Left-Handed Camera
*/
pub struct Camera {
    context: Rc<Context>,
    eye: nalgebra_glm::TVec3<f32>,
    center: nalgebra_glm::TVec3<f32>,
    up: nalgebra_glm::TVec3<f32>,
}

impl Camera {
    pub fn new(context: Rc<Context>) -> Self {
        let eye = nalgebra_glm::vec3(1.0, 0.0, 0.0);
        let center = nalgebra_glm::vec3(0.0, 0.0, 0.0);
        let up = nalgebra_glm::vec3(0.0, 1.0, 0.0);

        Self {
            context: context,
            eye: eye,
            center: center,
            up: up,
        }
    }

    fn CalculateProjectionViewMatrix(&self) -> nalgebra_glm::TMat4<f32> {
        let projection_matrix : &nalgebra_glm::TMat4<f32> = &self.context.projection_matrix;
        let view_matrix = nalgebra_glm::look_at(
            &self.eye,
            &self.center,
            &self.up,
        );
        projection_matrix * view_matrix
    }
}

/*
    WebGL Shader Program
*/
pub struct ShaderProgram {
    context: Rc<Context>,
    fs: WebGLShader,
    vs: WebGLShader,
    program: WebGLProgram,
}

impl ShaderProgram {
    pub fn new(context: Rc<Context>, vscode: &str, fscode: &str) -> Self {
        let webgl = context.webgl;

        let vs = webgl.create_shader(WebGL::VERTEX_SHADER).unwrap();
        webgl.shader_source(&vs, &vscode);
        webgl.compile_shader(&vs);

        let fs = webgl.create_shader(WebGL::FRAGMENT_SHADER).unwrap();
        webgl.shader_source(&fs, &fscode);
        webgl.compile_shader(&fs);

        let program = webgl.create_program().unwrap();
        webgl.attach_shader(&program, &vs);
        webgl.attach_shader(&program, &fs);
        webgl.link_program(&program);

        Self {
            context: context,
            fs: fs,
            vs: vs,
            program: program,
        }
    }

    fn GetUniform(&mut self, name: &str) -> WebGLUniformLocation {
        self.context.webgl.get_uniform_location(&self.program, name).unwrap()
    }
    fn GetAttrib(&mut self, name: &str) -> u32 {
        self.context.webgl.get_attrib_location(&self.program, name) as u32
    }
}

/*
    Geometry
    + Vertex locations
    + Index into vertices for triangles (clockwise, left-hand)
    + Normal vectors for each triangle
    + Colors for each triangle
*/
pub struct Geometry {
    context: Rc<Context>,
    program: ShaderProgram,
}


/*
    Cube Renderer
 */
pub struct Cube {
    context: Rc<Context>,
    program: ShaderProgram,
    mvp_matrix: WebGLUniformLocation,
    vertex_position: u32,
    vertex_color: u32,
    position_ebo: WebGLBuffer,
    position_vbo: WebGLBuffer,
    color_vbo: WebGLBuffer,
}

impl Cube {
    pub fn new(context: Rc<Context>) -> Self {
        let webgl = context.webgl;

        let vscode = include_str!("shaders/cube_vs.glsl");
        let fscode = include_str!("shaders/cube_fs.glsl");
        let program = ShaderProgram::new(context, vscode, fscode);

        let vertices = TypedArray::<f32>::from(&[
            -1.,-1.,-1.,  1.,-1.,-1.,  1., 1.,-1., -1., 1.,-1.,
            -1.,-1., 1.,  1.,-1., 1.,  1., 1., 1., -1., 1., 1.,
            -1.,-1.,-1., -1., 1.,-1., -1., 1., 1., -1.,-1., 1.,
             1.,-1.,-1.,  1., 1.,-1.,  1., 1., 1.,  1.,-1., 1.,
            -1.,-1.,-1., -1.,-1., 1.,  1.,-1., 1.,  1.,-1.,-1.,
            -1., 1.,-1., -1., 1., 1.,  1., 1., 1.,  1., 1.,-1., 
        ][..]).buffer();

        let indices = TypedArray::<u16>::from(&[
            0,1,2, 0,2,3, 4,5,6, 4,6,7,
            8,9,10, 8,10,11, 12,13,14, 12,14,15,
            16,17,18, 16,18,19, 20,21,22, 20,22,23 
        ][..]).buffer();

        let colors = TypedArray::<f32>::from(&[
            5.,3.,7., 5.,3.,7., 5.,3.,7., 5.,3.,7.,
            1.,1.,3., 1.,1.,3., 1.,1.,3., 1.,1.,3.,
            0.,0.,1., 0.,0.,1., 0.,0.,1., 0.,0.,1.,
            1.,0.,0., 1.,0.,0., 1.,0.,0., 1.,0.,0.,
            1.,1.,0., 1.,1.,0., 1.,1.,0., 1.,1.,0.,
            0.,1.,0., 0.,1.,0., 0.,1.,0., 0.,1.,0.
        ][..]).buffer();

        let position_ebo = webgl.create_buffer().unwrap();
        webgl.bind_buffer(WebGL::ELEMENT_ARRAY_BUFFER, position_ebo);

        let position_vbo = webgl.create_buffer().unwrap();
        webgl.bind_buffer(WebGL::ARRAY_BUFFER, position_vbo);

        let color_vbo = webgl.create_buffer().unwrap();
        webgl.bind_buffer(WebGL::ARRAY_BUFFER, color_vbo);

        Self {
            context: context,
            program: program,
            mvp_matrix: program.GetUniform("MVPMatrix"),
            vertex_position: program.GetAttrib("VertexPosition"),
            vertex_color: program.GetAttrib("VertexColor"),
            position_ebo: position_ebo,
            position_vbo: position_vbo,
            color_vbo: color_vbo,
        }
    }

    pub fn Draw() {
        let webgl = self.context.webgl;

        webgl.use_program(self.program.program);

        webgl.bind_buffer(WebGL::ELEMENT_ARRAY_BUFFER, &self.vertex_ebo);
        webgl.bind_buffer(WebGL::ARRAY_BUFFER, &self.vertex_vbo);
        webgl.enable_vertex_attrib_array(self.vertex_position);
        webgl.vertex_attrib_pointer(self.vertex_position, 3, WebGL::FLOAT, false, 0, 0) ;

        webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&color_buffer));
        let color = webgl.get_attrib_location(&shader_program, "color") as u32;
        webgl.vertex_attrib_pointer(color, 3, WebGL::FLOAT, false, 0, 0) ;

        // Color
        webgl.enable_vertex_attrib_array(color);
    }
}


/*
    Graphics Subsystem State
 */
pub struct GraphicsState {
    context: Rc<Context>,
    cube: Cube,
}

impl GraphicsState {
    pub fn new() -> Self {
        let context = Rc::new(Context::new("#canvas"));
        let cube = Cube::new(context);

        let state = Self {
            context: context,
            cube: cube,
        };

        state.initialize();

        return state;
    }

    fn initialize(&mut self) {
        let webgl = self.context.webgl;

        let mov_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];
        let mut view_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];

        // translating z
        view_matrix[14] -= 6.; //zoom
    }
}
