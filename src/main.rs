use player::Player;
use rapier2d::prelude::PhysicsWorld;
use raylib::prelude::*;

mod player;

const PLAYER_PNG: &[u8] = include_bytes!("chick.png");

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(800, 800)
    .title("My Game!")
    .vsync()
    .msaa_4x()
    .build();
    let mut image = Image::load_image_from_mem(".png", PLAYER_PNG).unwrap();
    image.resize(50, 50);
    let texture = rl.load_texture_from_image(&thread, &image).unwrap();
    let mut player = Player::new(Vector2{x:10.0, y: 400.0}, texture);
    while !rl.window_should_close() {
        player.tick(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::PINK);
        player.draw(&mut d);

    }
}
