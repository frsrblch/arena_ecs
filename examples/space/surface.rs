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
    pub fn insert(
        &mut self,
        id: Id<Self>,
        surface: SurfaceRow,
        links: SurfaceLinks,
    ) {
        self.area.insert(id, surface.area);
        self.albedo.insert(id, surface.albedo);

        self.system.insert(id, links.system);
        self.body.insert(id, links.body);

        self.temperature.insert(id, Default::default());
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceRow {
    pub area: f64,
    pub albedo: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct SurfaceLinks {
    pub body: Id<Body>,
    pub system: Id<System>,
}
