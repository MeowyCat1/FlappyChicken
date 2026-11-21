use rapier2d::prelude::PhysicsWorld;
use raylib::RaylibHandle;
use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw;
use raylib::ffi::Vector2;
use raylib::prelude::Texture2D;
pub struct Player {
    position : Vector2,
    image : Texture2D,
    jumping : bool,
    starty : f32,
    world : PhysicsWorld
}

impl Player {
    pub(crate) fn new(position : Vector2, image : Texture2D) -> Self {
        Self { position, image, jumping: false, starty: 0.0, world : PhysicsWorld::new() }
    }

    pub(crate) fn tick(&mut self, handle : &mut RaylibHandle) {
        self.position.x += handle.get_frame_time() * 50.0;
        if handle.is_key_pressed(KeyboardKey::KEY_SPACE) && !self.jumping {
            self.jumping = true;
            self.starty = self.position.y;
        }
        if self.jumping {
            println!("Changing height!");
            println!("{}", handle.get_frame_time());
            self.position.y -= handle.get_frame_time() * 200.0;
            if self.position.y <= self.starty - 100.0 {
                self.jumping = false
            }
        }
        else {
            self.position.y += handle.get_frame_time() * 100.0;
        }
    }
    pub(crate) fn draw(&self, draw: &mut impl RaylibDraw) {
        draw.draw_texture_v(&self.image, self.position, Color::WHITE);
    }
}