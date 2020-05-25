//use ultraviolet::*;
//pub fn ultraviolet_get_4x4(inmat: Mat4) -> [[f32; 4];4] {
//    let mut outer :  [[f32; 4];4]  = [[0f32,0f32,0f32,0f32],[0f32,0f32,0f32,0f32],[0f32,0f32,0f32,0f32],[0f32,0f32,0f32,0f32]];
//    let mut y = 0;
//    for v in inmat.cols.iter() {
//        outer[y][0] = v[0];
//        outer[y][1] = v[1];
//        outer[y][2] = v[2]+100f32;
//        outer[y][3] = v[3]+0f32;
//        println!("{:?}", outer[y]);
//        y +=1;
//    }
//    outer
//}
extern crate rand;
use glium::VertexBuffer;
use rand::*;
use std::borrow::Cow;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Error;
use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Copy, Clone)]
pub struct Vertexd {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vertexd, position, color);

pub fn deg2rad(inp: f32) -> f32 {
    ((inp * PI) / 180f32)
}

pub fn build_stars(display: glium::Display) -> (glium::Display, VertexBuffer<Vertexd>) {
    let mut rng = rand::thread_rng();
    let mut blank: Vertexd = Vertexd {
        position: [0.0, 0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    };

    let mut stars: [Vertexd; 1000] = [blank; 1000];

    for s in stars.iter_mut() {
        s.position = [
            rng.gen_range(-1.0f32, 1.0f32),
            rng.gen_range(-1.0f32, 1.0f32),
            rng.gen_range(-1.0f32, 1.0f32),
        ];
    }

    let vb = {
        glium::VertexBuffer::new(
            &display,
            &[
                Vertexd {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertexd {
                    position: [0.0, 0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                Vertexd {
                    position: [0.5, -0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap()
    };

    (display, vb)
}

pub fn build_tri(display: glium::Display) -> (glium::Display, VertexBuffer<Vertexd>) {
    let vb = {
        glium::VertexBuffer::new(
            &display,
            &[
                Vertexd {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertexd {
                    position: [0.0, 0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                Vertexd {
                    position: [0.5, -0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap()
    };

    (display, vb)
}

pub fn build_axis(display: glium::Display) -> (glium::Display, VertexBuffer<Vertexd>) {
    let vb = {
        glium::VertexBuffer::new(
            &display,
            &[
                Vertexd {
                    position: [0.0, 0.0, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertexd {
                    position: [100.0, 0.0, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertexd {
                    position: [0.0, 0.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertexd {
                    position: [0.0, 100.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertexd {
                    position: [0.0, 0.0, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                Vertexd {
                    position: [0.0, 0.0, 100.0],
                    color: [0.0, 0.0, 1.0],
                },
            ],
        )
        .unwrap()
    };

    (display, vb)
}
