
attribute vec4  a_color;
attribute vec4  a_vertex;

varying vec4    v_color;

uniform vec2    u_screenSize = vec2(1024,600);

void main() 
{
    // divide to get from pixels to 0-1 scale
    vec4 transformedPos = a_vertex / vec4(screenSize.xy, 1.0, 1.0);
    // offset the pos to match opengl coordinates ( with (0,0) at the center)
    transformedPos += vec4(-1.0, -1.0, 0.0, 0.0);
    
    gl_Position = transformedPos;
    v_color = a_color;
}