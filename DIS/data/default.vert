
attribute vec4  a_color;
attribute vec4  a_vertex;
attribute vec2  a_texCoord;

varying vec4    v_color;
varying vec2    v_texCoords;

void main() 
{
    vec2 screenSize = vec2(1024, 600);

    // divide to get from pixels to 0-1 scale
    vec4 transformedPos = a_vertex / vec4(screenSize.x, screenSize.y, 1.0, 1.0);
    transformedPos.y = transformedPos.y * -1.0;transformedPos.y;
    // offset the pos to match opengl coordinates ( with (0,0) at the center)
    transformedPos += vec4(-1.0, 1.0, 0.0, 0.0);
        
    gl_Position = transformedPos;
    v_color = a_color;
    v_texCoords = a_texCoord;
}