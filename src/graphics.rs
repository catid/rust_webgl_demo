use stdweb::web;

pub struct GraphicsState {
    // Nothing here yet.
    canvas: CanvasElement,
    context: gl,
}

impl GraphicsState {
    fn new() -> GraphicsState {
        let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
        let context: gl = canvas.get_context().unwrap();

        let state = GraphicsState {
            canvas: canvas,
            context: context,
        };

        state.initialize();

        return state;
    }

    fn initialize(&mut self) {
        let canvas: &CanvasElement = self.canvas;
        let context: &gl = self.context;

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        context.clear_color(1.0, 0.0, 0.0, 1.0);
        context.clear(gl::COLOR_BUFFER_BIT);

        window().add_event_listener( move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        });

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

        // Create and store data into vertex buffer
        let vertex_buffer = context.create_buffer().unwrap();
        context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
        context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

        // Create and store data into color buffer
        let color_buffer = context.create_buffer().unwrap();
        context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
        context.buffer_data_1(gl::ARRAY_BUFFER, Some(&colors), gl::STATIC_DRAW);

        // Create and store data into index buffer
        let index_buffer = context.create_buffer().unwrap();
        context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

        /*=================== Shaders =========================*/
        let vert_code = r#"
            attribute vec3 position;
            uniform mat4 Pmatrix;
            uniform mat4 Vmatrix;
            uniform mat4 Mmatrix;
            attribute vec3 color;
            varying vec3 vColor;
            void main() {
                gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
                vColor = color;
            }
        "#;

        let frag_code = r#"
            precision mediump float;
            varying vec3 vColor;
            void main() {
                gl_FragColor = vec4(vColor, 1.);
            }
        "#;

        let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
        context.shader_source(&vert_shader, vert_code);
        context.compile_shader(&vert_shader);

        let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
        context.shader_source(&frag_shader, frag_code);
        context.compile_shader(&frag_shader);

        let shader_program = context.create_program().unwrap();
        context.attach_shader(&shader_program, &vert_shader);
        context.attach_shader(&shader_program, &frag_shader);
        context.link_program(&shader_program);

        /* ====== Associating attributes to vertex shader =====*/
        let p_matrix = context.get_uniform_location(&shader_program, "Pmatrix").unwrap();
        let v_matrix = context.get_uniform_location(&shader_program, "Vmatrix").unwrap();
        let m_matrix = context.get_uniform_location(&shader_program, "Mmatrix").unwrap();

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
