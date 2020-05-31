use super::*;

#[derive(Debug, Default)]
pub struct Government {
    pub name: Component<Self, String>,
}

impl Arena for Government {
    type Index = u8;
    type Generation = NonZeroU8;
    type Allocator = DynamicAllocator<Self>;
}

impl Government {
    pub fn create(
        &mut self,
        allocator: &mut Allocator<Self>,
        government: GovernmentRow,
    ) -> Id<Self> {
        let id = allocator.create();

        self.name.insert(id, government.name);

        id.id
    }
}

#[derive(Debug, Clone)]
pub struct GovernmentRow {
    pub name: String,
}

impl State {
    pub fn create_government(&mut self, row: GovernmentRow) -> Id<Government> {
        self.arenas.government.create(&mut self.allocators.government, row)
    }
}