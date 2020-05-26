use super::*;

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub surface: Option<SurfaceRow>,
    pub orbit: PlanetOrbitRow,
    pub moons: Vec<Moon>,
}

impl CreateLinked<Planet> for State {
    type Links = Id<System>;
    type Id = PlanetIds;

    fn create_linked(&mut self, planet: Planet, system: Id<System>) -> PlanetIds {
        let body = self.create_linked(planet.body, system);

        if let Some(surface) = planet.surface {
            let links = SurfaceLinks { body, system };
            let surface = self.create_linked(surface, links);
            self.arenas.body.link_child(body, surface);
        }

        let _orbit = self.create_linked(planet.orbit, body);

        let links = MoonLinks { system, parent: body };
        let moons = planet.moons
            .into_iter()
            .map(|moon| self.create_linked(moon, links))
            .collect();

        PlanetIds { body, moons }
    }
}

#[derive(Debug, Clone)]
pub struct PlanetIds {
    pub body: Id<Body>,
    pub moons: Vec<Id<Body>>,
}
