use super::*;

#[derive(Debug, Default)]
pub struct Surface {
    pub body: Component<Self, Id<Body>>,
    pub system: Component<Self, Id<System>>,
    pub area: Component<Self, f64>,
    pub temperature: Component<Self, f64>,
    pub albedo: Component<Self, f64>,
}

impl Arena for Surface {
    type Index = u32;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Surface {
    pub fn create(
        &mut self,
        allocator: &mut Allocator<Self>,
        surface: SurfaceRow,
        links: SurfaceLinks,
    ) -> Id<Surface> {
        let id = allocator.create();

        self.insert(id, surface);
        self.link(id, links);
        self.defaults(id);

        id
    }

    fn insert(&mut self, id: Id<Self>, surface: SurfaceRow) {
        self.area.insert(id, surface.area);
        self.albedo.insert(id, surface.albedo);
    }

    fn link(&mut self, id: Id<Self>, links: SurfaceLinks) {
        self.system.insert(id, links.system);
        self.body.insert(id, links.body);
    }

    fn defaults(&mut self, id: Id<Self>) {
        self.temperature.insert(id, Default::default());
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceRow {
    pub area: f64,
    pub albedo: f64,
}

impl CreateLinked<SurfaceRow> for State {
    type Links = SurfaceLinks;
    type Id = Id<Surface>;

    fn create_linked(&mut self, row: SurfaceRow, links: Self::Links) -> Id<Surface> {
        self.arenas.surface.create(&mut self.allocators.surface, row, links)
    }
}

#[derive(Debug)]
pub struct SurfaceLinks {
    pub body: Id<Body>,
    pub system: Id<System>,
}
