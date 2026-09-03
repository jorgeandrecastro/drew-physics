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
//! Ce programme illustre la création d'un monde physique 2D contraint dans un écran
//! de dimensions $240 \times 320$, le lâcher d'une bille avec une vitesse initiale,
//! et l'affichage console de sa trajectoire sur 60 frames (1 seconde à 60 FPS).
//!
//! Exécution :
//! ```sh
//! cargo run --example demo
//! ```

use drew_physics::{Body, Vec2, World, WorldSettings};

/// Point d'entrée principal du programme d'exemple.
fn main() {
    println!("=== Test de simulation avec drew-physics ===");

    // Configuration des paramètres du monde (écran 240x320)
    let settings = WorldSettings {
        gravity: Vec2::new(0.0, 9.81),
        viscosity: 0.05,
        bounds_min: Vec2::ZERO,
        bounds_max: Vec2::new(240.0, 320.0),
    };

    // Instanciation du monde physique sans allocation dynamique (capacité fixe de 4 corps)
    let mut world = World::<4>::new(settings);

    // Initialisation d'une bille dynamique avec vitesse initiale
    let mut bille = Body::new(Vec2::new(120.0, 10.0), 1.0, 8.0);
    bille.velocity = Vec2::new(40.0, 0.0);
    world.add_body(bille);

    // Intervalle de temps par frame (soit ~60 images par seconde)
    let dt = 0.016;

    // Boucle de simulation sur 60 pas de temps
    for step in 0..60 {
        world.step(dt);

        if let Some(body) = world.bodies[0] {
            let x_px = body.position.x as i32;
            let y_px = body.position.y as i32;

            // Affichage toutes les 10 frames pour suivre l'évolution
            if step % 10 == 0 {
                println!(
                    "Frame {:02} | Pos: ({:3}, {:3}) | Vel: ({:6.2}, {:6.2})",
                    step, x_px, y_px, body.velocity.x, body.velocity.y
                );
            }
        }
    }
}