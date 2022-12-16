use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::part1::{scan_rock_paths, Mat, Pos, Scan};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep(Duration::from_millis(250), "animate_sand")
            .add_startup_system(load_scan_map)
            .add_fixed_timestep_system("animate_sand", 0, animate_sand);
    }
}

#[derive(Component)]
struct Position(Pos);

#[derive(Component)]
struct Block(Mat);

#[derive(Component)]
struct FallingParticle;

#[derive(Bundle)]
struct BlockBundle {
    position: Position,
    block: Block,
    #[bundle]
    object: PbrBundle,
}

fn load_scan_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let input = include_str!("../test.txt");
    let scan = scan_rock_paths(input);

    scan.map.iter().for_each(|(pos, mat)| {
        commands.spawn(BlockBundle {
            position: Position(*pos),
            block: Block(*mat),
            object: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                }),
                transform: Transform::from_xyz(pos.x as f32, -(pos.y as f32), 0.0),
                ..default()
            },
        });
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(500.0, -2.0, 50.0),
        point_light: PointLight {
            intensity: 50000.0,
            range: 200.0,
            radius: 0.2,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(500.0, -4.0, 20.0)
            .looking_at(Vec3::new(500.0, -4.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.insert_resource(ScanResource {
        scan,
        current: None,
    });
}

#[derive(Resource)]
struct ScanResource {
    scan: Scan,
    current: Option<Pos>,
}

enum SandState {
    Falling(Pos),
    Stopped,
    Finished,
}

fn next_sand_particle(scan: &mut Scan, pos: Option<Pos>) -> SandState {
    let mut pos = match pos {
        Some(p) => p,
        None => Pos { x: 500, y: 0 },
    };

    if pos.y > scan.max.y {
        return SandState::Finished;
    }

    if scan.get(&pos.to(0, 1)) == Mat::Air {
        pos = pos.to(0, 1);
        SandState::Falling(pos)
    } else if scan.get(&pos.to(-1, 1)) == Mat::Air {
        pos = pos.to(-1, 1);
        SandState::Falling(pos)
    } else if scan.get(&pos.to(1, 1)) == Mat::Air {
        pos = pos.to(1, 1);
        SandState::Falling(pos)
    } else {
        scan.insert(pos, Mat::Sand);
        SandState::Stopped
    }
}

fn animate_sand(
    mut scan: ResMut<ScanResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Transform), With<FallingParticle>>,
) {
    let current_pos = scan.current;
    let falling_particle = query.get_single_mut();

    match next_sand_particle(&mut scan.scan, current_pos) {
        SandState::Falling(pos) => {
            match falling_particle {
                Ok((_, mut transform)) => {
                    transform.translation = Vec3::new(pos.x as f32, -(pos.y as f32), 0.0);
                }
                Err(_) => {
                    commands.spawn((
                        FallingParticle,
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::UVSphere {
                                radius: 0.5,
                                ..default()
                            })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::hex("C2B280").unwrap(),
                                ..default()
                            }),
                            transform: Transform::from_xyz(pos.x as f32, -(pos.y as f32), 0.0),
                            ..default()
                        },
                    ));
                }
            }
            scan.current = Some(pos);
        }
        SandState::Stopped => {
            scan.current = None;
            commands
                .entity(falling_particle.unwrap().0)
                .remove::<FallingParticle>();
        }
        SandState::Finished => {
            scan.current = None;
        }
    }
}
