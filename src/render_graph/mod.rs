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
        // AssetRenderResourcesNode, TextureCopyNode,
        PassNode,
        RenderGraph,
        WindowSwapChainNode,
        //WindowTextureNode,
    },
    shader::Shader,
    //texture::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage},
};
use bevy_window::WindowId;

mod blur_pipeline;
mod bright_pipeline;
mod combine_pipeline;

pub use blur_pipeline::*;
pub use bright_pipeline::*;
pub use combine_pipeline::*;

/// the names of bloom graph nodes
pub mod bloom {
    pub const WINDOW_SAMPLED_COLOUR: &str = "bloom_window_sampled_colour";
    pub const BRIGHTNESS_PASS: &str = "bloom_brightness_pass";
    pub const BLUR_HORIZONTAL_PASS: &str = "bloom_horizontal_blur_pass";
    pub const BLUR_VERTICAL_PASS: &str = "bloom_vertical_blur_pass";
    pub const COMBINE_PASS: &str = "bloom_combine_pass";
}

pub mod node {
    pub const PRIMARY_SWAP_CHAIN: &str = "swapchain";
    pub const MAIN_PASS: &str = "main_pass";
    pub const SHARED_BUFFERS: &str = "shared_buffers";
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

    //
    // Nodes
    //

    let brightness_node = PassNode::<&bright_pipeline::Brightness>::new(PassDescriptor {
        color_attachments: vec![RenderPassColorAttachmentDescriptor {
            attachment: TextureAttachment::Input("brightness_input_texture".to_string()),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Load,
                store: true,
            },
        }],
        sample_count: 1,
        depth_stencil_attachment: None,
    });

    let horizontal_node = PassNode::<&blur_pipeline::BlurHorizontal>::new(PassDescriptor {
        color_attachments: vec![RenderPassColorAttachmentDescriptor {
            attachment: TextureAttachment::Input("horizontal_blur_texture".to_string()),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Load,
                store: true,
            },
        }],
        sample_count: 1,
        depth_stencil_attachment: None,
    });

    let vertical_node = PassNode::<&blur_pipeline::BlurVertical>::new(PassDescriptor {
        color_attachments: vec![RenderPassColorAttachmentDescriptor {
            attachment: TextureAttachment::Input("vertical_blur_texture".to_string()),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Load,
                store: true,
            },
        }],
        sample_count: 1,
        depth_stencil_attachment: None,
    });

    let combine_node = PassNode::<&combine_pipeline::Combine>::new(PassDescriptor {
        color_attachments: vec![
            RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Input("original_pixels".to_string()),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            },
            RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Input("bright_and_blur".to_string()),
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

    // get a copy of the current window pixels
    graph.add_node(
        node::PRIMARY_SWAP_CHAIN,
        WindowSwapChainNode::new(WindowId::primary()),
    );

    graph.add_node(bloom::BRIGHTNESS_PASS, brightness_node);
    graph.add_node(bloom::BLUR_HORIZONTAL_PASS, horizontal_node);
    graph.add_node(bloom::BLUR_VERTICAL_PASS, vertical_node);

    // Eventually combine the output of the vertical and horizontal blur with the original input pixels
    graph.add_node(bloom::COMBINE_PASS, combine_node);

    //
    // Links
    //

    graph
        .add_slot_edge(
            node::PRIMARY_SWAP_CHAIN,
            WindowSwapChainNode::OUT_TEXTURE,
            bloom::BRIGHTNESS_PASS,
            "brightness_input_texture",
        )
        .unwrap();

    // Blur brightness in horizontal and vertical directions
    graph
        .add_slot_edge(
            bloom::BRIGHTNESS_PASS,
            "texture",
            bloom::BLUR_HORIZONTAL_PASS,
            "horizontal_blur_texture",
        )
        .unwrap();

    graph
        .add_slot_edge(
            bloom::BLUR_HORIZONTAL_PASS,
            "texture",
            bloom::BLUR_VERTICAL_PASS,
            "vertical_blur_texture",
        )
        .unwrap();

    // Combine output of blur (color_attachment) with original inputs (color_attachment) -> color_attachment0 / color_attachment1

    graph
        .add_slot_edge(
            bloom::WINDOW_SAMPLED_COLOUR,
            WindowSwapChainNode::OUT_TEXTURE,
            bloom::COMBINE_PASS,
            "original_pixels",
        )
        .unwrap();

    graph
        .add_slot_edge(
            bloom::BLUR_VERTICAL_PASS,
            "texture",
            bloom::COMBINE_PASS,
            "bright_and_blur",
        )
        .unwrap();

    // Add pipelines

    let bright_pipeline =
        build_bright_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());

    let blur_pipeline =
        build_blur_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());

    let combine_pipeline =
        build_combine_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());

    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();

    pipelines.set_untracked(BRIGHT_PIPELINE_HANDLE, bright_pipeline);
    pipelines.set_untracked(BLUR_PIPELINE_HANDLE, blur_pipeline);
    pipelines.set_untracked(COMBINE_PIPELINE_HANDLE, combine_pipeline);
}
