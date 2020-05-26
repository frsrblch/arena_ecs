use super::*;

#[derive(Debug, Clone)]
pub struct Moon {
    pub body: BodyRow,
    pub surface: SurfaceRow,
    pub orbit: MoonOrbitRow,
}

impl CreateLinked<Moon> for State {
    type Links = MoonLinks;
    type Id = Id<Body>;

    fn create_linked(&mut self, row: Moon, links: MoonLinks) -> Self::Id {
        let MoonLinks { system, parent } = links;

        let body = self.create_linked(row.body, system);

        let surface_links = SurfaceLinks { system, body };
        let surface = self.create_linked(row.surface, surface_links);
        self.arenas.body.link_child(body, surface);

        let _orbit = self.create_linked(row.orbit, MoonOrbitLinks { body, parent });

        body
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MoonLinks {
    pub system: Id<System>,
    pub parent: Id<Body>,
}
