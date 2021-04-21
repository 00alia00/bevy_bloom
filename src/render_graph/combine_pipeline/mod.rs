use bevy_asset::{Assets, Handle, HandleUntyped};
use bevy_reflect::TypeUuid;
use bevy_render::{
    pipeline::PipelineDescriptor,
    // render_graph::{Node, ResourceSlotInfo},
    renderer::RenderResources, // RenderResourceType},
    shader::{Shader, ShaderDefs, ShaderStage, ShaderStages},
    texture::Texture,
};
// use std::borrow::Cow;
pub const COMBINE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771386); // TODO: get real UUID

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "a35a6ca6-3f4f-4ebf-88d5-4d2aaeac08cc"]
pub struct Combine {
    pub original_pixels: Handle<Texture>,
    pub bright_and_blur: Handle<Texture>,
}

impl Combine {
    pub const TEXTURE_ORIGINAL: &'static str = "original_pixels";
    pub const TEXTURE_BRIGHTBLUR: &'static str = "bright_and_blur";
    pub const OUT_TEXTURE: &'static str = "texture";
}

// impl Node for Combine{

//     fn output(&self) -> &[ResourceSlotInfo] {
//         static OUTPUT: &[ResourceSlotInfo] = &[ResourceSlotInfo {
//             name: Cow::Borrowed(Combine::OUT_TEXTURE),
//             resource_type: RenderResourceType::Texture,
//         }];
//         OUTPUT
//     }

//     fn update(
//         &mut self,
//         world: &bevy_ecs::prelude::World,
//         render_context: &mut dyn bevy_render::renderer::RenderContext,
//         input: &bevy_render::render_graph::ResourceSlots,
//         output: &mut bevy_render::render_graph::ResourceSlots,
//     ) {
//         todo!()
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
