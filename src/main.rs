#[macro_use]
extern crate glium;
#[macro_use]

use crate::camera::Camera;
use crate::controller::Controller;
use crate::models::{build_axis, build_tri, deg2rad};
use glam;
use glium::index::PrimitiveType;
use glium::{glutin, Surface};
use glium::{Program, VertexBuffer};
use nalgebra;
use nalgebra::Perspective3;
use rand::prelude::*;
use std::f32::consts::PI;
use std::ops::Mul;
use std::time::Instant;
use std::{process, thread};

use glium::backend::Facade;
use std::borrow::{Borrow, BorrowMut};

mod camera;
mod controller;
mod models;
mod shaders;
mod utils;

fn main() {
    //init values
    let mut window_size = glutin::dpi::Size::new(glutin::dpi::PhysicalSize::new(800, 600));
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new()
        .with_title("test")
        .with_inner_size(window_size);
    let cb = glutin::ContextBuilder::new();
    let mut display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let WIDTH: f32 = window_size.to_logical(1f64).width;
    let HEIGHT: f32 = window_size.to_logical(1f64).height;
    println!("size: {},{}", WIDTH, HEIGHT);

    //BUILD CAMERA
    let mut camera = Camera::new(WIDTH, HEIGHT);

    //BUILDING BUFFERS FOR AXIS
    let vbd_axis = build_axis(display);
    let vertex_buffer_axis = vbd_axis.1;
    display = vbd_axis.0;
    let index_buffer_axis = glium::index::NoIndices(PrimitiveType::LinesList);

    //BUILDING BUFFERS FOR TRIANGLE
    let vbd_tri = build_tri(display);
    let vertex_buffer_tri = vbd_tri.1;
    display = vbd_tri.0;
    let index_buffer_tri = glium::index::NoIndices(PrimitiveType::TriangleStrip);

    // compiling shaders and linking them together
    let displayandshader = shaders::makeshader(display);
    display = displayandshader.1;
    let program = displayandshader.0;

    //INITIALIZING MATRICIES
    let mut model = glam::Mat4::identity();
    let mut view = glam::Mat4::identity();
    let mut projection = glam::Mat4::identity();
    let mut PVM = projection * view * model;

    //initialize controller
    let mut ctrl = Controller::new(0);

    //movement speed
    let move_speed = 20.0; // in units/sec

    let mut frame_delta = 0f32;

    //==================================================================================
    // the main loop
    event_loop.run(move |event, _, control_flow| {
        //set delta timer
        let now = Instant::now();
        model = glam::Mat4::from_translation(glam::Vec3::new(
            camera.position.0,
            camera.position.1,
            camera.position.2,
        ));

        view = camera.lookat_upd8();
        projection = glam::Mat4::perspective_infinite_lh(60f32.to_radians(), WIDTH / HEIGHT, 0.1);
        PVM = projection * view * model;

        // building the uniforms
        let uniforms = uniform! {
        matrix: PVM.to_cols_array_2d()
        };

        //update key values
        frame_delta = now.elapsed().as_nanos() as f32;
        camera.update_vals((frame_delta / 100000000.0) * move_speed, &ctrl);

        // drawing a frame
        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target
            .draw(
                &vertex_buffer_axis,
                &index_buffer_axis,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        //draw axis ?and other stuff?
        target
            .draw(
                &vertex_buffer_tri,
                &index_buffer_tri,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();

        //===============================================================
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    //if keyevent pass to the controller targeting the keyboard
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        let pressed = input.state == glutin::event::ElementState::Pressed;
                        let key = match input.virtual_keycode {
                            Some(key) => key,
                            None => return,
                        };
                        ctrl.keyboard_input(key, pressed);

                        glutin::event_loop::ControlFlow::Poll
                    }

                    glutin::event::WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        modifiers,
                    } => {
                        let pressed = state == glutin::event::ElementState::Pressed;
                        ctrl.mouse_input(button, pressed);

                        glutin::event_loop::ControlFlow::Poll
                    }

                    // Break from the main loop when the window is closed.
                    glutin::event::WindowEvent::CloseRequested => {
                        glutin::event_loop::ControlFlow::Exit
                    }
                    // Redraw the triangle when the window is resized.
                    glutin::event::WindowEvent::Resized(..) => {
                        //draw();
                        glutin::event_loop::ControlFlow::Poll
                    }
                    _ => glutin::event_loop::ControlFlow::Poll,
                }
            }

            glutin::event::Event::DeviceEvent { device_id, event } => {
                //add while focused on window
                match event {
                    glutin::event::DeviceEvent::MouseMotion { delta } => {
                        ctrl.mouse_move(delta);
                        glutin::event_loop::ControlFlow::Poll
                    }
                    _ => glutin::event_loop::ControlFlow::Poll,
                }
            }

            _ => glutin::event_loop::ControlFlow::Poll,
        };

        //===============================================================
    });

    //end of main
}
