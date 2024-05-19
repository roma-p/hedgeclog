use bevy::prelude::*;
use core::f32::consts::PI;

use crate::common::common::GridPosition;
use crate::common::asset_loader::HedgehogAssets;

#[derive(Bundle, Default)]
pub struct BundleHedgehog{
    pub model: PbrBundle,
    pub grid_position: GridPosition, 
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub enum EnumHedgehogOnGrid {
    #[default]
    Empty,
    HedgehogAlive,
    HedgehogDeadBurnt,
    HedgehogDeadTooClose,
}

#[derive(Component)]
pub struct MarkerHedgehogOnLevel;

pub struct PluginHedghog;

#[derive(Resource, Debug, Default)]
pub struct ResHedgeHogInfo {
    pub transform_shift: Transform
}

impl Plugin for PluginHedghog{
    fn build(&self, app: &mut App){
        // app.add_systems(Startup, spawn_hedgehog);
        app
            .init_resource::<ResHedgeHogInfo>()
            .add_systems(Startup, init_hedgehog_info);
    }
}

fn init_hedgehog_info(mut r_hedgehog_info: ResMut<ResHedgeHogInfo>) {
    r_hedgehog_info.transform_shift = Transform::IDENTITY
        .mul_transform(Transform::from_rotation(Quat::from_rotation_y(PI/4.0)))   // 45° rotation to face player.
        .mul_transform(Transform::from_scale(Vec3{x:1.5, y:1.5, z:1.5}))          // scaling it to fit tiles
        .mul_transform(Transform::from_translation(Vec3{x:-0.4, y:1.5, z:1.3}));  // positionning it on the tiles
}

fn spawn_hedgehog(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut r_hedgehog: Res<HedgehogAssets>
) {
    // let hedgehog_material = materials.add(r_hedgehog.sprite_idle.clone());
    // commands.spawn(
    //     BundleHedgehog {
    //         model: PbrBundle {
    //             mesh: meshes.add(Mesh::from(Plane3d{normal: Direction3d::X})),
    //             material: hedgehog_material,
    //             ..Default::default()
    //         },
    //     }
    // );
}
