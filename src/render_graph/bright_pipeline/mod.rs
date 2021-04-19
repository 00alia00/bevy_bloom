use bevy_asset::{Assets, Handle, HandleUntyped};
use bevy_reflect::TypeUuid;
use bevy_render::{
    pipeline::PipelineDescriptor,
    renderer::RenderResources,
    shader::{Shader, ShaderDefs, ShaderStage, ShaderStages},
    texture::Texture,
};

pub const BRIGHT_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771388); // TODO: get real UUID

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "7de6175d-6acf-4d57-8ee9-ae0a07fa98d5"]
pub struct Brightness {
    pub texture: Handle<Texture>,
}

pub(crate) fn build_bright_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("bright.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("bright.frag"),
        ))),
    })
}
