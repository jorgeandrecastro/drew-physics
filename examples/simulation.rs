// Copyright (C) 2026 Jorge Andre Castro
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

//! Exemple d'intégration et de simulation en temps réel avec `drew_physics`.
//!
//! Ce programme illustre la création d'un monde physique 2D contraint (240x320),
//! l'interaction entre plusieurs corps (collisions inter-corps), les rebonds sur
//! les bordures et la suppression dynamique d'un corps en cours de route.
//!
//! Exécution :
//! ```sh
//! cargo run --example demo
//! ```

use drew_physics::{Body, Vec2, World, WorldSettings};

/// Point d'entrée principal du programme d'exemple.
fn main() {
    println!("=== Test de simulation avec drew-physics (v0.2.0) ===");

    // Configuration des paramètres du monde (écran 240x320)
    let settings = WorldSettings {
        gravity: Vec2::new(0.0, 9.81),
        viscosity: 0.02,
        bounds_min: Vec2::ZERO,
        bounds_max: Vec2::new(240.0, 320.0),
    };

    // Instanciation du monde physique (capacité fixe de 4 corps)
    let mut world = World::<4>::new(settings);

    // 1. Ajout d'une première bille dynamique
    let mut bille1 = Body::new(Vec2::new(100.0, 50.0), 1.0, 10.0);
    bille1.velocity = Vec2::new(30.0, 10.0);
    let idx1 = world.add_body(bille1).expect("Monde plein");

    // 2. Ajout d'une seconde bille positionnée sur sa trajectoire pour provoquer une collision
    let mut bille2 = Body::new(Vec2::new(160.0, 55.0), 1.5, 12.0);
    bille2.velocity = Vec2::new(-20.0, 5.0);
    let _idx2 = world.add_body(bille2).expect("Monde plein");

    // Intervalle de temps par frame (~60 images par seconde)
    let dt = 0.016;

    // Boucle de simulation sur 60 pas de temps
    for step in 0..60 {
        world.step(dt);

        // Exemple de suppression dynamique au milieu de la simulation (à la frame 30)
        if step == 30 {
            println!("> Suppression du premier corps via remove_body({idx1})");
            world.remove_body(idx1);
        }

        // Affichage de l'état du premier corps s'il existe encore
        if let Some(body) = world.bodies[idx1] {
            let x_px = body.position.x as i32;
            let y_px = body.position.y as i32;

            if step % 10 == 0 {
                println!(
                    "Frame {:02} | Bille 1 Pos: ({:3}, {:3}) | Vel: ({:6.2}, {:6.2})",
                    step, x_px, y_px, body.velocity.x, body.velocity.y
                );
            }
        } else if step >= 30 && step % 10 == 0 {
            println!("Frame {:02} | Bille 1 est supprimée du monde.", step);
        }
    }
}
