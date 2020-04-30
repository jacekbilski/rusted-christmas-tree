#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec4 specular;
};

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 FragPosition;
in vec3 Normal;
flat in uint MaterialId;

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

layout (std140) uniform Lights {
    int lightsNo;
    Light light[4];
};

layout (std140) uniform Materials {
    Material material[100];
};

out vec4 FragColor;

vec3 calcLight(Light light);

void main() {
    vec3 result = vec3(0.0);
    for (int i = 0; i < lightsNo; i++) {
        result += calcLight(light[i]);
    }
    FragColor = vec4(result, 1.0);
}

vec3 calcLight(Light light) {
    vec3 ambient = light.ambient * material[MaterialId].ambient;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPosition);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light.diffuse * material[MaterialId].diffuse;

    vec3 viewDir = normalize(cameraPosition - FragPosition);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwayDir), 0.0), material[MaterialId].specular.w);
    vec3 specular = spec * light.specular * vec3(material[MaterialId].specular);

    return ambient + diffuse + specular;
}
