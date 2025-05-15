#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
in vec3 Normal;
in vec3 FragPos;

uniform sampler2D baseTexture;
uniform sampler2D roughnessTexture;
uniform sampler2D normalTexture;
uniform sampler2D heightTexture;

uniform vec3 lightPos;
uniform vec3 viewPos;

void main() {
    vec3 baseColor = texture(baseTexture, TexCoords).rgb;
    float roughness = texture(roughnessTexture, TexCoords).r;
    vec3 normalMap = texture(normalTexture, TexCoords).rgb;
    normalMap = normalize(normalMap * 2.0 - 1.0);

    vec3 norm = normalize(normalMap);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);

    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 64.0) * (1.0 - roughness);

    vec3 result = (diff + spec) * baseColor;
    FragColor = vec4(result, 1.0);
}
