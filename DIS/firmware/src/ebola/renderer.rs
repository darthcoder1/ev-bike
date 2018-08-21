#![allow(non_snake_case)]

use std::path::Path;
use std::fs;

use opengles::glesv2 as gl;

pub struct RenderContext {
    pub shaderStages: Vec<ShaderStage>,
    pub clearColor : [f32;4],
    pub renderCommands : Vec<Vec<RenderCommand>>,
}


#[derive(Clone)]
pub struct AttributeBinding {

    // Handle to the attribute to bind to
    attributeHndl : gl::GLuint,
    
    // Handle to the data buffer (VBO)
    dataBufferHndl : gl::GLuint,
    
    // Number of components
    // This is the stride of the data. 
    numComponents: u32,
}

pub enum PrimitivesType {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    TriangleStrip,
    TriangleFan,
    Triangles,
}

fn ToGL(vit : & PrimitivesType) -> gl::GLenum {
    match vit {
        PrimitivesType::Points => gl::GL_POINTS,
        PrimitivesType::LineStrip => gl::GL_LINE_STRIP,
        PrimitivesType::LineLoop => gl::GL_LINE_LOOP,
        PrimitivesType::Lines => gl::GL_LINES,
        PrimitivesType::TriangleStrip => gl::GL_TRIANGLE_STRIP,
        PrimitivesType::TriangleFan => gl::GL_TRIANGLE_FAN,
        PrimitivesType::Triangles => gl::GL_TRIANGLES,
    }
}

pub struct RenderCommand {
    attributeBindings : Vec<AttributeBinding>,
    primitiveType : PrimitivesType,
    numPrimitives: u32,
}

impl RenderCommand {
    
    pub fn new (attributeBindings : Vec<AttributeBinding>, primitiveType : PrimitivesType, numPrimitives : u32) -> RenderCommand {
        RenderCommand {
            attributeBindings: attributeBindings,
            primitiveType: primitiveType,
            numPrimitives: numPrimitives,
        }
    }

    pub fn Execute(& self) {
        self.Bind();
        self.Draw(& self.primitiveType, self.numPrimitives);
        self.Unbind();
    }

    fn Bind(& self) {
        for binding in self.attributeBindings.iter() {
            // bind buffer
            gl::enable_vertex_attrib_array(binding.attributeHndl);
            gl::bind_buffer(gl::GL_ARRAY_BUFFER, binding.dataBufferHndl);
            gl::vertex_attrib_pointer_offset(binding.attributeHndl, binding.numComponents as gl::GLint, gl::GL_FLOAT, false, 0, 0);
        }
    }

    fn Draw(& self, primitvesType : & PrimitivesType, numPrimitives : u32) {
        gl::draw_arrays(ToGL(& primitvesType), 0, numPrimitives as gl::GLint);
    }

    fn Unbind(& self) {
        for binding in self.attributeBindings.iter() {
            gl::disable_vertex_attrib_array(binding.attributeHndl);
        }

         gl::bind_buffer(gl::GL_ARRAY_BUFFER, 0);
    }
}


pub struct ShaderProgram(gl::GLuint);
pub struct ShaderCode(gl::GLuint);


pub struct ShaderStage {
    program : ShaderProgram,
    fragShader : ShaderCode,
    vertShader : ShaderCode,
}

pub struct ShaderDataHndl(gl::GLuint);

impl ShaderStage {
    
    pub fn CreateBinding(&self, attributeName : & str, dataBuffer : u32, componentsPerVertex : u32) -> AttributeBinding {
        
        let attributeHndl = gl::get_attrib_location(self.program.0, attributeName) as gl::GLuint;

        if attributeHndl == gl::GL_INVALID_OPERATION
        {
            panic!("Failed to create binding for '{}'", attributeName);
        }

        AttributeBinding {
            attributeHndl: attributeHndl,
            dataBufferHndl: dataBuffer,
            numComponents: componentsPerVertex,
        }
    }
}

 pub fn Render(shaderStages : & Vec<ShaderStage>, commands : & Vec<Vec<RenderCommand>>) {
        
    for (i, stage) in shaderStages.iter().enumerate() {
        
        let commands = & commands[i];
        
        for cmd in commands.iter() {
            cmd.Execute();
        }
    }
}

impl Default for ShaderStage 
{
    fn default() -> ShaderStage {
        ShaderStage {
            program: ShaderProgram(0),
            fragShader: ShaderCode(0),
            vertShader: ShaderCode(0),
        }
    }
}

// LoadShader loads the shaderfiles located at the specified path.
// The path must omit the file extension. Then the system will look
// for '<path>.vert' and '<path>.frag' and load them accordingly.
pub fn LoadShaderStage(path : & str) -> Result<ShaderStage, ()> {

    let mut vertPath = path.to_owned();
    vertPath.push_str(".vert");
    
    let mut fragPath = path.to_owned();
    fragPath.push_str(".frag");

    if !Path::new(& vertPath).exists() || !Path::new(& fragPath).exists()
    {
        let errString = "Load shader for {} failed. Failed to find vertex/fragment shader.";
        panic!(errString);
    }

    let program = gl::create_program();

    // setup fragment shader
    let fragShader = LoadShaderInternal(& fragPath, gl::GL_FRAGMENT_SHADER).unwrap();
    gl::attach_shader(program, fragShader);
    // setup vertex shader
    let vertShader = LoadShaderInternal(& vertPath, gl::GL_VERTEX_SHADER).unwrap();
    gl::attach_shader(program, vertShader);

    gl::link_program(program);
    gl::use_program(program);

    Ok(ShaderStage{
        program: ShaderProgram(program),
        fragShader: ShaderCode(fragShader),
        vertShader: ShaderCode(vertShader),
    })
}

fn LoadShaderInternal(path : & str, shaderType : gl::GLenum) -> Result<gl::GLuint, ()>
{
    let shaderCode = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            panic!("Failed to load shader: {}", error);
        }
    };
    
    let shader = gl::create_shader(shaderType);

    gl::shader_source(shader, shaderCode.as_bytes());
    gl::compile_shader(shader);

    Ok(shader)
}