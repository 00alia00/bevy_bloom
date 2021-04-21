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
        TextureNode,
        //WindowTextureNode,
        WindowSwapChainNode,
    },
    shader::Shader,
    //texture::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage},
    texture::{
        Extent3d,
        SamplerDescriptor,
        TextureDescriptor,
        TextureDimension, //TextureFormat,
        TextureUsage,
    },
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

pub const TEXTURE_NODE0: &str = "texure_node0";
pub const TEXTURE_NODE1: &str = "texure_node1";
pub const TEXTURE_NODE2: &str = "texure_node2";

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
            attachment: TextureAttachment::Input(bright_pipeline::Brightness::TEXTURE.to_string()),
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
            attachment: TextureAttachment::Input(
                blur_pipeline::BlurHorizontal::TEXTURE.to_string(),
            ),
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
            attachment: TextureAttachment::Input(blur_pipeline::BlurVertical::TEXTURE.to_string()),
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
                attachment: TextureAttachment::Input(
                    combine_pipeline::Combine::TEXTURE_ORIGINAL.to_string(),
                ),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            },
            RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Input(
                    combine_pipeline::Combine::TEXTURE_BRIGHTBLUR.to_string(),
                ),
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
        bloom::WINDOW_SAMPLED_COLOUR,
        WindowSwapChainNode::new(WindowId::primary()),
    );

    graph.add_node(bloom::BRIGHTNESS_PASS, brightness_node);
    graph.add_node(bloom::BLUR_HORIZONTAL_PASS, horizontal_node);
    graph.add_node(bloom::BLUR_VERTICAL_PASS, vertical_node);

    // Eventually combine the output of the vertical and horizontal blur with the original input pixels
    graph.add_node(bloom::COMBINE_PASS, combine_node);

    graph
        .add_node_edge(bloom::BRIGHTNESS_PASS, bloom::BLUR_HORIZONTAL_PASS)
        .unwrap();
    graph
        .add_node_edge(bloom::BLUR_HORIZONTAL_PASS, bloom::BLUR_VERTICAL_PASS)
        .unwrap();
    graph
        .add_node_edge(bloom::BLUR_VERTICAL_PASS, bloom::COMBINE_PASS)
        .unwrap();
    graph
        .add_node_edge(bloom::WINDOW_SAMPLED_COLOUR, bloom::COMBINE_PASS)
        .unwrap();

    let size = Extent3d::new(512, 512, 1);
    graph.add_node(
        TEXTURE_NODE0,
        TextureNode::new(
            TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: Default::default(),
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            Some(SamplerDescriptor::default()),
            None,
        ),
    );
    graph.add_node(
        TEXTURE_NODE1,
        TextureNode::new(
            TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: Default::default(),
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            Some(SamplerDescriptor::default()),
            None,
        ),
    );
    graph.add_node(
        TEXTURE_NODE2,
        TextureNode::new(
            TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: Default::default(),
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            Some(SamplerDescriptor::default()),
            None,
        ),
    );
    //
    // Links
    //

    graph
        .add_slot_edge(
            bloom::WINDOW_SAMPLED_COLOUR,
            WindowSwapChainNode::OUT_TEXTURE,
            bloom::BRIGHTNESS_PASS,
            bright_pipeline::Brightness::TEXTURE,
        )
        .unwrap();

    // Blur brightness in horizontal and vertical directions

    graph
        .add_node_edge(bloom::BRIGHTNESS_PASS, TEXTURE_NODE0)
        .unwrap();
    graph
        .add_slot_edge(
            bloom::BRIGHTNESS_PASS,
            bright_pipeline::Brightness::TEXTURE,
            TEXTURE_NODE0,
            TextureNode::TEXTURE,
        )
        .unwrap();

    // graph
    //     .add_node_edge(TEXTURE_NODE0, bloom::BLUR_HORIZONTAL_PASS)
    //     .unwrap();
    // graph
    //     .add_slot_edge(
    //         TEXTURE_NODE0,
    //         TextureNode::TEXTURE,
    //         bloom::BLUR_HORIZONTAL_PASS,
    //         blur_pipeline::BlurHorizontal::TEXTURE,
    //     )
    //     .unwrap();

    // graph
    //     .add_slot_edge(
    //         bloom::BLUR_HORIZONTAL_PASS,
    //         "horizontal_blur_texture",
    //         bloom::BLUR_VERTICAL_PASS,
    //         "vertical_blur_texture",
    //     )
    //     .unwrap();

    // Combine output of blur (color_attachment) with original inputs (color_attachment) -> color_attachment0 / color_attachment1

    // graph
    //     .add_slot_edge(
    //         bloom::WINDOW_SAMPLED_COLOUR,
    //         WindowSwapChainNode::OUT_TEXTURE,
    //         bloom::COMBINE_PASS,
    //         combine_pipeline::Combine::TEXTURE_ORIGINAL,
    //     )
    //     .unwrap();

    // graph
    //     .add_slot_edge(
    //         bloom::BLUR_VERTICAL_PASS,
    //         "vertical_blur_texture",
    //         bloom::COMBINE_PASS,
    //         combine_pipeline::Combine::IN_TEXTURE_BRIGHTBLUR,
    //     )
    //     .unwrap();

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
