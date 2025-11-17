#version 330 core

in vec3 pos;
in vec3 nor;

uniform vec4 color;
uniform vec3 sun_position;

out vec4 frag_color;

void main() {
    // Vector hacia el sol
    vec3 to_sun = normalize(sun_position - pos);
    
    // Iluminación difusa simple
    float diffuse = max(dot(normalize(nor), to_sun), 0.1);
    
    // Efecto de terminador (límite día/noche)
    float terminator = smoothstep(0.0, 0.2, diffuse);
    
    // Color final con iluminación
    vec3 final_color = color.rgb * diffuse;
    
    // Añadir un poco de ambient light
    final_color += color.rgb * 0.2;
    
    frag_color = vec4(final_color, 1.0);
}