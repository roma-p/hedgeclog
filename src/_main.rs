use bevy::prelude::*;
use bevy::gltf::Gltf;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_state::<GameState>()
        .add_systems(Startup, (load_ressources, build_level))
        // .add_systems(OnEnter(GameState::Playing), load_ressources)
        // .add_systems(OnEnter(GameState::Playing), build_level)
        .run();
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Component)]
pub struct GridPosition {
    pub i: usize,
    pub j: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource, Default)]
struct Game {}

#[derive(Resource)]
struct TilePack(Handle<Gltf>);

fn load_ressources(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut game: ResMut<Game>,
) {
    let gltf = asset_server.load("tiles.gltf");
    commands.insert_resource(TilePack(gltf));
        // spawn the scene named "YellowCar"
    // commands.spawn(SceneBundle {
    //     scene: gltf.named_meshes["tile_floor"].clone(),
    //     transform: Transform::from_xyz(1.0, 2.0, 3.0),
    //     ..Default::default()
    // });

}

fn build_level(
    mut commands: Commands,
    tile_pack: Res<TilePack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&tile_pack.0) {
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            // scene: gltf.named_scenes["tile_floor"].clone(),
            // let carwheel = assets_gltfmesh.get(&gltf.named_meshes["CarWheel"]).unwrap();
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
    }
}
