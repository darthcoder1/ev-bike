
precision mediump float;

varying vec4 v_color;
varying vec2 v_texCoords;

uniform sampler2D u_tex0;

void main() 
{
    vec4 color = texture2D(u_tex0, v_texCoords);
    gl_FragColor = color;
}