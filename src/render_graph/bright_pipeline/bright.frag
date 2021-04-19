
// From https://learnopengl.com/Advanced-Lighting/Bloom
#version 450

layout(location = 0)  in vec2 TexCoords;
layout (location = 1) out vec4 FragColor;
layout (location = 0) out vec4 BrightColor;

layout(set = 2, binding = 0) uniform texture2D brightness_texture; 
layout(set = 2, binding = 1) uniform sampler brightness_texture_sampler;

void main()
{   
    vec3 result = texture(sampler2D(brightness_texture, brightness_texture_sampler), TexCoords).rgb; // current fragment's contribution
   
   // check whether fragment output is higher than threshold, if so output as brightness color
    float brightness = dot(result, vec3(0.2126, 0.7152, 0.0722));
    if(brightness > 0.5) // 1.0
        BrightColor = vec4(result, 1.0);
    else
        BrightColor = vec4(0.0, 0.0, 0.0, 1.0);
}
