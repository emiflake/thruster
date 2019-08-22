pub fn get_program(display: &glium::Display) -> glium::Program {
	program!(display,
		140 => {
			vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

			fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
		},

		110 => {
			vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec2 tex_coords;
                varying vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

			fragment: "
                #version 110
                uniform sampler2D tex;
                varying vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
		},

		100 => {
			vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec2 tex_coords;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

			fragment: "
                #version 100
                uniform lowp sampler2D tex;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
		},
	)
	.unwrap()
}

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub fn get_buffers(
	display: &glium::Display,
) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
	let vertex_buffer = {
		glium::VertexBuffer::new(
			display,
			&[
				Vertex {
					position: [-1.0, -1.0],
					tex_coords: [0.0, 0.0],
				},
				Vertex {
					position: [-1.0, 1.0],
					tex_coords: [0.0, 1.0],
				},
				Vertex {
					position: [1.0, 1.0],
					tex_coords: [1.0, 1.0],
				},
				Vertex {
					position: [1.0, -1.0],
					tex_coords: [1.0, 0.0],
				},
			],
		)
		.unwrap()
	};
	let index_buffer = glium::IndexBuffer::new(
		display,
		glium::index::PrimitiveType::TriangleStrip,
		&[1 as u16, 2, 0, 3],
	)
	.unwrap();

	(vertex_buffer, index_buffer)
}
