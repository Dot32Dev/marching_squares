use bevy::ui;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::mesh::{self, PrimitiveTopology, Indices};
use bevy::sprite::Mesh2dHandle;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_debug_lines::*;

mod generate_vertices;
use generate_vertices::generate_vertices;

#[derive(Default, Resource)]
struct UiState {
    detail_level: u32,
    z_value: f32,
    lerped: bool,
    animate: bool,
    x_y_scale: f32,
    wireframe: bool,
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
            x_y_scale: 0.007,
            wireframe: false,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marching Squares".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin::default())
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

    let (positions, normals, uvs, indices) = generate_vertices(10.0, 0.0, true, 0.007);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(600.0, 600.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
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
        ui.add(egui::Slider::new(&mut ui_state.x_y_scale, 0.001..=0.04).text("X/Y Scale (Zoom)"));
        // ui.label("Lerped or midpoint");
        ui.checkbox(&mut ui_state.lerped, "Lerped");
        ui.checkbox(&mut ui_state.animate, "Animate");
        ui.checkbox(&mut ui_state.wireframe, "Wireframe");
    });

    if ui_state.animate {
        ui_state.z_value += 0.01;
    }
}

fn marching_squares_system(
    ui_state: Res<UiState>,
    mut marching_squares_meshes: Query<&mut Mesh2dHandle, With<MarchingSquares>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lines: ResMut<DebugLines>,
) {
    // println!("marching_squares_meshes: {:?}", marching_squares_meshes.iter().len());
    if ui_state.is_changed() {
        for mut mesh_handle in marching_squares_meshes.iter_mut() {
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
            let (positions, normals, uvs, indices) = generate_vertices(ui_state.detail_level as f32, ui_state.z_value as f32, ui_state.lerped, ui_state.x_y_scale);
                
            if ui_state.wireframe {
                // Draw debug lines for every triangle (fake wireframe)
                for i in 0..indices.len() / 3 {
                    let i = i * 3;
                    let a = positions[indices[i] as usize];
                    let b = positions[indices[i + 1] as usize];
                    let c = positions[indices[i + 2] as usize];
                    // lines.line_colored(a.extend(0.1), b.extend(0.1), 0.1, Color::RED);
                    // lines.line_colored(b.extend(0.1), c.extend(0.1), 0.1, Color::RED);
                    // lines.line_colored(c.extend(0.1), a.extend(0.1), 0.1, Color::RED);
                    lines.line_colored(Vec3::new(a[0], a[1], 0.1), Vec3::new(b[0], b[1], 0.1), 0.0, Color::BLACK);
                    lines.line_colored(Vec3::new(b[0], b[1], 0.1), Vec3::new(c[0], c[1], 0.1), 0.0, Color::BLACK);
                    lines.line_colored(Vec3::new(c[0], c[1], 0.1), Vec3::new(a[0], a[1], 0.1), 0.0, Color::BLACK);
                }
            }

            let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            new_mesh.set_indices(Some(Indices::U32(indices)));

            *mesh_handle = meshes.add(new_mesh).into();
        }
    }
}