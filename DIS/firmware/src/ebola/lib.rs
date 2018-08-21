#![feature(duration_as_u128)]
#![allow(non_snake_case)]

pub mod texture;
pub mod renderer;

extern crate videocore;
extern crate egl;
extern crate opengles;

use std::ptr;
use std::time::Instant;

use videocore::bcm_host;
use videocore::dispmanx;
use videocore::image::Rect;
use videocore::dispmanx::{ Window,
                           FlagsAlpha,
                           VCAlpha,
                           Transform };

use egl::{ EGLConfig,
           EGLContext,
           EGLDisplay,
           EGLNativeDisplayType,
           EGLSurface };

use opengles::glesv2 as gl;

// contains all context relevant EGL data
pub struct GLContext {
    pub config:  EGLConfig,
    pub context: EGLContext,
    pub display: EGLDisplay,
    pub surface: EGLSurface
}

pub fn CreateRenderWindow() -> Window {
    
    // open the display
    let display = dispmanx::display_open(0);
    // get the update handle
    let update_hndl = dispmanx::update_start(0);

    // query the screen resolution of the connected screen
    let screenRes = match bcm_host::graphics_get_display_size(0) {
        Some(x) => x,
        None => panic!("bcm_host::init() has not been called prior to creating a window.")
    };

    println!("Screen Resolution: {}x{}", screenRes.width, screenRes.height);

    let mut dest_rect = Rect {
        x:      0,
        y:      0,
        width:  screenRes.width as i32,
        height: screenRes.height as i32,
    };

    let mut src_rect = Rect {
        x:      0,
        y:      0,
        width:  0,
        height: 0,
    };

    let mut alpha = VCAlpha { 
        flags: FlagsAlpha::FIXED_ALL_PIXELS,
        opacity: 255,
        mask: 0,
    };

    let element = dispmanx::element_add(update_hndl, 
                                        display, 
                                        3, // layer to draw on
                                        & mut dest_rect, 
                                        0, 
                                        & mut src_rect, 
                                        dispmanx::DISPMANX_PROTECTION_NONE, 
                                        & mut alpha, 
                                        ptr::null_mut(), 
                                        Transform::NO_ROTATE);

    // submit display setup
    dispmanx::update_submit_sync(update_hndl);

    Window { 
        element:    element,
        width:      screenRes.width as i32,
        height:     screenRes.height as i32,
    }
}

pub fn InitEGL(window : & mut Window) -> GLContext {
    
    let context_attr = [ egl::EGL_CONTEXT_CLIENT_VERSION, 2, egl::EGL_NONE ];
    
    let config_attr = [ egl::EGL_RED_SIZE,      8,
                        egl::EGL_GREEN_SIZE,    8,
                        egl::EGL_BLUE_SIZE,     8,
                        egl::EGL_ALPHA_SIZE,    8,
                        egl::EGL_SURFACE_TYPE,  egl::EGL_WINDOW_BIT,
                        egl::EGL_NONE ];

    let egl_display = match egl::get_display(egl::EGL_DEFAULT_DISPLAY) {
        Some(x) => x,
        None    => panic!("Failed to get EGL display")
    };

    if !egl::initialize(egl_display, &mut 0i32, &mut 0i32) {
        panic!("Failed to initialize EGL");
    }

    // select first config
    let egl_config = match egl::choose_config(egl_display, & config_attr, 1) {
        Some(x)     => x,
        None        => panic!("Failed to find compatible EGL config")
    };

    if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
        panic!("Failed to bind OpenGL ES API");
    }

    // create the egl context
    let egl_context = match egl::create_context(egl_display, egl_config, egl::EGL_NO_CONTEXT, &context_attr) {
        Some(context)   => context,
        None            => panic!("Failed to create EGL context")
    };

    let egl_surface = match egl::create_window_surface(egl_display, egl_config, window as *mut _ as EGLNativeDisplayType, &[]) {
        Some(surface)   => surface,
        None            => panic!("Failed to create EGL surface")
    };

    // activate context
    if !egl::make_current(egl_display, egl_surface, egl_surface, egl_context) {
        panic!("Failed to activate EGL context");
    }

    GLContext {
        config: egl_config,
        context: egl_context,
        display: egl_display,
        surface: egl_surface,
    }
}




