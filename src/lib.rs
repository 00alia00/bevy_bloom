pub mod render_graph;

use bevy_app::prelude::*;
use bevy_asset::{
    AddAsset, 
    // Assets, Handle
};
use bevy_ecs::prelude::IntoSystem;
use bevy_render::shader;

use render_graph::{add_bloom_graph, BlurHorizontal, BlurVertical, Brightness};

#[derive(Default)]
pub struct BloomPlugin;

impl Plugin for BloomPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<BlurHorizontal>().add_system_to_stage(
            CoreStage::PostUpdate,
            shader::asset_shader_defs_system::<BlurHorizontal>.system(),
        );

        app.add_asset::<BlurVertical>().add_system_to_stage(
            CoreStage::PostUpdate,
            shader::asset_shader_defs_system::<BlurVertical>.system(),
        );

        app.add_asset::<Brightness>();

        add_bloom_graph(app.world_mut());

        // Not sure i need this at all
        
        // let mut material_hoizonal = app
        //     .world_mut()
        //     .get_resource_mut::<Assets<BlurHorizontal>>()
        //     .unwrap();

        // material_hoizonal.set_untracked(
        //     Handle::<BlurHorizontal>::default(),
        //     BlurHorizontal {
        //         horizontal: true,
        //         ..Default::default()
        //     },
        // );

        // let mut material_vertical = app
        //     .world_mut()
        //     .get_resource_mut::<Assets<BlurVertical>>()
        //     .unwrap();

        // material_vertical.set_untracked(
        //     Handle::<BlurVertical>::default(),
        //     BlurVertical {
        //         horizontal: false,
        //         ..Default::default()
        //     },
        // );
    }
}
