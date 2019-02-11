precision mediump float;

varying vec3 FragColor;

// https://github.com/stackgl/glsl-lighting-walkthrough#flat-normals

void main() {
    gl_FragColor = vec4(FragColor, 1.0);
}
