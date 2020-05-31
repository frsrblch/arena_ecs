use super::*;

#[derive(Debug, Default)]
pub struct Colony {
    pub body: Component<Self, Id<Body>>,
    pub name: Component<Self, String>,
    pub population: Component<Self, f64>,
    pub food: Component<Self, f64>,
    pub government: Component<Self, Id<Government>>,
}

impl Arena for Colony {
    type Index = u16;
    type Generation = NonZeroU16;
    type Allocator = DynamicAllocator<Self>;
}

impl Colony {
    pub fn create(
        &mut self,
        allocator: &mut Allocator<Self>,
        colony: ColonyRow,
        links: ColonyLinks,
    ) -> Id<Self> {
        let id = allocator.create();

        self.name.insert(id, colony.name);
        self.population.insert(id, colony.population);
        self.body.insert(id, links.body);
        self.government.insert(id, links.government);
        self.food.insert(id, Default::default());

        id.id
    }
}

#[derive(Debug, Clone)]
pub struct ColonyRow {
    pub name: String,
    pub population: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
    pub government: Id<Government>,
}

impl State {
    pub fn create_colony(&mut self, colony: ColonyRow, links: ColonyLinks) -> Id<Colony> {
        self.arenas.colony.create(&mut self.allocators.colony, colony, links)
    }
}