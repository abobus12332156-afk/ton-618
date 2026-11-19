#version 450

const vec2 position[4] = vec2[](
    vec2(-1.0, -1.0),
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0,  1.0)
);

const uint indices[6] = uint[](0, 1, 2, 1, 3, 2);

void main()
{
    uint index = indices[gl_VertexIndex];
    gl_Position = vec4(position[index], 0.0, 1.0);
}