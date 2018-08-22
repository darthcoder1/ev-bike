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

struct Vector2 {
    x: f32, 
    y: f32,
}

struct Vector3 {
    x:f32, 
    y:f32,
    z:f32,
}

struct Geometry {
    vertices : GPUBuffer,
    colors : GPUBuffer,
    texcoords : GPUBuffer,
}

fn CreateRenderQuad(pos : & Vector2, size : & Vector2, color : Vector3) -> Geometry {

    let vertices = [ pos.x, pos.y,                          // top left
                     pos.x, pos.y + size.y,                 // bottom left
                     pos.x + size.x, pos.y + size.y,        // bottom right
                     pos.x + size.x, pos.y] as [f32;8];     // top right

    let colors = [ color.x, color.y, color.z, 
                   color.x, color.y, color.z, 
                   color.x, color.y, color.z,
                   color.x, color.y, color.z ] as [f32; 12];

    let texCoords = [ -1.0, -1.0,                // top left
                      -1.0,  1.0,                // bottom left
                       1.0,  1.0,                // bottom right
                       1.0,  1.0 ] as [f32;8];   // top right

    let vertexData = renderer::GPUBuffer::new(& vertices, GPUBufferTarget::Array, GPUBufferUsage::Static);
    let colorData = renderer::GPUBuffer::new(& colors, GPUBufferTarget::Array, GPUBufferUsage::Static);
    let texCoordsData = renderer::GPUBuffer::new(& texCoords, GPUBufferTarget::Array, GPUBufferUsage::Static);   

    Geometry {
        vertices : vertexData,
        colors : colorData,
        texcoords : texCoordsData,
    }
}

fn PrepareStages() -> (renderer::ShaderStage, Vec<RenderCommand>) {
    
    let shaderPath = format!("{}/{}", DATA_PATH, "default");
    let defaultStage = renderer::LoadShaderStage(& shaderPath).unwrap();

    
    let greenQuad = CreateRenderQuad(& Vector2{ x: 0.0, y: 50.0 }, & Vector2 { x:100.0, y:100.0 }, Vector3 { x:0.0, y:1.0, z: 0.0 });
    let greenQuadBindings = vec![
        defaultStage.CreateBinding("a_vertex", & greenQuad.vertices, 2),
        defaultStage.CreateBinding("a_color", & greenQuad.colors, 3),
    ];

    let blueQuad = CreateRenderQuad(& Vector2{ x: 10.0, y: 550.0 }, & Vector2 { x:1004.0, y:500.0 }, Vector3 { x:0.0, y:0.0, z: 1.0 });
    let blueQuadBindings = vec![
        defaultStage.CreateBinding("a_vertex", & blueQuad.vertices, 2),
        defaultStage.CreateBinding("a_color", & blueQuad.colors, 3),
    ];

    let renderCommands = vec![
        RenderCommand::new(greenQuadBindings, PrimitivesType::TriangleFan, 4),
        RenderCommand::new(blueQuadBindings, PrimitivesType::TriangleFan, 4)
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