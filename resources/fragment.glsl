#version 330 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;
in vec4 Color;

uniform sampler2D ourTexture;
uniform vec3 lightPosition;

void main()
{
    vec3 normal = normalize(Normal);
    vec3 lightDirection = normalize(lightPosition - FragPos);
    float diffuse = max(dot(normal, lightDirection), 0.0);
    FragColor = texture(ourTexture, TexCoord) * Color * vec4(diffuse, diffuse, diffuse, 1.0);
    if (FragColor.a == 0.0) {
        discard;
    }
}
