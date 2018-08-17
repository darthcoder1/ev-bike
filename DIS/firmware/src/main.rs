#![deny(unsafe_code)]
#![allow(non_snake_case)]

extern crate videocore;

extern crate ebola;

use ebola::texture;

use videocore::bcm_host;

fn main() {
    bcm_host::init();

    let mut window = ebola::CreateRenderWindow();

    let glContext = ebola::InitEGL(&mut window);

    let tex = ebola::texture::LoadTexture("~/firmware/data/test.jpg", 0);
    ebola::TickEGL(glContext);
}