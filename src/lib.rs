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

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # Moteur Physique 2D `no_std`
//!
//! Moteur de simulation physique 2D minimaliste, déterministe et conçu pour
//! les environnements contraints ou embarqués sans allocation dynamique (`#![no_std]`).
//!
//! ## Caractéristiques
//!
//! - **Sans allocation (`no_std`)** : Utilise des tableaux à taille fixe génériques `[Option<Body>; N]`.
//! - **Intégration d'Euler semi-implicite** : Garantit une bonne stabilité numérique pour la vélocité et la position.
//! - **Gestion des frontières** : Détection et résolution automatique des collisions avec rebond (`restitution`).
//! - **Viscosité et gravité** : Prise en charge des forces environnementales globales.
//!
//! ## Exemple d'utilisation
//!
//! ```ignore
//! use drew_physics::{Body, Vec2, World, WorldSettings};
//!
//! // Configuration de l'environnement
//! let settings = WorldSettings {
//!     gravity: Vec2::new(0.0, 9.81),
//!     viscosity: 0.1,
//!     bounds_min: Vec2::ZERO,
//!     bounds_max: Vec2::new(800.0, 600.0),
//! };
//!
//! // Instanciation d'un monde capable de contenir jusqu'à 10 corps
//! let mut world = World::<10>::new(settings);
//!
//! // Création d'un corps dynamique (masse = 2.0, rayon = 5.0)
//! let body = Body::new(Vec2::new(100.0, 100.0), 2.0, 5.0);
//! let body_idx = world.add_body(body).expect("Monde plein");
//!
//! // Simulation d'un pas de temps (dt = 16ms)
//! world.step(0.016);
//! ```

use core::ops::{Add, AddAssign, Mul, Sub};

/// Vecteur 2D à composantes en virgule flottante (`f32`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    /// Composante sur l'axe horizontal X.
    pub x: f32,
    /// Composante sur l'axe vertical Y.
    pub y: f32,
}

impl Vec2 {
    /// Vecteur nul `(0.0, 0.0)`.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Crée un nouveau vecteur 2D avec les composantes `x` et `y`.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calcule la norme au carré du vecteur (x² + y²).
    ///
    /// Utile pour comparer des distances sans coût d'extraction de racine carrée (`sqrt`).
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Description d'un corps physique particulaire dans la simulation.
#[derive(Clone, Copy, Debug)]
pub struct Body {
    /// Position actuelle du corps dans l'espace 2D.
    pub position: Vec2,
    /// Vecteur vitesse linéaire.
    pub velocity: Vec2,
    /// Accélération accumulée pour le pas de temps courant.
    pub acceleration: Vec2,
    /// Inverse de la masse (1 / m). Une valeur de `0.0` indique une masse infinie (corps statique/immobile).
    pub inv_mass: f32,
    /// Coefficient de restitution / rebond (compris entre `0.0` pour un choc inélastique et `1.0` pour un choc parfait).
    pub restitution: f32,
    /// Rayon du corps (utilisé pour la détection des collisions avec les limites du monde).
    pub radius: f32,
}

impl Body {
    /// Crée un nouveau corps physique à la position donnée.
    ///
    /// Si `mass <= 0.0`, l'inverse de la masse (`inv_mass`) sera fixé à `0.0`,
    /// rendant le corps statique (insensible à la gravité et aux forces).
    pub fn new(position: Vec2, mass: f32, radius: f32) -> Self {
        let inv_mass = if mass <= 0.0 { 0.0 } else { 1.0 / mass };
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            inv_mass,
            restitution: 0.8,
            radius,
        }
    }

    /// Indique si le corps est statique (masse infinie / `inv_mass == 0.0`).
    pub fn is_static(&self) -> bool {
        self.inv_mass == 0.0
    }

    /// Applique une force extérieure au corps.
    ///
    /// Ne produit aucun effet si le corps est statique (`inv_mass == 0.0`).
    pub fn apply_force(&mut self, force: Vec2) {
        if self.inv_mass > 0.0 {
            self.acceleration += force * self.inv_mass;
        }
    }
}

