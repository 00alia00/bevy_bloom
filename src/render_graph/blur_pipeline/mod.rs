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

pub const BLUR_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771387); // TODO: get real UUID

//
// TODO: These should be the same struct just with the horizontal field set differently, i havent found a way to do that yet
//

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "17f7cff3-c567-4592-b8dd-9b076fc8f46c"]
pub struct BlurHorizontal {
    pub texture: Handle<Texture>,

    #[render_resources(ignore)]
    #[shader_def]
    pub horizontal: bool,
}

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "34b55606-2f23-4c8e-be62-c23e93319ac7"]
pub struct BlurVertical {
    pub texture: Handle<Texture>,

    #[render_resources(ignore)]
    #[shader_def]
    pub horizontal: bool,
}

impl BlurHorizontal {
    pub const TEXTURE: &'static str = "horizontal_blur_texture";
}

impl BlurVertical {
    pub const TEXTURE: &'static str = "vertical_blur_texture";
}

// const OUT_TEXTURE: &'static str = "texture";

// impl Node for BlurHorizontal{

//     fn output(&self) -> &[ResourceSlotInfo] {
//         static OUTPUT: &[ResourceSlotInfo] = &[ResourceSlotInfo {
//             name: Cow::Borrowed(OUT_TEXTURE),
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

// impl  Node for BlurVertical{

//     fn output(&self) -> &[ResourceSlotInfo] {
//         static OUTPUT: &[ResourceSlotInfo] = &[ResourceSlotInfo {
//             name: Cow::Borrowed(OUT_TEXTURE),
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

pub(crate) fn build_blur_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("blur.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("blur.frag"),
        ))),
    })
}
