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
        self.insert(id, government);
        id.id
    }

    fn insert(&mut self, id: Valid<Self>, government: GovernmentRow) {
        self.name.insert(id, government.name);
    }
}

#[derive(Debug, Clone)]
pub struct GovernmentRow {
    pub name: String,
}

impl Create<GovernmentRow> for State {
    type Id = Id<Government>;

    fn create(&mut self, row: GovernmentRow) -> Self::Id {
        self.arenas
            .government
            .create(&mut self.allocators.government, row)
    }
}
