use super::*;

#[derive(Debug, Default)]
pub struct Government {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub capital: Component<Self, Id<Colony>>,

    pub trade: Component<Self, Graph<Colony, f64>>,
}

impl Arena for Government {
    type Allocator = DynamicAllocator<Self>;
}

impl Government {
    pub fn create(&mut self, govt: GovernmentRow) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, govt.name);

        id.value
    }

    pub fn build_trade_graphs(&mut self, colonies: &Colony, bodies: &Body) {
        self.trade.iter_mut().for_each(|g| g.clear());

        let get_colony_govt_body_and_id = || {
            let body_govt = colonies.government.zip(&colonies.body);

            colonies.alloc.zip_id_and_filter(body_govt)
        };

        let iter_pairs = get_colony_govt_body_and_id()
            .enumerate()
            .flat_map(move |(i, t1)| {
                get_colony_govt_body_and_id()
                    .skip(i + 1)
                    .map(move |t2| (t1, t2))
            });

        for ((T(g1, b1), c1), (T(g2, b2), c2)) in iter_pairs {
            if g1 == g2 {
                if let Some(govt) = self.alloc.validate(*g1) {
                    let distance = bodies.get_distance(*b1, *b2);
                    let graph = self.trade.get_mut(govt);
                    graph.insert_ids(c1, c2, distance);
                }
            }
        }
    }
}

impl Body {
    fn get_distance(&self, a: Id<Self>, b: Id<Self>) -> f64 {
        let p1 = self.position.get(a);
        let p2 = self.position.get(b);
        let x = p1.0 - p2.0;
        let y = p1.1 - p2.1;
        (x * x + y * y).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct GovernmentRow {
    pub name: String,
}
