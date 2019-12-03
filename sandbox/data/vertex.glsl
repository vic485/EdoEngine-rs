#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(constant_id = 0) const float scale = 1.0f;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;
layout(location = 0) out vec2 v_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    v_uv = a_uv;// * 1.5 + vec2(0.5);
    gl_Position = vec4(scale * a_pos, 1.0);
}
