#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(binding = 0) uniform Matrices {
	mat4 inv_proj;
	mat4 view;
} matrices;

layout(location = 0) out vec3 outColor;

vec2 positions[3] = vec2[](
  vec2(0.0, -0.5),
  vec2(0.5, 0.5),
  vec2(-0.5, 0.5)
);

vec3 colors[3] = vec3[](
  vec3(1.0, 0.0, 0.0),
  vec3(0.0, 1.0, 0.0),
  vec3(0.0, 0.0, 1.0)
);

void main() {
  gl_Position = matrices.inv_proj * vec4(positions[gl_VertexIndex], -2.0, 1.0);
  // outColor = vec3(matrices.inv_proj[0][2], matrices.inv_proj[1][2], matrices.inv_proj[2][2]);
  outColor = colors[gl_VertexIndex];
}