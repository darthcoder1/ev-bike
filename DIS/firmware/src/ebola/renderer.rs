#![allow(non_snake_case)]

pub struct RenderContext {
    shaderStages: Vec<ShaderStage>,
    clearColor : [float;4],
}


struct AttributeBinding {
    attributeHndl : gl::GLuint,
    dataBufferHndl : gl::GLuint,
}

pub struct RenderCommand {
    attributeBindings : Vec<AttributeBinding>,
}

impl RenderCommand {
    
    // Binds the specified data buffer to the attribute
    fn SetupBinding(& self, attributeName: & str, GL) {

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

    if (!Path::new(& vertPath).exists() || !Path::new(& fragPath).exists()) 
    {
        let errString = "Load shader for {} failed. Failed to find vertex/fragment shader.";
        panic!(errString);
        return Err(());
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
            return Err(());
        }
    };
    
    let shader = gl::create_shader(shaderType);

    gl::shader_source(shader, shaderCode.as_bytes());
    gl::compile_shader(shader);

    Ok(shader)
}