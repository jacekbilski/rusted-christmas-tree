#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aCol;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in mat4 instanceModel;

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

out vec3 FragPosition;
out vec3 Colour;
out vec3 Normal;

void main() {
    vec4 pos = instanceModel * vec4(aPos, 1.0);
    gl_Position = projection * view * pos;
    FragPosition = vec3(pos);
    Colour = aCol;
    Normal = mat3(transpose(inverse(instanceModel))) * aNormal;
}
