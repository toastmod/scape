use crate::controller::Controller;
use crate::utils::*;
use glam::Mat4;
use glium::glutin;
use std::f32::consts::PI;

pub struct Camera {
    pub aspect_ratio: f32,
    pub position: (f32, f32, f32),
    pub direction: (f32, f32, f32),
    pub rotation: (f32, f32),
    pub WIDTH: f32,
    pub HEIGHT: f32,
}

fn b2f(b: bool) -> f32 {
    (b as i8) as f32
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera {
        Camera {
            aspect_ratio: width / height,
            WIDTH: width,
            HEIGHT: height,
            position: (0.0, 0.0, 1.0),
            direction: (0.0, 0.0, 1.0),
            rotation: (0.0, 0.0),
        }
    }

    pub fn set_pos(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_dir(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }
    pub fn set_rot(&mut self, rot: (f32, f32)) {
        self.rotation = rot;
    }

    pub fn get_pos(&mut self) -> (f32, f32, f32) {
        self.position
    }

    pub fn get_dir(&mut self) -> (f32, f32, f32) {
        self.direction
    }

    pub fn get_rot(&mut self) -> (f32, f32) {
        self.rotation
    }

    pub fn update_vals(&mut self, mmod: f32, ctrl: &Controller) {

        //rotation angle
        self.rotation.0 = -map(ctrl.mousex as f32, (0f32, self.WIDTH), (0f32, 360f32)).to_radians();
        //elevation angle
        self.rotation.1 = -clamp(map(ctrl.mousey as f32, (0f32, self.HEIGHT), (0f32, 360f32)), -90f32, 90f32).to_radians();

        self.direction.0 = -self.rotation.0.cos();
        self.direction.2 = -self.rotation.0.sin();

        self.position.0 += ((-self.rotation.0.cos()*(b2f(ctrl.forward)-b2f(ctrl.backward)))+((self.rotation.0+(90f32.to_radians())).cos())*(b2f(ctrl.right)-b2f(ctrl.left)))*mmod;
        self.position.1 += (b2f(ctrl.down) - b2f(ctrl.up)) * mmod;
        self.position.2 += ((-self.rotation.0.sin()*(b2f(ctrl.forward)-b2f(ctrl.backward)))+((self.rotation.0+(90f32.to_radians())).sin())*(b2f(ctrl.right)-b2f(ctrl.left)))*mmod;
    }

    pub fn lookat_upd8(&mut self) -> Mat4 {

        // glam::Mat4::look_at_lh(
        //     glam::Vec3::new(0f32, 0f32, 0f32),
        //     glam::Vec3::new(
        //
        //         map(self.rotation.0.to_radians(),
        //             (0f32.to_radians(),360f32.to_radians()),
        //             (0f32,1f32)
        //         ),
        //
        //         self.rotation.1.to_radians().sin(),
        //         -self.rotation.1.to_radians().cos()
        //     ),
        //
        //     glam::Vec3::new(
        //         0f32,
        //         (self.rotation.1+90f32).to_radians().sin(),
        //         (self.rotation.1+90f32).to_radians().cos()
        //     ),
        // )

        glam::Mat4::look_at_lh(
            glam::Vec3::new(0f32,0f32,0f32),
            glam::Vec3::new(
                self.rotation.0.cos()*self.rotation.1.cos(),
                self.rotation.1.sin(),
                self.rotation.0.sin()*self.rotation.1.cos()
            ),

            glam::Vec3::new(
                0f32,
                1f32,
                0f32
            ),
        )
    }
}
