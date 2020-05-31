use super::*;

#[derive(Debug, Clone)]
pub struct Moon {
    pub body: BodyRow,
    pub surface: SurfaceRow,
    pub orbit: MoonOrbitRow,
}

impl State {
    pub fn create_moon(&mut self, row: Moon, links: MoonLinks) -> Id<Body> {
        let MoonLinks { system, parent } = links;

        let body = self.allocators.body.create();
        let surface = self.allocators.surface.create();
        let orbit = self.allocators.moon_orbit.create();

        self.arenas.body.insert(body, row.body, BodyLinks { system, orbit: Orbit::Moon(orbit), surface: Some(surface)});
        self.arenas.surface.insert(surface, row.surface, SurfaceLinks { system, body });
        self.arenas.moon_orbit.insert(orbit, row.orbit, MoonOrbitLinks { body, parent });

        body
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MoonLinks {
    pub system: Id<System>,
    pub parent: Id<Body>,
}