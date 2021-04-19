
// From https://learnopengl.com/Advanced-Lighting/Bloom
#version 450
// #define Blur_HORIZONTAL

layout(location = 0)  in vec2 TexCoords;
layout(location = 0) out vec4 FragColor;

layout(set = 2, binding = 0) uniform texture2D blur_texture; 
layout(set = 2, binding = 1) uniform sampler blur_texture_sampler;

void main()
{   
    const float weight[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);
          
    vec2 tex_offset = 1.0 / vec2(31.0,32.0);//; textureSize(blur_texture, 0);//// gets size of single texel
    
    vec3 result = texture(sampler2D(blur_texture, blur_texture_sampler), TexCoords).rgb*weight[0]; // current fragment's contribution

#ifdef BLUR_HORIZONTAL
    for(int i = 1; i < 5; ++i)
    {
        result += texture(sampler2D(blur_texture, blur_texture_sampler), TexCoords + vec2(tex_offset.x * i, 0.0)).rgb * weight[i];
        result += texture(sampler2D(blur_texture, blur_texture_sampler), TexCoords - vec2(tex_offset.x * i, 0.0)).rgb * weight[i];
    }
#else
    for(int i = 1; i < 5; ++i)
    {
        result += texture(sampler2D(blur_texture, blur_texture_sampler), TexCoords + vec2(0.0, tex_offset.y * i)).rgb * weight[i];
        result += texture(sampler2D(blur_texture, blur_texture_sampler), TexCoords - vec2(0.0, tex_offset.y * i)).rgb * weight[i];
    }
#endif
    
    FragColor = vec4(result, 1.0);
}
