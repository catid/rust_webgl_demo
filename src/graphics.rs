//use stdweb::traits::*;
use stdweb::unstable::TryInto;
use webgl_rendering_context::{
    WebGLRenderingContext as WebGL,
    WebGLUniformLocation,
    WebGLBuffer,
    WebGLShader,
    WebGLProgram,
};
use stdweb::web::{
    IHtmlElement,
    IParentNode,
    document,
    TypedArray,
};
use stdweb::web::html_element::CanvasElement;
use glm::{Vec2, Vec3, Quat, Mat4};
use tools::js_log;

/*
    WebGL Context with Right-Handed Projection Matrix
*/
pub struct Context {
    canvas: CanvasElement,
    webgl: WebGL,
    width: i32,
    height: i32,
    projection_matrix: Mat4,
}

impl Context {
    pub fn new(element_id: &str) -> Self {
        let canvas : CanvasElement = document().query_selector(&element_id).unwrap().unwrap().try_into().unwrap();
        let webgl : WebGL = canvas.get_context().unwrap();

        // Right-hand rule for rendering only front faces
        webgl.enable(WebGL::CULL_FACE);
        webgl.front_face(WebGL::CCW);
        webgl.cull_face(WebGL::BACK);

        webgl.enable(WebGL::DEPTH_TEST);
        webgl.depth_func(WebGL::GREATER);

        Self {
            canvas: canvas,
            webgl: webgl,
            width: 0,
            height: 0,
            projection_matrix: Mat4::identity(),
        }
    }

