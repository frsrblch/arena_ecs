use super::*;

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub moons: Vec<BodyRow>,
}

impl State {
    pub fn create_planet(&mut self, planet: Planet, system: Id<System>) -> PlanetIds {
        let body = self.allocators.body.create();
        let links = BodyLinks { system, orbit: Orbit::Planet };
        self.arenas.body.insert(body, planet.body, links);

        let moons = planet.moons
            .into_iter()
            .map(|moon| {
                let id = self.allocators.body.create();
                let links = BodyLinks {
                    system,
                    orbit: Orbit::Moon { parent: body },
                };
                self.arenas.body.insert(id, moon, links);
                id
            })
            .collect();

        PlanetIds { body, moons }
    }
}

#[derive(Debug, Clone)]
pub struct PlanetIds {
    pub body: Id<Body>,
    pub moons: Vec<Id<Body>>,
}
