use super::*;

#[derive(Debug, Default)]
pub struct Colony {
    pub alloc: Allocator<Self>,

    pub body: Component<Self, Id<Body>>,
    pub name: Component<Self, String>,
    pub population: Component<Self, f64>,
    pub food_stockpile: Component<Self, f64>,
    pub food_production: Component<Self, f64>,
    pub food_supply_demand: Component<Self, f64>,
    pub government: Component<Self, Id<Government>>,
}

impl Arena for Colony {
    type Allocator = DynamicAllocator<Self>;
}

impl Colony {
    pub fn create(&mut self, colony: ColonyRow, links: ColonyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, colony.name);
        self.population.insert(id, colony.population);

        self.body.insert(id, links.body);
        self.government.insert(id, links.government);

        self.food_stockpile.insert(id, Default::default());
        self.food_production.insert(id, Default::default());
        self.food_supply_demand.insert(id, Default::default());

        id.id()
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
        self.food_stockpile
            .iter_mut()
            .zip(&mut self.food_supply_demand)
            .zip(&self.food_production)
            .zip(&self.population)
            .for_each(|T(T(T(stocks, supply_demand), production), pop)| {
                *supply_demand = production - 2.0 * pop;
                *stocks -= *supply_demand;
            });
    }
}
