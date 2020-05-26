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
    pub position: Component<Self, (f64, f64)>,

    pub surface: IdMap<Self, Surface>,
}

impl Arena for Body {
    type Index = u32;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Body {
    pub fn create(
        &mut self,
        allocator: &mut Allocator<Self>,
        body: BodyRow,
        system: Id<System>,
    ) -> Id<Self> {
        let id = allocator.create();

        self.insert(id, body);
        self.link(id, system);
        self.defaults(id);

        id
    }

    fn insert(&mut self, id: Id<Self>, body: BodyRow) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);
    }

    fn link(&mut self, id: Id<Self>, system: Id<System>) {
        self.system.insert(id, system);
    }

    fn defaults(&mut self, id: Id<Self>) {
        self.position.insert(id, Default::default());
    }
}

impl LinkChild<Surface> for Body {
    fn link_child(&mut self, id: Id<Self>, surface: Id<Surface>) {
        self.surface.insert(id, surface)
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
}

impl CreateLinked<BodyRow> for State {
    type Links = Id<System>;
    type Id = Id<Body>;

    fn create_linked(&mut self, value: BodyRow, system: Id<System>) -> Id<Body> {
        self.arenas.body.create(&mut self.allocators.body, value, system)
    }
}

