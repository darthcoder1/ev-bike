#![deny(unsafe_code)]

extern crate videocore;

extern crate ebola;

use ebola::texture;

use videocore::bcm_host;

fn main() {
    bcm_host::init();

    let mut window = ebola::CreateRenderWindow();

    let glContext = ebola::InitEGL(&mut window);

    ebola::texture::LoadTexture("blka");
    ebola::TickEGL(glContext);
}