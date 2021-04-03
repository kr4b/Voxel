#version 430

layout (location = 0) out vec2 texCoord;
layout (location = 1) out vec3 dir;
layout (location = 2) out vec3 origin;

uniform mat4 inv_proj;
uniform mat4 view;

void main() {
	//      tex   pos
	// 0: (0, 0) (-1, -1)
	// 1: (2, 0) (3, -1)
	// 2: (0, 2) (-1, 3)
	texCoord = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
	vec2 pos = texCoord * 2.0 - 1.0;
	gl_Position = vec4(pos, 0.0, 1.0);

	dir = (view * vec4((inv_proj * vec4(pos, 1.0, 1.0)).xyz, 0.0)).xyz;
	origin = vec3(view[3][0], view[3][1], view[3][2]);
}