use crate::utils::*;
use glium::glutin;
use winit;

pub struct Controller {
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
    pub scrollbuf: i8,
    pub lclick: bool,
    pub rclick: bool,
    pub mclick: bool,
    pub mousex: f64,
    pub mousey: f64,
    pub sensitivity: (f64,f64),
}

impl Controller {
    pub fn new(plyr: i8) -> Controller {
        Controller {
            up: false,
            left: false,
            down: false,
            right: false,
            forward: false,
            backward: false,
            scrollbuf: 0,
            lclick: false,
            rclick: false,
            mclick: false,
            mousex: 0f64,
            mousey: 0f64,
            sensitivity: (6.0f64,6.0f64),
        }
    }

    pub fn keyboard_input(&mut self, key: glutin::event::VirtualKeyCode, pressed: bool) {
        //TODO: manage keybinds
        match key {
            glutin::event::VirtualKeyCode::Up => self.up = pressed,
            glutin::event::VirtualKeyCode::Down => self.down = pressed,
            glutin::event::VirtualKeyCode::A => self.left = pressed,
            glutin::event::VirtualKeyCode::D => self.right = pressed,
            glutin::event::VirtualKeyCode::W => self.forward = pressed,
            glutin::event::VirtualKeyCode::S => self.backward = pressed,

            _ => (),
        };
    }

    pub fn mouse_input(&mut self, btn: winit::event::MouseButton, pressed: bool) {
        match btn {
            winit::event::MouseButton::Left => {
                self.lclick = pressed;
            }
            winit::event::MouseButton::Middle => {
                self.mclick = pressed;
            }
            winit::event::MouseButton::Right => {
                self.rclick = pressed;
            }
            _ => {}
        }
    }

    pub fn mouse_move(&mut self, delta: (f64, f64)) {
        self.mousex += delta.0*self.sensitivity.0;
        self.mousey += delta.1*self.sensitivity.1;
    }

    pub fn set_cursor(&mut self, pos: (f64, f64)) {
        self.mousex = pos.0;
        self.mousey = pos.1;
    }

    pub fn set_sens(&mut self, horz: f64, vert: f64) {
        self.mousex = horz;
        self.mousey = vert;
    }

}
