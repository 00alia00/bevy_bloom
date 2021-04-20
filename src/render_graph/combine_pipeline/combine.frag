
// From https://learnopengl.com/Advanced-Lighting/Bloom
#version 450

layout (location = 0)  in vec2 TexCoords;
layout (location = 1) out vec4 FragColor;

layout(set = 2, binding = 0) uniform texture2D scene_texture; 
layout(set = 2, binding = 1) uniform sampler scene_texture_sampler;

// TODO: this is probably wrong
layout(set = 3, binding = 0) uniform texture2D bloomBlur_texture; 
layout(set = 3, binding = 1) uniform sampler bloomBlur_texture_sampler;

layout(set = 4, binding = 0)uniform float exposure;


void main()
{             
    const float gamma = 2.2;
    vec3 hdrColor = texture(sampler2D(scene_texture, scene_texture_sampler), TexCoords).rgb;      
    vec3 bloomColor = texture(sampler2D(bloomBlur_texture, bloomBlur_texture_sampler), TexCoords).rgb;
    hdrColor += bloomColor; // additive blending
    // tone mapping
    vec3 result = vec3(1.0) - exp(-hdrColor * exposure);
    // also gamma correct while we're at it       
    result = pow(result, vec3(1.0 / gamma));
    FragColor = vec4(result, 1.0);
}  