use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::mesh::{self, PrimitiveTopology, Indices};
use noise::{NoiseFn, Simplex, Perlin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marching Squares".into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let simplex = Simplex::new(1);

    let square_size = 20.0;
    let rows = 30;
    let cols = 30;

    // Create a 2D vector of squares
    let mut grid = vec![vec![0.0; cols + 1]; rows + 1];

    for row in 0..=rows {
        for col in 0..=cols {
            let x = col as f32 * square_size - (cols as f32 * square_size) / 2.;
            let y = row as f32 * square_size - (rows as f32 * square_size) / 2.;

            let noise = simplex.get([x as f64 * 0.003, y as f64 * 0.003, 100.0]) as f32 + 0.45;

            grid[row][col] = noise;

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(square_size*0.2).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(noise, noise, noise))),
                transform: Transform::from_translation(Vec3::new(x, y, 0.1)),
                ..default()
            });
        }
    }

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Iterate over 4 grid points at a time
    for row in 0..rows {
        for col in 0..cols {
            // let value = (grid[row][col]*8.0 + grid[row][col + 1]*4.0 + grid[row + 1][col + 1]*2.0 + grid[row + 1][col]*1.0) as u8;
            let value = ((grid[row][col]+0.5).floor()*8.0 + (grid[row][col + 1]+0.5).floor()*4.0 + (grid[row + 1][col + 1]+0.5).floor()*2.0 + (grid[row + 1][col]+0.5).floor()*1.0) as u8;
            
            let left_x = col as f32 * square_size - (cols as f32 * square_size) / 2.;
            let top_y = row as f32 * square_size - (rows as f32 * square_size) / 2.;

            let right_x = left_x + square_size;
            let bottom_y = top_y + square_size;

            match value {
                0 => {}
                1 => {
                    // Top left corner
                    positions.push([left_x, top_y + square_size/2.0, 0.0]);
                    positions.push([left_x, bottom_y, 0.0]);
                    positions.push([right_x - square_size/2.0, bottom_y, 0.0]);

                    for _ in 0..3 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                2 => {
                    // Top right corner
                    positions.push([left_x + square_size/2.0, bottom_y, 0.0]);
                    positions.push([right_x, top_y + square_size/2.0, 0.0]);
                    positions.push([right_x, bottom_y, 0.0]);

                    for _ in 0..3 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                3 => {
                    // Top rectangle
                    positions.push([left_x, top_y + square_size/2.0, 0.0]);
                    positions.push([right_x, top_y + square_size/2.0, 0.0]);
                    positions.push([right_x, bottom_y, 0.0]);
                    positions.push([left_x, bottom_y, 0.0]);

                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                4 => {
                    // Bottom right corner
                    positions.push([right_x, top_y, 0.0]);
                    positions.push([right_x, top_y + square_size/2.0, 0.0]);
                    positions.push([left_x + square_size/2.0, top_y, 0.0]);

                    for _ in 0..3 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                5 => {
                    // Beam from top left to bottom right
                }
                6 => {
                    // Right rectangle
                    positions.push([left_x + square_size/2.0, top_y, 0.0]);
                    positions.push([right_x, top_y, 0.0]);
                    positions.push([right_x, bottom_y, 0.0]);
                    positions.push([left_x + square_size/2.0, bottom_y, 0.0]);

                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                7 => {
                    // The opposite of the bottom left corner, made from 3 triangles
                    positions.push([left_x, bottom_y, 0.0]); // Top left
                    positions.push([right_x, bottom_y, 0.0]); // Top right
                    positions.push([right_x, top_y, 0.0]); // Bottom right
                    positions.push([left_x + square_size/2.0, top_y, 0.0]); // Bottom center
                    positions.push([left_x, top_y + square_size/2.0, 0.0]); // Center left

                    for _ in 0..5 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    // Triangle 1
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    // Triangle 2
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    // Triangle 3
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                8 => {
                    // Bottom left corner
                    positions.push([left_x, top_y, 0.0]);
                    positions.push([left_x + square_size/2.0, top_y, 0.0]);
                    positions.push([left_x, top_y + square_size/2.0, 0.0]);

                    for _ in 0..3 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                9 => {
                    // Left rectangle
                    positions.push([left_x, top_y, 0.0]);
                    positions.push([left_x + square_size/2.0, top_y, 0.0]);
                    positions.push([left_x + square_size/2.0, bottom_y, 0.0]);
                    positions.push([left_x, bottom_y, 0.0]);

                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                10 => {
                    // Beamn from top right to bottom left
                }
                11 => {
                    // The opposite of the bottom right corner, made from 3 triangles
                    positions.push([right_x, bottom_y, 0.0]); // Top right
                    positions.push([left_x, bottom_y, 0.0]); // Top left
                    positions.push([left_x, top_y, 0.0]); // Bottom left
                    positions.push([left_x + square_size/2.0, top_y, 0.0]); // Bottom center
                    positions.push([right_x, top_y + square_size/2.0, 0.0]); // Center right
                    

                    for _ in 0..5 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    // Triangle 1
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    // Triangle 2
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    // Triangle 3
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);                  
                }
                12 => {
                    // Bottom rectangle
                    positions.push([right_x, bottom_y - square_size/2.0, 0.0]);
                    positions.push([left_x, bottom_y - square_size/2.0, 0.0]);
                    positions.push([left_x, top_y, 0.0]);
                    positions.push([right_x, top_y, 0.0]);

                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                13 => {
                    // Opposite of the top left corner, made from 3 triangles
                    positions.push([right_x, top_y, 0.0]); // Bottom right
                    positions.push([left_x, top_y, 0.0]); // Bottom left
                    positions.push([left_x, bottom_y, 0.0]); // Top left
                    positions.push([left_x + square_size/2.0, bottom_y, 0.0]); // Top center
                    positions.push([right_x, top_y + square_size/2.0, 0.0]); // Center right

                    for _ in 0..5 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    // Triangle 1
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    // Triangle 2
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    // Triangle 3
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                14 => {
                    // The opposite of the top left corner, made from 3 triangles
                    positions.push([left_x, top_y, 0.0]); // bottom left
                    positions.push([right_x, top_y, 0.0]); // bottom right
                    positions.push([right_x, bottom_y, 0.0]); // top right
                    positions.push([left_x + square_size/2.0, bottom_y, 0.0]); // top center
                    positions.push([left_x, top_y + square_size/2.0, 0.0]); // Center left

                    for _ in 0..5 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    // Triangle 1
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    // Triangle 2
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    // Triangle 3
                    indices.push((positions.len() - 5) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                15 => {
                    //Square
                    positions.push([left_x, top_y, 0.0]);
                    positions.push([right_x, top_y, 0.0]);
                    positions.push([right_x, bottom_y, 0.0]);
                    positions.push([left_x, bottom_y, 0.0]);

                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                        uvs.push([0.0, 0.0]);
                    }

                    // Triangle1
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 3) as u32);
                    indices.push((positions.len() - 2) as u32);
                    // Triangle2
                    indices.push((positions.len() - 4) as u32);
                    indices.push((positions.len() - 2) as u32);
                    indices.push((positions.len() - 1) as u32);
                }
                _ => {}
            };
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(Color::WHITE.into()),
        ..default()
    });
}