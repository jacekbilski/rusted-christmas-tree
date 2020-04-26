#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform Material material;
in vec3 FragPosition;
in vec3 Normal;

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

layout (std140) uniform Lights {
    Light light;
};

out vec4 FragColor;

void main() {
    vec3 ambient = light.ambient * material.ambient;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPosition);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light.diffuse * material.diffuse;

    vec3 viewDir = normalize(cameraPosition - FragPosition);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = spec * light.specular * material.specular;

    FragColor = vec4((ambient + diffuse + specular), 1.0);
}
