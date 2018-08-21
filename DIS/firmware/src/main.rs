#![deny(unsafe_code)]
#![allow(non_snake_case)]

extern crate videocore;
extern crate ebola;

use videocore::bcm_host;

use ebola::renderer;
use ebola::renderer::{RenderContext,RenderCommand, PrimitivesType};


const DATA_PATH : & str = "/opt/firmware/data" ;

fn PrepareStages() -> (renderer::ShaderStage, Vec<RenderCommand>) {
    
    let shaderPath = format!("{}/{}", DATA_PATH, "/default");
    let defaultStage = renderer::LoadShaderStage(& shaderPath).unwrap();

    let (color_vbo, vertex_vbo, texcoord_vbo) = (0,0,0);
    
    let bindings = vec![
        defaultStage.CreateBinding("a_vertex", vertex_vbo, 2),
        defaultStage.CreateBinding("a_color", color_vbo, 3),
        defaultStage.CreateBinding("a_texcoord", texcoord_vbo, 2),
    ];

    let renderCommands = vec![
        RenderCommand::new(bindings, PrimitivesType::TriangleFan, 3)
    ];

    (defaultStage, renderCommands)
}

fn main() {
    bcm_host::init();

    let mut window = ebola::CreateRenderWindow();

    let glContext = ebola::InitEGL(&mut window);

    let texturePath = format!("{}/{}", DATA_PATH, "/test.jpg");
    let tex = ebola::texture::LoadTexture(& texturePath, 0);

    
    let (shaderStage, renderCommands) = PrepareStages();
    

    ebola::RunMainLoop(RenderContext  {
                                    shaderStages: vec![shaderStage],
                                    clearColor: [1.0, 0.0, 0.0, 1.0],
                                    renderCommands: vec![renderCommands],
                                },
                       glContext);
}