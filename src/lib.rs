pub mod render_graph;

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_ecs::prelude::IntoSystem;
use bevy_render::shader;

use render_graph::{add_bloom_graph, Brightness, BlurHorizontal, BlurVertical, Combine};

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
        app.add_asset::<Combine>();

        add_bloom_graph(app.world_mut());
    }
}
