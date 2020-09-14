use super::*;

#[derive(Debug, Default)]
pub struct Government {
    pub name: Component<Self, String>,
}

impl Arena for Government {
    type Allocator = DynamicAllocator<Self>;
}

impl Government {
    pub fn insert(&mut self, id: impl Indexes<Self>, government: GovernmentRow) {
        self.name.insert(id, government.name);
    }
}

#[derive(Debug, Clone)]
pub struct GovernmentRow {
    pub name: String,
}

impl State {
    pub fn create_government(&mut self, row: GovernmentRow) -> Id<Government> {
        let id = self.allocators.government.create();
        self.arenas.government.insert(id, row);
        id.id
    }
}