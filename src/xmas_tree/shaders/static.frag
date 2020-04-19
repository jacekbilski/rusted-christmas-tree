#version 330 core

in vec3 FragPosition;
in vec3 Colour;
in vec3 Normal;

uniform vec3 lightColour;
uniform vec3 lightPosition;
uniform vec3 cameraPosition;

out vec4 FragColor;

const float ambientStrength = 0.1;
const float specularStrength = 0.5;

void main() {
    vec3 ambient = ambientStrength * lightColour;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPosition - FragPosition);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColour;

    vec3 viewDir = normalize(cameraPosition - FragPosition);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColour;

    FragColor = vec4((ambient + diffuse + specular) * Colour, 1.0);
}