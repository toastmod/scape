use glium::{Display, Program};

pub fn makeshader(display: Display) -> (Program, Display) {
    (
        program!(&display,
            140 => {
                vertex: "
                #version 140
                uniform mat4 matrix;
                in vec3 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = matrix * vec4(position, 1.0);
                    vColor = color;
                }
            ",

                fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
            },

            110 => {
                vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec3 position;
                attribute vec3 color;
                varying vec3 vColor;
                void main() {
                    gl_Position = matrix * vec4(position, 1.0);
                    vColor = color;
                }
            ",

                fragment: "
                #version 110
                varying vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
            },

            100 => {
                vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec3 position;
                attribute lowp vec3 color;
                varying lowp vec3 vColor;
                void main() {
                    gl_Position = matrix * vec4(position, 1.0);
                    vColor = color;
                }
            ",

                fragment: "
                #version 100
                varying lowp vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
            },
        )
        .unwrap(),
        display,
    )
}
