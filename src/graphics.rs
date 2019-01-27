//use stdweb::webapi::typed_array::TypedArray;
use stdweb::web;
use webgl_rendering_context::{
    WebGLRenderingContext as webgl,
    WebGLUniformLocation,
    WebGLBuffer
};
use stdweb::web::{
    IEventTarget,
    IHtmlElement,
    IParentNode,
    document,
    window,
    TypedArray,
};
use stdweb::web::html_element::CanvasElement;

/*
    WebGL Context
 */
pub struct Context {
    canvas: CanvasElement,
    context: webgl,
}

impl Context {
    pub fn new(element_id: String) -> Self {
        let element : web::Element = web::document().query_selector(&element_id).unwrap().unwrap();
        let canvas : CanvasElement = CanvasElement::from(element);
        let context : webgl = canvas.get_context("webgl").unwrap();

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        web::window().add_event_listener( move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        });

        context.clear_color(1.0, 0.0, 0.0, 1.0);
        context.clear(webgl::COLOR_BUFFER_BIT);

        Self {
            canvas: canvas,
            context: context,
        }
    }
}


/*
    WebGL Shader Program
 */
pub struct ShaderProgram {
    context: Context,
    fs: web::WebGLShader,
    vs: web::WebGLShader,
    program: web::WebGLShader,
}

impl ShaderProgram {
    pub fn new(context: Context, vscode: String, fscode: String) -> Self {
        let vs = context.glctx.create_shader(gl::VERTEX_SHADER).unwrap();
        context.glctx.shader_source(&vs, vscode);
        context.glctx.compile_shader(&vs);

        let fs = context.glctx.create_shader(gl::FRAGMENT_SHADER).unwrap();
        context.glctx.shader_source(&fs, fscode);
        context.glctx.compile_shader(&fs);

        let program = context.glctx.create_program().unwrap();
        context.glctx.attach_shader(&program, &vs);
        context.glctx.attach_shader(&program, &fs);
        context.glctx.link_program(&program);

        Self {
            context: context,
            fs: fs,
            vs: vs,
            program: program,
        }
    }

    fn get_p_matrix() {
        context.get_uniform_location(&shader_program, "Pmatrix").unwrap()
    }
}


/*
    Cube Renderer
 */
pub struct Cube {
    context: Context,
    shader: ShaderProgram,
}

impl Cube {
    pub fn new(context: Context) -> Self {
        let vs = include_str!("shaders/cube_vs.glsl");
        let fs = include_str!("shaders/cube_fs.glsl");
        let shader = ShaderProgram::new(context, vs, fs);

        let vertices = TypedArray::<f32>::from(&[
            -1.,-1.,-1.,  1.,-1.,-1.,  1., 1.,-1., -1., 1.,-1.,
            -1.,-1., 1.,  1.,-1., 1.,  1., 1., 1., -1., 1., 1.,
            -1.,-1.,-1., -1., 1.,-1., -1., 1., 1., -1.,-1., 1.,
             1.,-1.,-1.,  1., 1.,-1.,  1., 1., 1.,  1.,-1., 1.,
            -1.,-1.,-1., -1.,-1., 1.,  1.,-1., 1.,  1.,-1.,-1.,
            -1., 1.,-1., -1., 1., 1.,  1., 1., 1.,  1., 1.,-1., 
        ][..]).buffer();

        let colors = TypedArray::<f32>::from(&[
            5.,3.,7., 5.,3.,7., 5.,3.,7., 5.,3.,7.,
            1.,1.,3., 1.,1.,3., 1.,1.,3., 1.,1.,3.,
            0.,0.,1., 0.,0.,1., 0.,0.,1., 0.,0.,1.,
            1.,0.,0., 1.,0.,0., 1.,0.,0., 1.,0.,0.,
            1.,1.,0., 1.,1.,0., 1.,1.,0., 1.,1.,0.,
            0.,1.,0., 0.,1.,0., 0.,1.,0., 0.,1.,0.
        ][..]).buffer();

        let indices = TypedArray::<u16>::from(&[
            0,1,2, 0,2,3, 4,5,6, 4,6,7,
            8,9,10, 8,10,11, 12,13,14, 12,14,15,
            16,17,18, 16,18,19, 20,21,22, 20,22,23 
        ][..]).buffer();

        Self {
            context: context,
            shader: shader,
        }
    }
}


/*
    Graphics Subsystem State
 */
pub struct GraphicsState {
    context: Context,
    cube: Cube,
}

impl GraphicsState {
    pub fn new() -> Self {
        let context = Context::new("#canvas");
        let cube = Cube::new(context);

        let state = Self {
            context: context,
            cube: cube,
        };

        state.initialize();

        return state;
    }

    fn initialize(&mut self) {
        /* ====== Associating attributes to vertex shader =====*/



        context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
        let position = context.get_attrib_location(&shader_program, "position") as u32;
        context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0) ;

        // Position
        context.enable_vertex_attrib_array(position);
        context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
        let color = context.get_attrib_location(&shader_program, "color") as u32;
        context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0) ;

        // Color
        context.enable_vertex_attrib_array(color);
        context.use_program(Some(&shader_program));

        let mov_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];
        let mut view_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];

        // translating z
        view_matrix[14] -= 6.; //zoom

        let state = Rc::new(RefCell::new(State {
            time_old: 0.0,
            mov_matrix,
            view_matrix,
            canvas,
            context,
            p_matrix,
            v_matrix,
            m_matrix,
            index_buffer,
        }));

        state.borrow_mut().animate(0., state.clone());
    }
}
