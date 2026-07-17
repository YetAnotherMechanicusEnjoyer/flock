pub mod components;
pub mod crt;

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use crt::CrtMaterial;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CrtMaterial>::default())
            .add_systems(Startup, crt::setup_crt_pipeline);
    }
}
