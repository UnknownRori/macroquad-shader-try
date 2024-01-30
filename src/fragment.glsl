#version 100

precision lowp float;

varying vec4 color; // from vertex
varying vec2 uv; // from vertex

uniform float iTime;
uniform vec2 iResolution;
// uniform sampler2D Texture; // have no idea what this thing do

vec3 palette( float t ) {
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.5, 0.5);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.263,0.416,0.557);

    return a + b*cos( 6.28318*(c*t+d) );
}

void main() {
    vec2 uv0 = uv;
    uv0.x *= iResolution.x / iResolution.y;
    vec2 uv1 = uv0;
    vec3 finalColor = vec3(0.0);

    for (float i = 0.0; i < 5.0; i++) {
        uv0 = fract(uv0 * 1.5) - 0.5;

        float d = length(uv0) * exp(-length(uv1));

        vec3 col = palette(length(uv1) + i*.4 + iTime*.4);

        d = sin(d*8. + iTime)/8.;
        d = abs(d);

        d = pow(0.01 / d, 1.2);
        d = smoothstep(0.0, 1.0, d);

        finalColor += col * d;
    }


    gl_FragColor = vec4(finalColor, 1.0);
}
