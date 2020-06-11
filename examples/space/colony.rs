use super::*;

impl State {
    pub fn create_colony(&mut self, colony: ColonyRow, links: ColonyLinks) -> Id<Colony> {
        let id = self.allocators.colony.create();
        self.arenas.colony.insert(id, colony, links);
        id.id
    }
}

#[derive(Debug, Default)]
pub struct Colony {
    pub body: Component<Self, Id<Body>>,
    pub name: Component<Self, String>,
    pub population: Component<Self, f64>,
    pub food_stockpile: Component<Self, f64>,
    pub food_production: Component<Self, f64>,
    pub food_supply_demand: Component<Self, f64>,
    pub government: Component<Self, Id<Government>>,
}

impl Arena for Colony {
    type Index = u16;
    type Generation = NonZeroU16;
    type Allocator = DynamicAllocator<Self>;
}

impl Colony {
    pub fn insert(
        &mut self,
        id: impl Indexes<Self>,
        colony: ColonyRow,
        links: ColonyLinks,
    ) {
        self.name.insert(id, colony.name);
        self.population.insert(id, colony.population);

        self.body.insert(id, links.body);
        self.government.insert(id, links.government);

        self.food_stockpile.insert(id, Default::default());
        self.food_production.insert(id, Default::default());
        self.food_supply_demand.insert(id, Default::default());
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

impl Colony {
    pub fn update_food(&mut self) {
        self.food_stockpile.iter_mut()
            .zip(self.food_supply_demand.iter_mut())
            .zip(self.food_production.iter())
            .zip(self.population.iter())
            .for_each(|(((stocks, supply_demand), production), pop)| {
                *supply_demand = production - 2.0 * pop;
                *stocks -= *supply_demand;
            });
    }
}