precision mediump float;

uniform vec3 Light0ViewPosition; // ViewMatrix * Light0Position

varying vec3 FragPosition;
varying vec3 FragColor;
varying vec3 FragNormal;

float attenuation(float r, float f, float d) {
    float denom = d / r + 1.0;
    float attenuation = 1.0 / (denom*denom);
    float t = (attenuation - f) / (1.0 - f);
    return max(t, 0.0);
}

void main() {
    vec3 lightVector = Light0ViewPosition - FragPosition;
    float lightDistance = length(lightVector);
    float falloff = attenuation(light.radius, light.falloff, lightDistance);
    vec3 L = normalize(lightVector);

    gl_FragColor = vec4(FragColor, 1.0);
}
