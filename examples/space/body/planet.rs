use super::*;

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub moons: Vec<BodyRow>,
}

impl State {
    pub fn create_planet(&mut self, planet: Planet, system: Id<System>) -> PlanetIds {
        let links = BodyLinks {
            system,
            orbit: Orbit::Planet,
        };
        let body = self.body.create(planet.body, links);

        let moons = planet
            .moons
            .into_iter()
            .map(|moon| {
                let links = BodyLinks {
                    system,
                    orbit: Orbit::Moon { parent: body },
                };
                self.body.create(moon, links)
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
