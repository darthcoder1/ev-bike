#![allow(non_snake_case)]

use std::path::Path;
use std::fs;
use std::time::Instant;

use opengles::glesv2 as gl;

pub struct RenderContext {
    pub shaderStages: Vec<ShaderStage>,
    pub clearColor : [f32;4],
    pub renderCommands : Vec<Vec<RenderCommand>>,
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

pub fn ToGL(vit : & PrimitivesType) -> gl::GLenum {
    match *vit {
        PrimitivesType::Points => gl::GL_POINTS,
        PrimitivesType::LineStrip => gl::GL_LINE_STRIP,
        PrimitivesType::LineLoop => gl::GL_LINE_LOOP,
        PrimitivesType::Lines => gl::GL_LINES,
        PrimitivesType::TriangleStrip => gl::GL_TRIANGLE_STRIP,
        PrimitivesType::TriangleFan => gl::GL_TRIANGLE_FAN,
        PrimitivesType::Triangles => gl::GL_TRIANGLES,
    }
}

////////////////////////////////////
// Render Command

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

enum UniformTypedData {
    Integer(Vec<i32>),
    Float(Vec<f32>),
}

pub struct UniformBinding {

    handle : gl::GLint,
    data : UniformTypedData,
}

pub struct RenderCommand {
    attributeBindings : Vec<AttributeBinding>,
    uniformBindings : Vec<UniformBinding>,
    primitiveType : PrimitivesType,
    numVertices: u32,
}

impl RenderCommand {
    
    pub fn new (attributeBindings : Vec<AttributeBinding>, uniformBindings : Vec<UniformBinding>, primitiveType : PrimitivesType, numVertices : u32) -> RenderCommand {
        RenderCommand {
            attributeBindings: attributeBindings,
            uniformBindings : uniformBindings,
            primitiveType: primitiveType,
            numVertices: numVertices,
        }
    }

    pub fn Execute(& self) {
        self.Bind();
        self.Draw();
        self.Unbind();
    }

    fn Bind(& self) {
        for binding in self.attributeBindings.iter() {
            
            gl::enable_vertex_attrib_array(binding.attributeHndl);
            gl::bind_buffer(gl::GL_ARRAY_BUFFER, binding.dataBufferHndl);
            gl::vertex_attrib_pointer_offset(binding.attributeHndl, binding.numComponents as gl::GLint, gl::GL_FLOAT, false, 0, 0);
        }

        for binding in self.uniformBindings.iter() {
            match binding.data {
                UniformTypedData::Integer(ref intVec) => self.BindIntegers(binding.handle, & intVec),
                UniformTypedData::Float(ref floatVec) => self.BindFloats(binding.handle, & floatVec),
            }
        }
    }

    fn BindIntegers(&self, uniformHndl : gl::GLint, data : & Vec<i32>) {
        match data.len() {
            0 => panic!("No elements. Cannot bind empty array "),
            1 => gl::uniform1i(uniformHndl, data[0]),
            2 => gl::uniform2i(uniformHndl, data[0], data[1]),
            3 => gl::uniform3i(uniformHndl, data[0], data[1], data[2]),
            4 => gl::uniform4i(uniformHndl, data[0], data[1], data[2], data[3]),
            _ => panic!("Invalid data size for integer uniform"),
        }
    }

    fn BindFloats(&self, uniformHndl : gl::GLint, data : & Vec<f32>) {
        match data.len() {
            0 => panic!("No elements. Cannot bind empty array "),
            1 => gl::uniform1f(uniformHndl, data[0]),
            2 => gl::uniform2f(uniformHndl, data[0], data[1]),
            3 => gl::uniform3f(uniformHndl, data[0], data[1], data[2]),
            4 => gl::uniform4f(uniformHndl, data[0], data[1], data[2], data[3]),
            _ => panic!("Invalid data size for float uniform"),
        }
    }

    fn Draw(& self) {
        gl::draw_arrays(ToGL(& self.primitiveType), 0, self.numVertices as gl::GLint);
    }

