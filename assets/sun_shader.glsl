#version 330 core

in vec3 pos;
in vec3 nor;

uniform vec4 color;
uniform float glow_intensity;
uniform float time;

out vec4 frag_color;

void main() {
    // Efecto de brillo pulsante
    float pulse = sin(time * 2.0) * 0.1 + 0.9;
    float intensity = glow_intensity * pulse;
    
    // Gradiente radial del sol
    float gradient = 1.0 - length(pos) * 0.5;
    gradient = max(gradient, 0.3);
    
    // Color base con variación
    vec3 base_color = color.rgb * gradient * intensity;
    
    // Efecto de borde brillante
    float edge = 1.0 - abs(dot(normalize(nor), normalize(-pos)));
    edge = pow(edge, 3.0) * 0.5;
    
    vec3 final_color = base_color + edge * color.rgb;
    frag_color = vec4(final_color, 1.0);
}