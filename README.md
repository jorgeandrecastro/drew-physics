# drew-physics

[![Crates.io](https://img.shields.io/crates/v/drew-physics.svg)](https://crates.io/crates/drew-physics)
[![Downloads](https://img.shields.io/crates/d/drew-physics.svg)](https://crates.io/crates/drew-physics)
[![docs.rs](https://docs.rs/drew-physics/badge.svg)](https://docs.rs/drew-physics)
[![License](https://img.shields.io/crates/l/drew-physics.svg)](LICENSE)

Moteur de simulation physique 2D agnostique, déterministe, sans allocation et optimisé `f32`, conçu pour les environnements contraints ou embarqués (`#![no_std]`).

## Les quatre piliers du moteur

| Composant | Rôle | Caractéristiques |
|---|---|---|
| **Stockage statique** | Gestion des corps sans allocation mémoire (`heap`) | Utilise un tableau à taille fixe générique `[Option<Body>; N]` avec ajout/suppression dynamiques |
| **Intégration numérique** | Calcul des trajectoires vitesse et position | Euler semi-implicite pour une meilleure stabilité dynamique |
| **Gestion des collisions** | Détection et résolution des collisions | Rebond sur les bordures et entre les corps (cercle-cercle) par impulsions |
| **Forces environnementales** | Application des contraintes globales | Prise en charge de la gravité et de la viscosité (frottement du milieu) |

## Installation

Ajoutez ceci à votre fichier `Cargo.toml` :

```toml
[dependencies]
drew-physics = "0.2"
```

Aucune dépendance externe (hors calcul mathématique de racine carrée embarqué) : la bibliothèque est compatible avec toutes les cibles `#![no_std]`.

## Utilisation
### Initialisation et boucle de simulation
```rust
use drew_physics::{Body, Vec2, World, WorldSettings};

// Configuration des paramètres environnementaux
let settings = WorldSettings {
    gravity: Vec2::new(0.0, 9.81),
    viscosity: 0.05,
    bounds_min: Vec2::ZERO,
    bounds_max: Vec2::new(800.0, 600.0),
};

// Instanciation d'un monde d'une capacité maximale de 10 corps
let mut world = World::<10>::new(settings);

// Ajout d'un corps dynamique (masse = 1.0, rayon = 5.0)
let mut body = Body::new(Vec2::new(100.0, 100.0), 1.0, 5.0);
body.velocity = Vec2::new(50.0, 0.0);

let body_idx = world.add_body(body).expect("Capacité du monde atteinte");

// Avancement de la simulation d'un pas de temps (dt = 16ms)
world.step(0.016);

// Suppression optionnelle du corps par son indice
world.remove_body(body_idx);
```

## Corps statiques et forces appliquées

Pour créer un corps statique (immobile et insensible à la gravité), passez une masse de 0.0 :
```rust
use drew_physics::{Body, Vec2};

// Corps immobile de rayon 10.0
let static_body = Body::new(Vec2::new(400.0, 300.0), 0.0, 10.0);

// Pour un corps dynamique, vous pouvez aussi appliquer des impulsions ponctuelles
let mut dynamic_body = Body::new(Vec2::new(100.0, 100.0), 2.0, 5.0);
dynamic_body.apply_force(Vec2::new(10.0, -50.0));
```

## Exemple complet

Pour lancer la démonstration en ligne de commande :
```bash
cargo run --example simulation
```
Exemple visuel Bevy version 0.19

```bash
cargo run --example simulation_gui
``


## Historique des versions

    0.2.0 : Ajout des collisions inter-corps (cercle-cercle), de la méthode de suppression dynamique `remove_body`, et correction de l'application de la gravité.
    0.1.0 : Version initiale : support #![no_std], intégration d'Euler semi-implicite, frottement visqueux, limites de boîte et gestion des corps statiques et dynamiques.

## Licence

GPL-2.0-or-later

Copyright (C) 2026 Jorge Andre Castro