    // Following the guide here:
    // https://webglfundamentals.org/webgl/lessons/webgl-anti-patterns.html
    // We should not react to the resize event to update the canvas size.
    pub fn UpdateViewport(&mut self) {
        let width = self.canvas.offset_width();
        let height = self.canvas.offset_height();

        if width != self.width || height != self.height {
            self.canvas.set_width(width as u32);
            self.canvas.set_height(height as u32);

            self.webgl.viewport(0, 0, width, height);

            const fov : f32 = 60.;
            const near : f32 = 2.;
            const far : f32 = 20.;

            self.projection_matrix = glm::perspective_fov_rh_zo(
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

    pub fn Clear(&self) {
        self.webgl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.webgl.clear_depth(0.0);
        self.webgl.clear(WebGL::COLOR_BUFFER_BIT | WebGL::DEPTH_BUFFER_BIT);
    }
}

/*
    WebGL Shader Program
*/
pub struct ShaderProgram {
    fs: WebGLShader,
    vs: WebGLShader,
    webGlProgram: WebGLProgram,
}

impl ShaderProgram {
    pub fn new(context: &Context, vsCode: &str, fsCode: &str) -> Self {
        let webgl = &context.webgl;

        let vs = webgl.create_shader(WebGL::VERTEX_SHADER).unwrap();
        webgl.shader_source(&vs, &vsCode);
        webgl.compile_shader(&vs);

        let vs_success : bool = webgl.get_shader_parameter(&vs, WebGL::COMPILE_STATUS).try_into().unwrap();

        if !vs_success {
            let info = webgl.get_shader_info_log(&vs);
            js_log(format!("CompileShader failed: {}", info.unwrap()));
        }

        let fs = webgl.create_shader(WebGL::FRAGMENT_SHADER).unwrap();
        webgl.shader_source(&fs, &fsCode);
        webgl.compile_shader(&fs);

        let fs_success : bool = webgl.get_shader_parameter(&fs, WebGL::COMPILE_STATUS).try_into().unwrap();

        if !fs_success {
            let info = webgl.get_shader_info_log(&fs);
            js_log(format!("CompileShader failed: {}", info.unwrap()));
        }

        let program = webgl.create_program().unwrap();
        webgl.attach_shader(&program, &vs);
        webgl.attach_shader(&program, &fs);
        webgl.link_program(&program);

        let success : bool = webgl.get_program_parameter(&program, WebGL::LINK_STATUS).try_into().unwrap();

        if !success {
            let info = webgl.get_program_info_log(&program);
            js_log(format!("LinkProgram failed: {}", info.unwrap()));
        }

        Self {
            fs: fs,
            vs: vs,
            webGlProgram: program,
        }
    }

    fn GetUniform(&self, context: &Context, name: &str) -> WebGLUniformLocation {
        context.webgl.get_uniform_location(&self.webGlProgram, name).unwrap()
    }
    fn GetAttrib(&self, context: &Context, name: &str) -> u32 {
        context.webgl.get_attrib_location(&self.webGlProgram, name) as u32
    }
}

/*
    Cube Renderer
 */
pub struct Cube {
    program: ShaderProgram,
    unifMvpMatrix: WebGLUniformLocation,
    attrVertexPosition: u32,
    attrVertexColor: u32,
    attrVertexNormal: u32,
    positionVbo: WebGLBuffer,
    colorVbo: WebGLBuffer,
    normalVbo: WebGLBuffer,
    tri_count: i32,
}

impl Cube {
    pub fn new(context: &Context) -> Self {
        let webgl = &context.webgl;

        let vsCode = include_str!("shaders/flat_vs.glsl");
        let fsCode = include_str!("shaders/flat_fs.glsl");
        let program = ShaderProgram::new(context, vsCode, fsCode);

        /*
            Corner vertices of a cube, oriented x+right, y+top, z+up,
            centered at 0,0,0, scaled to span from -1 to +1 on each axis.
            Vertex and side names are based on a perspective looking down.
        */
        let corners = vec![
            /* Down-z side of cube */
            -1.,-1.,-1., /* LL */
             1.,-1.,-1., /* LR */ 
             1., 1.,-1., /* UR */ 
            -1., 1.,-1., /* UL */ 
            /* Up+z side of cube */
            -1.,-1., 1., /* LL */
             1.,-1., 1., /* LR */ 
             1., 1., 1., /* UR */ 
            -1., 1., 1., /* UL */ 
        ];

        // This follows a right-hand winding order, where the right-hand rule
        // dictates the direction of the normals of each triangle, facing
        // out of the cube.  Side names are based on a perspective looking down.
        let triIndices : Vec<u8> = vec![
            /* Down-z */
            2, 1, 0,  0, 3, 2,
            /* Up+z */
            4, 5, 6,  6, 7, 4,
            /* Bottom-y */
            0, 5, 4,  0, 1, 5,
            /* Top+y */
            3, 7, 6,  3, 6, 2,
            /* Left-x */
            0, 4, 3,  4, 7, 3,
            /* Right+x */
            2, 6, 5,  2, 5, 1,
        ];

        let triColors : Vec<u8> = vec![
            /* Down-z */
            255,0,200, 255,0,255,
            /* Up+z */
            200,200,200, 200,200,255,
            /* Bottom-y */
            100,200,100, 100,255,100,
            /* Top+y */
            200,200,100, 200,255,100,
            /* Left-x */
            200,0,0, 255,0,0,
            /* Right+x */
            0,200,0, 0,255,0,
        ];

        let tri_count = triIndices.len() / 3;

        let mut vertices = Vec::with_capacity(tri_count * 3);
        let mut colors = Vec::with_capacity(tri_count * 3);
        let mut normals = Vec::with_capacity(tri_count * 3);

        for i in 0..tri_count {
            let triIndicesOffset = i * 3;
            let mut triVertices : [Vec3; 3] = unsafe { std::mem::uninitialized() };

            for j in 0..3 {
                let vertexIndex = triIndices[triIndicesOffset + j];
                let cornersOffset = vertexIndex as usize * 3;
                let scale = 1.0f32;
                let x = corners[cornersOffset] * scale;
                let y = corners[cornersOffset + 1] * scale;
                let z = corners[cornersOffset + 2] * scale;
                triVertices[j] = glm::vec3(x, y, z);
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
            }

            let normal = glm::triangle_normal(
                &triVertices[0],
                &triVertices[1],
                &triVertices[2]
            );

            //js_log(format!("triangle_normal for i = {}: {}", i, normal));

            for _j in 0..3 {
                normals.push(normal.x);
                normals.push(normal.y);
                normals.push(normal.z);
            }

            let colorOffset = i as usize * 3;
            let r = triColors[colorOffset];
            let g = triColors[colorOffset + 1];
            let b = triColors[colorOffset + 2];
            for _j in 0..3 {
                colors.push(r);
                colors.push(g);
                colors.push(b);
            }
        }

        let webVertices = TypedArray::<f32>::from(vertices.as_slice()).buffer();
        let webColors = TypedArray::<u8>::from(colors.as_slice()).buffer();
        let webNormals = TypedArray::<f32>::from(normals.as_slice()).buffer();

        //js_log(format!("Generated {} triangles", tri_count));

        let positionVbo = webgl.create_buffer().unwrap();
        webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&positionVbo));
        webgl.buffer_data_1(WebGL::ARRAY_BUFFER, Some(&webVertices), WebGL::STATIC_DRAW);

        let colorVbo = webgl.create_buffer().unwrap();
        webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&colorVbo));
        webgl.buffer_data_1(WebGL::ARRAY_BUFFER, Some(&webColors), WebGL::STATIC_DRAW);

