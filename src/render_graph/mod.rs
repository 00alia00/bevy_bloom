use bevy_asset::Assets;
use bevy_ecs::{reflect::ReflectComponent, world::World};
use bevy_reflect::Reflect;
use bevy_render::{
    pass::{
        LoadOp, Operations, PassDescriptor, RenderPassColorAttachmentDescriptor, TextureAttachment,
    },
    pipeline::PipelineDescriptor,
    render_graph::{
        // base, SharedBuffersNode,
        AssetRenderResourcesNode, PassNode, RenderGraph, TextureCopyNode,
        WindowSwapChainNode, WindowTextureNode,
    },
    shader::Shader,
    texture::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage},
};
use bevy_window::WindowId;

mod blur_pipeline;
mod bright_pipeline;

pub use blur_pipeline::*;
pub use bright_pipeline::*;

/// the names of blur graph nodes
pub mod blur {
    pub const HORIZONTAL_0: &str = "horizontal_0";
    pub const VERTICAL_0: &str = "vertical_0";
}

/// the names of brightness graph nodes
pub mod brightness {
    pub const BRIGHTNESS: &str = "brightness";
}

pub mod bloom {
    pub const INPUT_TEX: &str = "input_texture";
    pub const BLOOM_MAIN_PASS: &str = "bloom_main_pass";
    pub const MAIN_SAMPLED_COLOR_ATTACHMENT: &str = "main_pass_sampled_color_attachment";
}

/// A component that indicates that an entity should be drawn in the "bloom pass"
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BloomPass;

pub(crate) fn add_bloom_graph(world: &mut World) {
    // WIP WIP WIP

    ////////
    // THIS DOESNT WORK YET
    ////////

    let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();

    graph.add_node(bloom::INPUT_TEX, TextureCopyNode::default());

    graph.add_system_node(
        blur::HORIZONTAL_0,
        AssetRenderResourcesNode::<blur_pipeline::BlurHorizontal>::new(true),
    );

    graph.add_system_node(
        blur::VERTICAL_0,
        AssetRenderResourcesNode::<blur_pipeline::BlurVertical>::new(true),
    );

    graph.add_system_node(
        brightness::BRIGHTNESS,
        AssetRenderResourcesNode::<bright_pipeline::Brightness>::new(true),
    );

    // Eventually combine the output of the vertical and horizontal blur with the original input pixels
    let bloom_combine_node = PassNode::<&BloomPass>::new(PassDescriptor {
        color_attachments: vec![
            RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Input("color_attachment0".to_string()),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            },
            RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Input("color_attachment1".to_string()),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            },
        ],
        sample_count: 1,
        depth_stencil_attachment: None,
    });

    // TODO: figure out how to set the attachments and run a shader for the same node
    graph.add_node(bloom::BLOOM_MAIN_PASS, bloom_combine_node);

    // get a copy of the current window pixels
    graph.add_node(
        bloom::MAIN_SAMPLED_COLOR_ATTACHMENT,
        WindowTextureNode::new(
            WindowId::primary(),
            TextureDescriptor {
                size: Extent3d {
                    depth: 1,
                    width: 1,
                    height: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::default(),
                usage: TextureUsage::OUTPUT_ATTACHMENT,
            },
        ),
    );

    // Link window pixels to the input texture
    graph
        .add_slot_edge(
            bloom::MAIN_SAMPLED_COLOR_ATTACHMENT,
            WindowSwapChainNode::OUT_TEXTURE,
            bloom::INPUT_TEX,
            WindowSwapChainNode::OUT_TEXTURE,//"color_attachment",
         )
        .unwrap();

    // The following assumes color_attachment is the slot that the texture is always in

    // Link brightness to initial texture
    graph
        .add_node_edge(brightness::BRIGHTNESS, bloom::INPUT_TEX)
        .unwrap();

    // Blur brightness in horizontal and vertical directions
    graph
        .add_node_edge(blur::HORIZONTAL_0, brightness::BRIGHTNESS)
        .unwrap();
    graph
        .add_node_edge(blur::VERTICAL_0, blur::HORIZONTAL_0)
        .unwrap();

    // Combine output of blur (color_attachment) with original inputs (color_attachment) -> color_attachment0 / color_attachment1

    graph
        .add_slot_edge(
            bloom::BLOOM_MAIN_PASS,
            "color_attachment0",
            bloom::INPUT_TEX,
            "color_attachment",
        )
        .unwrap();

    graph
        .add_slot_edge(
            bloom::BLOOM_MAIN_PASS,
            "color_attachment1",
            blur::VERTICAL_0,
            "color_attachment",
        )
        .unwrap();

    // Add pipelines

    let bright_pipeline =
        build_bright_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());
    let blur_pipeline =
        build_blur_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());

    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();

    pipelines.set_untracked(BRIGHT_PIPELINE_HANDLE, bright_pipeline);
    pipelines.set_untracked(BLUR_PIPELINE_HANDLE, blur_pipeline);
}
