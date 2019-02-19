precision mediump float;

uniform mat4 MVPMatrix;

attribute vec3 VertexPosition;
attribute vec3 VertexColor;
attribute vec3 VertexNormal;

varying vec3 FragColor;

void main() {
	gl_Position = MVPMatrix * vec4(VertexPosition.xyz, 0);
	FragColor = VertexColor;
}