        let normalVbo = webgl.create_buffer().unwrap();
        webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&normalVbo));
        webgl.buffer_data_1(WebGL::ARRAY_BUFFER, Some(&webNormals), WebGL::STATIC_DRAW);

        let unifMvpMatrix = program.GetUniform(&context, "MVPMatrix");
        let attrVertexPosition = program.GetAttrib(&context, "VertexPosition");
        let attrVertexColor = program.GetAttrib(&context, "VertexColor");
        let attrVertexNormal = program.GetAttrib(&context, "VertexNormal");

        Self {
            program: program,
            unifMvpMatrix: unifMvpMatrix,
            attrVertexPosition: attrVertexPosition,
            attrVertexColor: attrVertexColor,
            attrVertexNormal: attrVertexNormal,
            positionVbo: positionVbo,
            colorVbo: colorVbo,
            normalVbo: normalVbo,
            tri_count: tri_count as i32,
        }
    }

    pub fn DrawMultiple(&mut self, context: &Context, mvp_matrices: &Vec<Mat4>) {
        let webgl = &context.webgl;

        webgl.use_program(Some(&self.program.webGlProgram));

        if self.attrVertexPosition != u32::max_value() {
            webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&self.positionVbo));
            webgl.vertex_attrib_pointer(self.attrVertexPosition, 3, WebGL::FLOAT, false, 0, 0) ;
            webgl.enable_vertex_attrib_array(self.attrVertexPosition);
        }

        if self.attrVertexColor != u32::max_value() {
            webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&self.colorVbo));
            webgl.vertex_attrib_pointer(self.attrVertexColor, 3, WebGL::UNSIGNED_BYTE, true, 0, 0) ;
            webgl.enable_vertex_attrib_array(self.attrVertexColor);
        }

        if self.attrVertexNormal != u32::max_value() {
            webgl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&self.normalVbo));
            webgl.vertex_attrib_pointer(self.attrVertexNormal, 3, WebGL::FLOAT, false, 0, 0) ;
            webgl.enable_vertex_attrib_array(self.attrVertexNormal);
        }

        for mat in mvp_matrices {
            webgl.uniform_matrix4fv(Some(&self.unifMvpMatrix), false, mat.as_slice());

            webgl.draw_arrays(WebGL::TRIANGLES, 0, self.tri_count * 3);
        }
    }

    pub fn Draw(&mut self, context: &Context, mvp_matrix: Mat4) {
        let matrices = vec![
            mvp_matrix
        ];
        self.DrawMultiple(context, &matrices);
    }
}

/*
    Graphics Subsystem State
 */
pub struct GraphicsState {
    context: Context,
    cube: Cube,
    positions: Vec<Vec2>,
}

impl GraphicsState {
    pub fn new() -> Self {
        let context = Context::new("#canvas");
        let cube = Cube::new(&context);

        let positions = vec![
            glm::vec2(-2.0f32, 0.0f32),
            glm::vec2(0.0f32, 0.0f32),
            glm::vec2(2.0f32, 0.0f32),
            glm::vec2(0.0f32, -2.0f32),
            glm::vec2(0.0f32, 2.0f32),
        ];

        Self {
            context: context,
            cube: cube,
            positions: positions,
        }
    }

    pub fn RenderScene(&mut self, nowSeconds: f64) {
        self.context.UpdateViewport();
        self.context.Clear();

        // Look down from above at the cube
        let eye = glm::vec3(0.0, 0.0, -10.0);
        let center = glm::vec3(0.0, 0.0, 0.0);
        let up = glm::vec3(0.0, 1.0, 0.0);

        let view_matrix = glm::look_at_rh(
            &eye,
            &center,
            &up,
        );
        let proj_view_matrix = self.context.projection_matrix * view_matrix;

        let angle = glm::modf(nowSeconds / 1000.0f64, glm::two_pi()) as f32;

        let mut mvp_matrices = vec![];

        for position in &self.positions {
            // This will right-multiply the provided matrix by the scale matrix
            let scale_matrix = glm::scale(
                &glm::identity(),
                &glm::vec3(1.0, 1.0, 1.0)
            );
/*
            let quat_angle = &nalgebra_glm::quat_angle_axis(
                angle,
                &glm::vec3(1.0, 1.0, 1.0)
            );

            let rotate_matrix = glm::quat_to_mat4(quat_angle) * scale_matrix;
*/
            // This generates a translation matrix and right-multiplies it by the provided matrix
            let translate_matrix = glm::translate(&scale_matrix,
                &glm::vec3(position.x, position.y, 0.0f32)
            );

            let mvp = proj_view_matrix * translate_matrix;

            mvp_matrices.push(mvp);
        }

        self.cube.DrawMultiple(&self.context, &mvp_matrices);
    }
}
