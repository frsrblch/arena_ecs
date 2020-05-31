use super::*;

#[derive(Debug, Default)]
pub struct System {
    pub name: Component<Self, String>,
    pub temperature: Component<Self, f64>,
    pub radius: Component<Self, f64>,
    pub mass: Component<Self, f64>,
}

impl Arena for System {
    type Index = u16;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl System {
    pub fn create(&mut self, allocator: &mut Allocator<Self>, system: SystemRow) -> Id<Self> {
        let id = allocator.create();
        self.insert(id, system);
        id
    }

    fn insert(&mut self, id: Id<Self>, system: SystemRow) {
        self.name.insert(id, system.name);
        self.radius.insert(id, system.radius);
        self.temperature.insert(id, system.temperature);
        self.mass.insert(id, system.mass);
    }
}

#[derive(Debug, Clone)]
pub struct SystemRow {
    pub name: String,
    pub radius: f64,
    pub temperature: f64,
    pub mass: f64,
}

impl State {
    pub fn create_system(&mut self, system: SystemRow) -> Id<System> {
        self.arenas.system.create(&mut self.allocators.system, system)
    }
}