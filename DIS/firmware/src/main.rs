#![deny(unsafe_code)]
#![allow(non_snake_case)]

extern crate videocore;
extern crate ebola;

use videocore::bcm_host;


const DATA_PATH : & str = "/opt/firmware/data" ;

fn main() {
    bcm_host::init();

    let mut window = ebola::CreateRenderWindow();

    let glContext = ebola::InitEGL(&mut window);

    let texturePath = format!("{}/{}", DATA_PATH, "/test.jpg");
    let tex = ebola::texture::LoadTexture(& texturePath, 0);

    let shaderPath = format!("{}/{}", DATA_PATH, "/default");
    let defaultStage = ebola::renderer::LoadShaderStage(& shaderPath).unwrap();

    let shaderStages = vec![defaultStage];

    ebola::RunMainLoop(ebola::renderer::RenderContext  {
                                    shaderStages: shaderStages,
                                    clearColor: [1.0, 0.0, 0.0, 1.0],
                                },
                       glContext);
}