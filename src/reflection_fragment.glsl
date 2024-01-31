#version 330

varying vec4 color; // from vertex
varying vec2 uv; // from vertex
varying vec2 fragTexCoord;

uniform float iTime;
uniform vec2 iResolution;
uniform sampler2D Texture;

void main() {
    vec2 flippedTexCoord = vec2(fragTexCoord.x, 1.0 - fragTexCoord.y);
    vec4 mirrored = texture(Texture, flippedTexCoord);

    float reflectivity = 0.7;
    vec4 finalColor = mirrored * reflectivity;
    gl_FragColor = finalColor;
}
