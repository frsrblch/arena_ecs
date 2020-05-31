use super::*;

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub surface: Option<SurfaceRow>,
    pub orbit: PlanetOrbitRow,
    pub moons: Vec<Moon>,
}

impl State {
    pub fn create_planet(&mut self, planet: Planet, system: Id<System>) -> PlanetIds {
        let body = self.allocators.body.create();
        let orbit = self.allocators.planet_orbit.create();

        let surface = planet.surface.map(|surface| {
            let id = self.allocators.surface.create();
            self.arenas.surface.insert(id, surface, SurfaceLinks { body, system });
            id
        });

        self.arenas.body.insert(body, planet.body, BodyLinks { system, orbit: Orbit::Planet(orbit), surface });

        self.arenas.planet_orbit.insert(orbit, planet.orbit, body);

        let moons = planet.moons
            .into_iter()
            .map(|moon| self.create_moon(moon, MoonLinks { system, parent: body }))
            .collect();

        PlanetIds { body, moons }
    }
}

#[derive(Debug, Clone)]
pub struct PlanetIds {
    pub body: Id<Body>,
    pub moons: Vec<Id<Body>>,
}