    fn Unbind(& self) {
        for binding in self.attributeBindings.iter() {
            gl::disable_vertex_attrib_array(binding.attributeHndl);
        }

         gl::bind_buffer(gl::GL_ARRAY_BUFFER, 0);
    }
}


////////////////////////////////////
// GPUBuffer

pub enum GPUBufferTarget {
    Array,
    ElementArray,
}

pub enum GPUBufferUsage {
    Stream,
    Static,
    Dynamic,
}

pub struct GPUBuffer {

    pub handle : gl::GLuint,
    target : GPUBufferTarget,
    usage : GPUBufferUsage,
}

impl GPUBuffer {

    pub fn new<T>(cpuData : &[T], target : GPUBufferTarget, usage : GPUBufferUsage) -> GPUBuffer{
        let vbo = gl::gen_buffers(1)[0];
    
        let glTarget = TargetToGL(& target);

        gl::bind_buffer(glTarget, vbo);
        gl::buffer_data(glTarget, cpuData, UsageToGL(& usage));

        GPUBuffer {
            handle : vbo,
            target: target,
            usage: usage,
        }
    }
}


fn TargetToGL(vit : & GPUBufferTarget) -> gl::GLenum {
    match *vit {
        GPUBufferTarget::Array => gl::GL_ARRAY_BUFFER,
        GPUBufferTarget::ElementArray => gl::GL_ELEMENT_ARRAY_BUFFER,
    }
}

fn UsageToGL(vit : & GPUBufferUsage) -> gl::GLenum {
    match *vit {
        GPUBufferUsage::Stream => gl::GL_STREAM_DRAW,
        GPUBufferUsage::Static => gl::GL_STATIC_DRAW,
        GPUBufferUsage::Dynamic => gl::GL_DYNAMIC_DRAW,
    }
}

////////////////////////////////////
// ShaderStage
pub struct ShaderProgram(gl::GLuint);
pub struct ShaderCode(gl::GLuint);


pub struct ShaderStage {
    program : ShaderProgram,
    fragShader : ShaderCode,
    vertShader : ShaderCode,
}

pub struct ShaderDataHndl(gl::GLuint);

impl ShaderStage {
    
    pub fn BindAttribute(&self, attributeName : & str, buffer : & GPUBuffer, componentsPerVertex : u32) -> AttributeBinding {
        
        let attributeHndl = gl::get_attrib_location(self.program.0, attributeName) as gl::GLuint;

        if attributeHndl == gl::GL_INVALID_OPERATION
        {
            panic!("Failed to bind attribute '{}'", attributeName);
        }

        AttributeBinding {
            attributeHndl: attributeHndl,
            dataBufferHndl: buffer.handle,
            numComponents: componentsPerVertex,
        }
    }

    pub fn BindUniform(&self, uniformName : & str, uniformData : Vec<i32>) -> UniformBinding {
        
        let hndl = gl::get_uniform_location(self.program.0, uniformName);
        
        if hndl < 0
        {
            panic!("Failed to bind uniform '{}'", uniformName);
        }

        UniformBinding {
            handle: hndl,
            data: UniformTypedData::Integer(uniformData.clone()),
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

    let startTime = Instant::now();

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

    if gl::get_programiv(program, gl::GL_LINK_STATUS) == gl::GL_FALSE as i32 {
        match gl::get_program_info_log(program, 1024) {
            Some(log) => println!("Failed to link shaders: {}\n{}",path, log),
            None => ()
        }
    }

    gl::use_program(program);

    println!("LoadShaderStage({}) -- {}ms", path, startTime.elapsed().as_millis());
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
    println!("LoadShader({}) -> {}", path, shader);

    gl::shader_source(shader, shaderCode.as_bytes());
    gl::compile_shader(shader);

    if gl::get_shaderiv(shader, gl::GL_COMPILE_STATUS) == gl::GL_FALSE as i32 {
        match gl::get_shader_info_log(shader, 1024) {
            Some(log) => println!("Compilation Errors:\n{}",log),
            None => ()
        }
    }

    Ok(shader)
}