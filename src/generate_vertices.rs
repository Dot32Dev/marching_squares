use noise::{NoiseFn, Simplex, Perlin};

pub fn generate_vertices(
	square_size: f32,
	z_value: f32,
	lerped: bool,
) -> (std::vec::Vec<[f32; 3]>, std::vec::Vec<[f32; 3]>, std::vec::Vec<[f32; 2]>, std::vec::Vec<u32>) {
	let simplex = Simplex::new(1);

    let rows = 600/square_size as usize;
    let cols = 600/square_size as usize;

    // Create a 2D vector of squares
    let mut grid = vec![vec![0.0; cols + 1]; rows + 1];

    for row in 0..=rows {
        for col in 0..=cols {
            let x = col as f32 * square_size - (cols as f32 * square_size) / 2.;
            let y = row as f32 * square_size - (rows as f32 * square_size) / 2.;

            let noise = simplex.get([x as f64 * 0.007, y as f64 * 0.007, z_value as f64]) as f32 + 0.5;

            grid[row][col] = noise;

            // commands.spawn(MaterialMesh2dBundle {
            //     mesh: meshes.add(shape::Circle::new(square_size*0.1).into()).into(),
            //     material: materials.add(ColorMaterial::from(Color::rgb(noise, noise, noise))),
            //     transform: Transform::from_translation(Vec3::new(x, y, 0.1)),
            //     ..default()
            // });
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
            // generate_tile(
            //     grid: &Vec<Vec<f32>>,
            //     row: usize,
            //     col: usize,
            //     square_size: f32,
            //     rows: usize,
            //     cols: usize,
            //     positions: &mut Vec<[f32; 3]>,
            //     normals: &mut Vec<[f32; 3]>,
            //     uvs: &mut Vec<[f32; 2]>,
            //     indices: &mut Vec<u32>,	
            //     detail_level: usize,
            //     z_value: f32,
            //     lerped: bool,
            // )
            let value = ((grid[row][col]+0.5).floor()*8.0 + (grid[row][col + 1]+0.5).floor()*4.0 + (grid[row + 1][col + 1]+0.5).floor()*2.0 + (grid[row + 1][col]+0.5).floor()*1.0) as u8;
	
			let left_x = col as f32 * square_size - (cols as f32 * square_size) / 2.;
			let top_y = row as f32 * square_size - (rows as f32 * square_size) / 2.;

			let right_x = left_x + square_size;
			let bottom_y = top_y + square_size;

			let mut top = inverse_lerp(grid[row][col], grid[row][col + 1], 0.5);
			let mut right = inverse_lerp(grid[row][col + 1], grid[row + 1][col + 1], 0.5);
			let mut bottom = inverse_lerp(grid[row + 1][col], grid[row + 1][col + 1], 0.5);
			let mut left = inverse_lerp(grid[row][col], grid[row + 1][col], 0.5);

			if !lerped {
				// If not lerped, the top, right, bottom, and left values are set to 0.5
				top = 0.5;
				right = 0.5;
				bottom = 0.5;
				left = 0.5;
			}

			match value {
				0 => {}
				1 => {
					// Top left corner
					positions.push([left_x, top_y + square_size*left, 0.0]);
					positions.push([left_x, bottom_y, 0.0]);
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]);

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
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]);
					positions.push([right_x, top_y + square_size*right, 0.0]);
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
					positions.push([left_x, top_y + square_size*left, 0.0]);
					positions.push([right_x, top_y + square_size*right, 0.0]);
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
					positions.push([right_x, top_y + square_size*right, 0.0]);
					positions.push([left_x + square_size*top, top_y, 0.0]);

					for _ in 0..3 {
						normals.push([0.0, 0.0, 1.0]);
						uvs.push([0.0, 0.0]);
					}

					indices.push((positions.len() - 3) as u32);
					indices.push((positions.len() - 2) as u32);
					indices.push((positions.len() - 1) as u32);
				}
				5 => {
					// Top left AND bottom right corners
					// Top left corner
					positions.push([left_x, top_y + square_size*left, 0.0]);
					positions.push([left_x, bottom_y, 0.0]);
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]);
					// Bottom right corner
					positions.push([right_x, top_y, 0.0]);
					positions.push([right_x, top_y + square_size*right, 0.0]);
					positions.push([left_x + square_size*top, top_y, 0.0]);

					for _ in 0..6 {
						normals.push([0.0, 0.0, 1.0]);
						uvs.push([0.0, 0.0]);
					}

					// Top left corner
					indices.push((positions.len() - 6) as u32);
					indices.push((positions.len() - 5) as u32);
					indices.push((positions.len() - 4) as u32);
					// Bottom right corner
					indices.push((positions.len() - 3) as u32);
					indices.push((positions.len() - 2) as u32);
					indices.push((positions.len() - 1) as u32);
				}
				6 => {
					// Right rectangle
					positions.push([left_x + square_size*top, top_y, 0.0]);
					positions.push([right_x, top_y, 0.0]);
					positions.push([right_x, bottom_y, 0.0]);
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]);

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
					positions.push([left_x + square_size*top, top_y, 0.0]); // Bottom center
					positions.push([left_x, top_y + square_size*left, 0.0]); // Center left

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
					positions.push([left_x + square_size*top, top_y, 0.0]);
					positions.push([left_x, top_y + square_size*left, 0.0]);

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
					positions.push([left_x + square_size*top, top_y, 0.0]);
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]);
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
					// Top right AND bottom left corners
					// Top right corner
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]);
					positions.push([right_x, top_y + square_size*right, 0.0]);
					positions.push([right_x, bottom_y, 0.0]);
					// Bottom left corner
					positions.push([left_x, top_y, 0.0]);
					positions.push([left_x + square_size*top, top_y, 0.0]);
					positions.push([left_x, top_y + square_size*left, 0.0]);
					
					for _ in 0..6 {
						normals.push([0.0, 0.0, 1.0]);
						uvs.push([0.0, 0.0]);
					}

					// Top right corner
					indices.push((positions.len() - 6) as u32);
					indices.push((positions.len() - 5) as u32);
					indices.push((positions.len() - 4) as u32);
					// Bottom left corner
					indices.push((positions.len() - 3) as u32);
					indices.push((positions.len() - 2) as u32);
					indices.push((positions.len() - 1) as u32);
				}
				11 => {
					// The opposite of the bottom right corner, made from 3 triangles
					positions.push([right_x, bottom_y, 0.0]); // Top right
					positions.push([left_x, bottom_y, 0.0]); // Top left
					positions.push([left_x, top_y, 0.0]); // Bottom left
					positions.push([left_x + square_size*top, top_y, 0.0]); // Bottom center
					positions.push([right_x, top_y + square_size*right, 0.0]); // Center right
					

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
					positions.push([right_x, top_y + square_size*right, 0.0]); // right
					positions.push([left_x, top_y + square_size*left, 0.0]); // left
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
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]); // Top center
					positions.push([right_x, top_y + square_size*right, 0.0]); // Center right

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
					positions.push([left_x + square_size*bottom, bottom_y, 0.0]); // top center
					positions.push([left_x, top_y + square_size*left, 0.0]); // Center left

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

	return (positions, normals, uvs, indices);
}

fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if a == b {
        return 0.0;
    }

    return (value - a) / (b - a);
}