#![deny(unsafe_code)]
#![allow(non_snake_case)]

extern crate videocore;
extern crate ebola;

use ebola::texture;
use videocore::bcm_host;


const DATA_PATH : & str = "/opt/firmware/data" ;

fn main() {
    bcm_host::init();

    let mut window = ebola::CreateRenderWindow();

    let glContext = ebola::InitEGL(&mut window);

    let texturePath = format!("{}/{}", DATA_PATH, "/test.jpg");
    let tex = ebola::texture::LoadTexture(& texturePath, 0);

    let shaderPath = format!("{}/{}", DATA_PATH, "/default");
    let defaultStage = ebola::LoadShaderStage(& shaderPath);

    ebola::RunMainLoop(ebola::Context {
                                    shaderStages: vec![defaultStage],
                                }
                       glContext);
}