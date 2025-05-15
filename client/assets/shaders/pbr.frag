#version 330 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D albedoMap;
uniform sampler2D normalMap;
uniform sampler2D roughnessMap;
uniform sampler2D heightMap;

uniform vec3 camPos;
uniform vec3 lightPos;
uniform vec3 lightColor;

void main() {
    vec3 albedo = texture(albedoMap, TexCoords).rgb;
    float roughness = texture(roughnessMap, TexCoords).r;
    vec3 N = normalize(texture(normalMap, TexCoords).rgb * 2.0 - 1.0);
    vec3 V = normalize(camPos - FragPos);
    vec3 L = normalize(lightPos - FragPos);
    vec3 H = normalize(V + L);

    float distance = length(lightPos - FragPos);
    float attenuation = 1.0 / (distance * distance);
    vec3 radiance = lightColor * attenuation;

    float NDF = roughness;
    float G = roughness;
    vec3 F = vec3(0.04); 

    vec3 numerator = NDF * G * F;
    float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.001;
    vec3 specular = numerator / denominator;

    float NdotL = max(dot(N, L), 0.0);
    vec3 color = (albedo / 3.141592 + specular) * radiance * NdotL;

    FragColor = vec4(color, 1.0);
}
