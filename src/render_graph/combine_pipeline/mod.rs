use bevy_asset::{Assets, Handle, HandleUntyped};
use bevy_reflect::TypeUuid;
use bevy_render::{
    pipeline::PipelineDescriptor,
    renderer::RenderResources,
    shader::{Shader, ShaderDefs, ShaderStage, ShaderStages},
    texture::Texture,
};

pub const COMBINE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771386); // TODO: get real UUID

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "a35a6ca6-3f4f-4ebf-88d5-4d2aaeac08cc"]
pub struct Combine {
    pub original_pixels: Handle<Texture>,
    pub bright_and_blur: Handle<Texture>,
}

// // TODO: how do we define outputs?
// impl Combine{
//     fn output(&self) -> &[ResourceSlotInfo] {
//         &[]
//     }
// }

pub(crate) fn build_combine_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("combine.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("combine.frag"),
        ))),
    })
}
