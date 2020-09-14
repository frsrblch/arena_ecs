use super::*;

#[derive(Debug, Default)]
pub struct System {
    pub name: Component<Self, String>,
    pub temperature: Component<Self, f64>,
    pub radius: Component<Self, f64>,
    pub mass: Component<Self, f64>,
}

impl Arena for System {
    type Allocator = FixedAllocator<Self>;
}

impl System {
    pub fn insert(&mut self, id: Id<Self>, system: SystemRow) {
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
        let id = self.allocators.system.create();
        self.arenas.system.insert(id, system);
        id
    }
}