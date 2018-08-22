#![deny(unsafe_code)]
#![allow(non_snake_case)]

extern crate videocore;
extern crate ebola;

use videocore::bcm_host;

use ebola::renderer;
use ebola::renderer::{
        RenderContext,
        RenderCommand,
        PrimitivesType,
        GPUBuffer,
        GPUBufferTarget,
        GPUBufferUsage
    };


const DATA_PATH : & str = "/opt/firmware/data" ;

fn PrepareStages() -> (renderer::ShaderStage, Vec<RenderCommand>) {
    
    let shaderPath = format!("{}/{}", DATA_PATH, "/default");
    let defaultStage = renderer::LoadShaderStage(& shaderPath).unwrap();

    let vertices = [ -1.0, -1.0,                // bottom left
                      1.0, -1.0,                // bottom right
                      0.0,  1.0 ] as [f32;6];   // top

    let colors = [ 1.0, 0.0, 0.0, 
                   0.0, 1.0, 0.0, 
                   0.0, 0.0, 1.0 ] as [f32; 9];

    let texCoords = [ -1.0, -1.0,                // bottom left
                      1.0, -1.0,                // bottom right
                      0.0,  0.0 ] as [f32;6];   // top

    let vertexData = renderer::GPUBuffer::new(& vertices, GPUBufferTarget::Array, GPUBufferUsage::Static);
    let colorData = renderer::GPUBuffer::new(& colors, GPUBufferTarget::Array, GPUBufferUsage::Static);
    let texCoordsData = renderer::GPUBuffer::new(& texCoords, GPUBufferTarget::Array, GPUBufferUsage::Static);

    let bindings = vec![
        defaultStage.CreateBinding("a_vertex", & vertexData, 2),
        defaultStage.CreateBinding("a_color", & colorData, 3),
        defaultStage.CreateBinding("a_texcoord", & texCoordsData, 2),
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

    //let texturePath = format!("{}/{}", DATA_PATH, "/test.jpg");
    //let tex = ebola::texture::LoadTexture(& texturePath, 0);

    
    let (shaderStage, renderCommands) = PrepareStages();
    

    ebola::RunMainLoop(RenderContext  {
                                    shaderStages: vec![shaderStage],
                                    clearColor: [1.0, 0.0, 0.0, 1.0],
                                    renderCommands: vec![renderCommands],
                                },
                       glContext);
}