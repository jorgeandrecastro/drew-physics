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

#[cfg(test)]
mod tests {
    use drew_physics::{Body, Vec2, World, WorldSettings};

    fn setup_default_world() -> World<4> {
        let settings = WorldSettings {
            gravity: Vec2::new(0.0, 10.0),
            viscosity: 0.0,
            bounds_min: Vec2::new(0.0, 0.0),
            bounds_max: Vec2::new(100.0, 100.0),
        };
        World::new(settings)
    }

    #[test]
    fn test_gravity_acceleration() {
        let mut world = setup_default_world();
        let body = Body::new(Vec2::new(50.0, 0.0), 1.0, 5.0);
        world.add_body(body);

        let dt = 1.0;
        world.step(dt);

        let updated_body = world.bodies[0].unwrap();
        assert_eq!(updated_body.velocity.y, 10.0);
        assert_eq!(updated_body.position.y, 10.0);
    }

    #[test]
    fn test_viscosity_damping() {
        let settings = WorldSettings {
            gravity: Vec2::ZERO,
            viscosity: 0.5,
            bounds_min: Vec2::new(0.0, 0.0),
            bounds_max: Vec2::new(1000.0, 1000.0),
        };
        let mut world = World::<1>::new(settings);
        let mut body = Body::new(Vec2::ZERO, 1.0, 5.0);
        body.velocity = Vec2::new(100.0, 0.0);
        world.add_body(body);

        world.step(1.0);

        let updated_body = world.bodies[0].unwrap();
        assert_eq!(updated_body.velocity.x, 50.0);
    }

    #[test]
    fn test_boundary_bounce() {
        let settings = WorldSettings {
            gravity: Vec2::ZERO, // Désactivation de la gravité pour isoler le rebond X
            viscosity: 0.0,
            bounds_min: Vec2::new(0.0, 0.0),
            bounds_max: Vec2::new(100.0, 100.0),
        };
        let mut world = World::<1>::new(settings);
        let mut body = Body::new(Vec2::new(90.0, 50.0), 1.0, 10.0);
        body.restitution = 0.8;
        body.velocity = Vec2::new(50.0, 0.0);
        world.add_body(body);

        world.step(1.0);

        let updated_body = world.bodies[0].unwrap();
        assert_eq!(updated_body.position.x, 90.0);
        assert_eq!(updated_body.velocity.x, -40.0);
    }

    #[test]
    fn test_static_body() {
        let mut world = setup_default_world();
        let body = Body::new(Vec2::new(50.0, 50.0), 0.0, 5.0);
        world.add_body(body);

        world.step(1.0);

        let updated_body = world.bodies[0].unwrap();
        assert_eq!(updated_body.position, Vec2::new(50.0, 50.0));
        assert_eq!(updated_body.velocity, Vec2::ZERO);
    }
}