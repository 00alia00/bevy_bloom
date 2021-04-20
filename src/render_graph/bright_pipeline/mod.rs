use bevy_asset::{Assets, Handle, HandleUntyped};
use bevy_reflect::TypeUuid;
use bevy_render::{pipeline::PipelineDescriptor, 
    // render_graph::{Node, ResourceSlotInfo}, 
    renderer::{RenderResources,},// RenderResourceType},
    shader::{Shader, ShaderDefs, ShaderStage, ShaderStages},
    texture::Texture
};
// use std::borrow::Cow;

pub const BRIGHT_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771388); // TODO: get real UUID

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "7de6175d-6acf-4d57-8ee9-ae0a07fa98d5"]
pub struct Brightness {
    pub texture: Handle<Texture>,
}

// impl Brightness{
//     pub const OUT_TEXTURE: &'static str = "texture";
// }

// impl Node for Brightness{

//     fn output(&self) -> &[ResourceSlotInfo] {
//         static OUTPUT: &[ResourceSlotInfo] = &[ResourceSlotInfo {
//             name: Cow::Borrowed(Brightness::OUT_TEXTURE),
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
