#![allow(non_snake_case)]

extern crate image;

use std::path::Path;

use opengles::glesv2 as gl;

use gl::GLuint;
use gl::GLenum;

use self::image::GenericImage;

pub struct Texture
{
    identifier : GLuint,
    sampler : GLenum,
}

pub fn LoadTexture(path : &str, samplerIdx : i32) -> Texture {

    let img = image::open(&Path::new("data/textures/logo.jpg")).expect("Failed to load texture");
    let imgData = img.raw_pixels();

    let samplerEnum = match samplerIdx {
        0 => gl::GL_TEXTURE0,
        1 => gl::GL_TEXTURE1,
        2 => gl::GL_TEXTURE2,
        3 => gl::GL_TEXTURE3,
        4 => gl::GL_TEXTURE4,
        5 => gl::GL_TEXTURE5,
        6 => gl::GL_TEXTURE6,
        7 => gl::GL_TEXTURE7,
        _ => panic!("Possibly unsupported sampler index")
    };

    // create the texture identifier
    let textures = gl::gen_textures(1);

    // bind samples to texture identifier
    gl::active_texture(samplerEnum);

    gl::bind_texture(gl::GL_TEXTURE_2D, textures[0]);
    gl::tex_image_2d(gl::GL_TEXTURE_2D, 0, gl::GL_RGB as i32, img.width() as i32, img.height() as i32, 0, gl::GL_RGB, gl::GL_UNSIGNED_BYTE, &imgData[..]);

    //gl::tex_image_2d(gl::GL_TEXTURE_2D, level: GLint, internal_format: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, buffer: &[T])
    println!("Loaded texture {}", path);

    Texture {
        identifier: textures[0],
        sampler: samplerEnum,
    }
}