use super::*;
pub use planet::*;
pub use moon::*;

mod planet;
mod moon;

#[derive(Debug, Default)]
pub struct Body {
    pub system: Component<Self, Id<System>>,
    pub name: Component<Self, String>,
    pub mass: Component<Self, f64>,
    pub radius: Component<Self, f64>,
    pub orbit: Component<Self, Orbit>,
    pub position: Component<Self, (f64, f64)>,

    pub surface: IdMap<Self, Surface>,
}

impl Arena for Body {
    type Index = u32;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Body {
    pub fn insert(
        &mut self,
        id: Id<Self>,
        body: BodyRow,
        links: BodyLinks,
    ) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);

        self.system.insert(id, links.system);
        self.orbit.insert(id, links.orbit);
        if let Some(surface) = links.surface { self.surface.insert(id, surface); }

        self.position.insert(id, Default::default());
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub system: Id<System>,
    pub orbit: Orbit,
    pub surface: Option<Id<Surface>>,
}

#[derive(Debug, Copy, Clone)]
pub enum Orbit {
    Planet(Id<PlanetOrbit>),
    Moon(Id<MoonOrbit>),
}