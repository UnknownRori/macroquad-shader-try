#version 330

attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;
out vec2 fragTexCoord;

uniform float iTime;
uniform vec2 iResolution;
uniform mat4 Model;
uniform mat4 Projection;
uniform sampler2D Texture;

void main() {
    float waveHeight = 3.4;
    float frequency = 4.0;

    vec3 newPosition = position;
    newPosition.y += 20.0;
    newPosition.y += waveHeight * sin(iTime * frequency + position.x);

    float distortionAmount = 1.02;
    newPosition.x += distortionAmount * sin(iTime * 2.0 * frequency + position.y);
    newPosition.z += distortionAmount * cos(iTime * 1.5 * frequency + position.x);
    newPosition.x = abs(newPosition.x);
    newPosition.y = abs(newPosition.y);
    newPosition.z = abs(newPosition.z);

    gl_Position = Projection * Model * vec4(newPosition, 1.0);
    fragTexCoord = texcoord;
    color = color0 / 255.0;
    uv = (texcoord - textureSize(Texture, 0).xy) / textureSize(Texture, 0).y;
}

