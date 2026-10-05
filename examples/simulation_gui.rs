// Copyright (C) 2026 Jorge Andre Castro
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.

use bevy::prelude::*;
use drew_physics::{Body, Vec2, World, WorldSettings};

const MAX_BODIES: usize = 16;

#[derive(Resource)]
struct PhysicsWorld {
    world: World<MAX_BODIES>,
    target_idx: usize,
    step_count: u32,
}

#[derive(Component)]
struct PhysicsBodyMarker(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Drew-Physics Simulation GUI - Essaim".into(),
                resolution: (480, 640).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, physics_simulation_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Caméra 2D standard compatible Bevy 0.19
    commands.spawn(Camera2d);

    let settings = WorldSettings {
        gravity: Vec2::new(0.0, 9.81),
        viscosity: 0.01,
        bounds_min: Vec2::ZERO,
        bounds_max: Vec2::new(240.0, 320.0),
    };

    let mut world = World::<MAX_BODIES>::new(settings);

    let mut target_idx = 0;
    for i in 0..12 {
        let x = 40.0 + (i as f32 % 4.0) * 50.0;
        let y = 30.0 + (i as f32 / 4.0) * 40.0;

        let mut body = Body::new(Vec2::new(x, y), 1.0, 10.0);
        body.velocity = Vec2::new(((i * 17) % 50) as f32 - 25.0, ((i * 31) % 40) as f32 - 10.0);
        body.restitution = 0.85;

        if let Some(idx) = world.add_body(body) {
            if i == 0 {
                target_idx = idx;
            }
        }
    }

    for (i, body_opt) in world.bodies.iter().enumerate() {
        if let Some(body) = body_opt {
            let color = if i == target_idx {
                Color::srgb(0.95, 0.2, 0.2)
            } else {
                Color::srgb(0.1, 0.6, 0.9)
            };

            let translation = world_to_bevy_coords(body.position, 240.0, 320.0);

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(body.radius))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(translation),
                PhysicsBodyMarker(i),
            ));
        }
    }

    commands.insert_resource(PhysicsWorld {
        world,
        target_idx,
        step_count: 0,
    });

    println!("=== Simulation graphique (Essaim) lancée ===");
}

fn physics_simulation_system(
    mut physics: ResMut<PhysicsWorld>,
    mut query: Query<(&mut Transform, &PhysicsBodyMarker, Entity)>,
    mut commands: Commands,
) {
    let dt = 0.016;

    physics.world.step(dt);
    physics.step_count += 1;

    if physics.step_count == 240 {
        let target = physics.target_idx;
        println!("> Suppression dynamique du corps rouge (index {target}) via remove_body");
        physics.world.remove_body(target);

        for (_, marker, entity) in query.iter() {
            if marker.0 == target {
                commands.entity(entity).despawn();
            }
        }
    }

    for (mut transform, marker, _) in query.iter_mut() {
        if let Some(body) = physics.world.bodies[marker.0] {
            transform.translation = world_to_bevy_coords(body.position, 240.0, 320.0);
        }
    }
}

fn world_to_bevy_coords(pos: Vec2, width: f32, height: f32) -> Vec3 {
    let x = pos.x - (width / 2.0);
    let y = (height / 2.0) - pos.y;
    Vec3::new(x, y, 0.0)
}
