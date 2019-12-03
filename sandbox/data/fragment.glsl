#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

void main() {
    //target0 = texture(sampler2D(u_texture, u_sampler), v_uv * 1.5 + vec2(0.5));
    //target0 = vec4(v_uv, 0.0, 1.0);
    vec4 tex_color = texture(sampler2D(u_texture, u_sampler), v_uv);
    //vec4 uv_color = vec4(v_uv + vec2(0.5), 0.0, 1.0);
    //target0 = uv_color * tex_color;
    target0 = tex_color;
}
