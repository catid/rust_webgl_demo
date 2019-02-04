precision mediump float;

uniform mat4 MVMatrix;
uniform mat4 ProjMatrix;

attribute vec3 VertexPosition;
attribute vec3 VertexColor;
attribute vec3 VertexNormal;

varying vec3 FragPosition;
varying vec3 FragColor;
varying vec3 FragNormal;

void main() {
	FragPosition = MVMatrix * vec4(VertexPosition, 1.0);
	gl_Position = ProjMatrix * FragPosition;
	FragColor = VertexColor;
	FragNormal = VertexNormal;
}