/// Paramètres globaux de l'environnement physique.
#[derive(Clone, Copy, Debug)]
pub struct WorldSettings {
    /// Vecteur d'accélération de la pesanteur (ex: `Vec2::new(0.0, 9.81)`).
    pub gravity: Vec2,
    /// Coefficient de frottement/viscosité du milieu entraînant une traînée sur les corps en mouvement.
    pub viscosity: f32,
    /// Coin supérieur gauche (coordonnées minimales) de la boîte de confinement.
    pub bounds_min: Vec2,
    /// Coin inférieur droit (coordonnées maximales) de la boîte de confinement.
    pub bounds_max: Vec2,
}

/// Monde physique gérant l'intégration et la simulation globale.
///
/// Stocke un ensemble de corps dans un tableau statique de taille `N`, évitant toute
/// allocation sur le tas (`heap`).
pub struct World<const N: usize> {
    /// Emplacements réservés aux corps physiques du monde.
    pub bodies: [Option<Body>; N],
    /// Configuration globale du monde physique.
    pub settings: WorldSettings,
}

impl<const N: usize> World<N> {
    /// Crée un nouveau monde physique avec les paramètres spécifiés.
    pub fn new(settings: WorldSettings) -> Self {
        Self {
            bodies: [None; N],
            settings,
        }
    }

    /// Renvoie un itérateur mutable sur la liste des corps du monde.
    pub fn bodies_mut(&mut self) -> &mut [Option<Body>; N] {
        &mut self.bodies
    }

    /// Ajoute un corps dans le premier emplacement disponible du monde.
    ///
    /// Renvoie `Some(index)` en cas de succès, ou `None` si la capacité maximale `N` est atteinte.
    pub fn add_body(&mut self, body: Body) -> Option<usize> {
        for (idx, slot) in self.bodies.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(body);
                return Some(idx);
            }
        }
        None
    }

    /// Fait avancer la simulation d'un intervalle de temps `dt` (en secondes).
    ///
    /// Applique la gravité, calcule l'intégration semi-implicite d'Euler, applique
    /// la viscosité du milieu et résout les collisions avec les limites du monde.
    pub fn step(&mut self, dt: f32) {
        for slot in self.bodies.iter_mut() {
            if let Some(body) = slot {
                if body.is_static() {
                    continue; // Corps statique
                }

                // 1. Accumulation initiale avec la gravité
                body.acceleration += self.settings.gravity;

                // 2. Intégration d'Euler semi-implicite
                body.velocity += body.acceleration * dt;

                // 3. Application de la viscosité du milieu
                if self.settings.viscosity > 0.0 {
                    let damping = (1.0 - self.settings.viscosity * dt).max(0.0);
                    body.velocity = body.velocity * damping;
                }

                // 4. Mise à jour de la position
                body.position += body.velocity * dt;

                // 5. Remise à zéro de l'accélération pour la frame suivante
                body.acceleration = Vec2::ZERO;

                // 6. Gestion des collisions avec les bordures
                Self::resolve_boundaries(body, &self.settings);
            }
        }
    }

    /// Détecte et résout les collisions d'un corps avec les bordures du monde.
    fn resolve_boundaries(body: &mut Body, settings: &WorldSettings) {
        let min_x = settings.bounds_min.x + body.radius;
        let max_x = settings.bounds_max.x - body.radius;
        let min_y = settings.bounds_min.y + body.radius;
        let max_y = settings.bounds_max.y - body.radius;

        if body.position.x < min_x {
            body.position.x = min_x;
            body.velocity.x = -body.velocity.x * body.restitution;
        } else if body.position.x > max_x {
            body.position.x = max_x;
            body.velocity.x = -body.velocity.x * body.restitution;
        }

        if body.position.y < min_y {
            body.position.y = min_y;
            body.velocity.y = -body.velocity.y * body.restitution;
        } else if body.position.y > max_y {
            body.position.y = max_y;
            body.velocity.y = -body.velocity.y * body.restitution;
        }
    }
}