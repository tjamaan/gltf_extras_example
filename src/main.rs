use bevy::prelude::*;
use bevy::gltf::GltfExtras;
use bevy_inspector_egui::WorldInspectorPlugin;
use serde_json::Value;

/// A Stage that runs between `CoreStage::PreUpdate` and `CoreStage::Update`. Scenes should be added to the World at this point!
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct AssetsLoadedStage;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "GltfExtras example".into(),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::TEAL))
        .add_stage_before(CoreStage::Update, AssetsLoadedStage, SystemStage::parallel())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_level1)
        .add_system_to_stage(AssetsLoadedStage, parse_gltf_extras)
        .add_system(spinny_spin_spin)
        .add_system(scream_and_shout)
        .run();
}

fn setup_level1(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load and spawn scene
    commands.spawn_scene(asset_server.load("level1.glb#Scene0"));

    // setup perspective camera (transform x,y,z copied from blender)
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(7.35889, 6.92579, 4.95831)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    // setup a point light (transform x,y,z copied from blender)
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.07625, -1.00545, 5.90386),
        ..default()
    });
}

fn parse_gltf_extras(
    mut commands: Commands,
    query: Query<
        // query every entity with GltfExtras
        (Entity, &GltfExtras),
        // filter only the newly added GltfExtras
        Added<GltfExtras>,
    >,
) {
    for (entity, gltf_extras) in query.iter() {
        let v: Value = serde_json::from_str(&gltf_extras.value)
            .expect("Couldn't parse GltfExtra value as JSON");

        if let Some(Value::Number(z_rot)) = v.get("z_rot") {
            let z_rot = z_rot.as_f64().expect("Invalid z_rot value");
            commands.entity(entity).insert(ZRot(z_rot as f32));
        }
    }
}

#[derive(Component)]
struct ZRot(f32);

fn spinny_spin_spin(mut query: Query<(&mut Transform, &ZRot)>, time: Res<Time>) {
    for (mut transform, z_rot) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(z_rot.0 * time.delta_seconds()))
    }
}

fn scream_and_shout(query: Query<(Entity), (With<GltfExtras>, Without<ZRot>)>) {
    for entity in query.iter() {
        println!("AAAaaaAAAaaA {:?}", entity);
    }
}
