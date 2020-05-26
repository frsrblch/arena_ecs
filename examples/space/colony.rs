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

        self.insert(id, colony);
        self.link(id, links);
        self.defaults(id);

        id.id
    }

    fn insert(&mut self, id: Valid<Self>, colony: ColonyRow) {
        self.name.insert(id, colony.name);
        self.population.insert(id, colony.population);
    }

    fn link(&mut self, id: Valid<Self>, links: ColonyLinks) {
        self.body.insert(id, links.body);
        self.government.insert(id, links.government);
    }

    fn defaults(&mut self, id: Valid<Self>) {
        self.food.insert(id, Default::default());
    }
}

#[derive(Debug, Clone)]
pub struct ColonyRow {
    pub name: String,
    pub population: f64,
}

impl CreateLinked<ColonyRow> for State {
    type Links = ColonyLinks;
    type Id = Id<Colony>;

    fn create_linked(&mut self, row: ColonyRow, links: Self::Links) -> Self::Id {
        self.arenas.colony.create(&mut self.allocators.colony, row, links)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
    pub government: Id<Government>,
}
