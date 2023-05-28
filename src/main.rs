use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::mesh::{self, PrimitiveTopology, Indices};
use bevy::sprite::Mesh2dHandle;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod generate_tiles;
use generate_tiles::generate_tiles;

#[derive(Default, Resource)]
struct UiState {
    detail_level: u32,
    z_value: f32,
    lerped: bool,
    animate: bool,
}

#[derive(Component)]
struct MarchingSquares;

fn main() {
    App::new()
        // .init_resource::<UiState>()
        .insert_resource(UiState {
            detail_level: 10,
            z_value: 0.0,
            lerped: true,
            animate: false,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marching Squares".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(ui_example_system)
        .add_system(marching_squares_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let (positions, normals, uvs, indices) = generate_tiles(10.0, 0.0, true);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::WHITE.into()),
            ..default()
        }, 
        MarchingSquares,
    ));
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    time: Res<Time>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {}", (1.0 / Time::delta_seconds(&time)).floor()));
        ui.add(egui::Slider::new(&mut ui_state.detail_level, 3..=50).text("Grid Size (Lower is better)"));
        // ui.label("Z Value");
        ui.add(egui::Slider::new(&mut ui_state.z_value, 0.0..=100.0).text("Z Coordinate"));
        // ui.label("Lerped or midpoint");
        ui.checkbox(&mut ui_state.lerped, "Lerped");
        ui.checkbox(&mut ui_state.animate, "Animate");
    });

    if ui_state.animate {
        ui_state.z_value += 0.01;
    }
}

fn marching_squares_system(
    ui_state: Res<UiState>,
    mut marching_squares_meshes: Query<&mut Mesh2dHandle, With<MarchingSquares>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // println!("marching_squares_meshes: {:?}", marching_squares_meshes.iter().len());
    if ui_state.is_changed() {
        for mut mesh_handle in marching_squares_meshes.iter_mut() {
            println!("Updating mesh");
            // let mut mesh = meshes.get_mut(mesh_handle);
            // if mesh.is_some() {
            //     let (positions, normals, uvs, indices) = generate_tiles(ui_state.detail_level as f32, ui_state.z_value as f32, ui_state.lerped);
                
            //     let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            //     new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            //     new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            //     new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            //     new_mesh.set_indices(Some(Indices::U32(indices)));

            //     mesh = Some(&mut new_mesh);

            //     println!("Updated mesh")
            // }
            let (positions, normals, uvs, indices) = generate_tiles(ui_state.detail_level as f32, ui_state.z_value as f32, ui_state.lerped);
                
            let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            new_mesh.set_indices(Some(Indices::U32(indices)));

            *mesh_handle = meshes.add(new_mesh).into();
        }
    }
}