pub fn RunMainLoop(renderCtx : renderer::RenderContext, glCtx : GLContext) {
    
    let screen_res = bcm_host::graphics_get_display_size(0).unwrap();

    gl::viewport(0, 0, screen_res.width as i32, screen_res.height as i32);

    let shader = SetupShaders();

    let a_color = gl::get_attrib_location(shader, "a_color");
    let a_vertex = gl::get_attrib_location(shader, "a_vertex");
    let a_texcoord = gl::get_attrib_location(shader, "a_texcoord");

    let (vertex_vbo, color_vbo, texcoord_vbo) = SetupGeometry();

    let mut delta_time_ms = 0;
    
    loop {

        let time_now = Instant::now();
   
        gl::clear_color(renderCtx.clearColor[0] , renderCtx.clearColor[1], renderCtx.clearColor[2], renderCtx.clearColor[3]);
        gl::clear(gl::GL_COLOR_BUFFER_BIT);

        // bind color buffer
        gl::enable_vertex_attrib_array(a_color as gl::GLuint);
        gl::bind_buffer(gl::GL_ARRAY_BUFFER, color_vbo);
        gl::vertex_attrib_pointer_offset(a_color as gl::GLuint, 3, gl::GL_FLOAT, false, 0, 0);

        // bind vertex buffer
        gl::enable_vertex_attrib_array(a_vertex as gl::GLuint);
        gl::bind_buffer(gl::GL_ARRAY_BUFFER, vertex_vbo);
        gl::vertex_attrib_pointer_offset(a_vertex as gl::GLuint, 2, gl::GL_FLOAT, false, 0, 0);

        // bind texcoord buffer
        gl::enable_vertex_attrib_array(a_texcoord as gl::GLuint);
        gl::bind_buffer(gl::GL_ARRAY_BUFFER, texcoord_vbo);
        gl::vertex_attrib_pointer_offset(a_texcoord as gl::GLuint, 2, gl::GL_FLOAT, false, 0, 0);

        gl::draw_arrays(gl::GL_TRIANGLE_FAN, 0, 3);

        // disable attributes
        gl::disable_vertex_attrib_array(a_color as gl::GLuint);
        gl::disable_vertex_attrib_array(a_vertex as gl::GLuint);
        gl::disable_vertex_attrib_array(a_texcoord as gl::GLuint);

        // unbind buffers
        gl::bind_buffer(gl::GL_ARRAY_BUFFER, 0);

        // swap
        egl::swap_buffers(glCtx.display, glCtx.surface);

        delta_time_ms = time_now.elapsed().as_millis();
        println!("DeltaTime: {}", delta_time_ms as u64);
    }
}

pub fn SetupShaders() -> gl::GLuint {
    
    let program = gl::create_program();

    // setup fragment shader
    let frag_prog = gl::create_shader(gl::GL_FRAGMENT_SHADER);

    gl::shader_source(frag_prog, 
    "
    varying vec4 v_color;
    varying vec2 v_texCoord;
    
    uniform sampler2D tex0;

    void main() {
        gl_FragColor = texture2d(tex0, v_texCoord).bgra;
    }
    ".as_bytes());

    gl::compile_shader(frag_prog);
    gl::attach_shader(program, frag_prog);

    // setup vertex shader
    let vert_prog = gl::create_shader(gl::GL_VERTEX_SHADER);

    gl::shader_source(vert_prog, 
    "
    attribute vec4  a_color;
    attribute vec4  a_vertex;
    varying vec4    v_color;

    void main() {
        gl_Position = a_vertex;
        v_color = a_color;
    }".as_bytes());

    gl::compile_shader(vert_prog);
    gl::attach_shader(program, vert_prog);

    // link
    gl::link_program(program);
    gl::use_program(program);

    program
}

pub fn SetupGeometry() -> (gl::GLuint, gl::GLuint, gl::GLuint) {
    let vbos = gl::gen_buffers(3);
    
    let vertices = [ -1.0, -1.0,                // bottom left
                      1.0, -1.0,                // bottom right
                      0.0,  1.0 ] as [f32;6];   // top

    gl::bind_buffer(gl::GL_ARRAY_BUFFER, vbos[0]);
    gl::buffer_data(gl::GL_ARRAY_BUFFER, &vertices, gl::GL_STATIC_DRAW);

    let colors = [ 1.0, 0.0, 0.0, 
                   0.0, 1.0, 0.0, 
                   0.0, 0.0, 1.0 ] as [f32; 9];
                   
    gl::bind_buffer(gl::GL_ARRAY_BUFFER, vbos[1]);
    gl::buffer_data(gl::GL_ARRAY_BUFFER, &colors, gl::GL_STATIC_DRAW);

    let texCoords = [ -1.0, -1.0,                // bottom left
                      1.0, -1.0,                // bottom right
                      0.0,  0.0 ] as [f32;6];   // top

    gl::bind_buffer(gl::GL_ARRAY_BUFFER, vbos[2]);
    gl::buffer_data(gl::GL_ARRAY_BUFFER, &texCoords, gl::GL_STATIC_DRAW);

    (vbos[0], vbos[1], vbos[2])
}

