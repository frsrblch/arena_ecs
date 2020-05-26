use arena_ecs::*;

fn main() {
    let mut world = World::default();

    let sol = SystemRow {
        name: "Sol".to_string(),
        radius: 696340e3,
        temperature: 5778.0,
        mass: 1.989e30,
    };

    let sol = world.create(sol);

    let earth = Planet {
        body: BodyRow {
            name: "Earth".to_string(),
            mass: 5.972e24,
            radius: 6371e3,
        },
        surface: Some(SurfaceRow {
            area: 510.1e6,
            albedo: 0.3,
        }),
    };

    let earth = world.create_linked(earth, sol);

    let usa = GovernmentRow {
        name: "United States of America".to_string(),
    };

    let usa_govt = world.create(usa);

    let links = ColonyLinks {
        body: earth.body,
        government: usa_govt,
    };

    let usa = ColonyRow {
        name: "America".to_string(),
        population: 376e6,
    };

    let _usa = world.create_linked(usa, links);

    let china = GovernmentRow {
        name: "People's Republic of China".to_string(),
    };

    let china_govt = world.create(china);

    let china = ColonyRow {
        name: "China".to_string(),
        population: 1.657e9,
    };

    let links = ColonyLinks {
        body: earth.body,
        government: china_govt,
    };

    let _china = world.create_linked(china, links);

    world.print_with_government();
}

#[derive(Debug, Default)]
pub struct World {
    pub state: State,
    pub allocators: Allocators,
}

impl World {
    pub fn print_with_government(&self) {
        self.state.colony.name.iter()
            .zip(self.state.colony.population.iter())
            .zip(self.state.colony.government.iter())
            .zip(self.allocators.colony.living())
            .for_each(|(((colony, pop), govt_id), living)| {
                if living {
                    if let Some(govt_id) = self.allocators.government.validate(govt_id) {
                        let govt = self.state.government.name.get(govt_id);
                        println!("{} ({}): {}", colony, govt, pop);
                    }
                }
            });
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub system: System,
    pub body: Body,
    pub surface: Surface,
    pub colony: Colony,
    pub government: Government,
}

#[derive(Debug, Default)]
pub struct Allocators {
    pub system: Allocator<System>,
    pub body: Allocator<Body>,
    pub surface: Allocator<Surface>,
    pub colony: Allocator<Colony>,
    pub government: Allocator<Government>,
}

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

impl Create<SystemRow> for World {
    type Id = Id<System>;

    fn create(&mut self, value: SystemRow) -> Id<System> {
        self.state.system.create(&mut self.allocators.system, value)
    }
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub surface: Option<SurfaceRow>,
}

impl CreateLinked<Planet> for World {
    type Links = Id<System>;
    type Id = PlanetIds;

    fn create_linked(&mut self, planet: Planet, system: Self::Links) -> PlanetIds {
        let body = self.create_linked(planet.body, system);

        let surface = planet.surface.map(|surface| {
            let links = SurfaceLinks { body, system };
            let surface = self.create_linked(surface, links);
            self.state.body.link_child(body, surface);
            surface
        });

        PlanetIds { body, surface }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PlanetIds {
    pub body: Id<Body>,
    pub surface: Option<Id<Surface>>,
}

#[derive(Debug, Default)]
pub struct Body {
    pub system: Component<Self, Id<System>>,
    pub name: Component<Self, String>,
    pub mass: Component<Self, f64>,
    pub radius: Component<Self, f64>,
    pub position: Component<Self, (f64, f64)>,

    pub surface: IdMap<Self, Surface>,
}

impl Arena for Body {
    type Index = u32;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Body {
    pub fn create(
        &mut self,
        allocator: &mut Allocator<Self>,
        body: BodyRow,
        system: Id<System>,
    ) -> Id<Self> {
        let id = allocator.create();

        self.insert(id, body);
        self.link(id, system);
        self.defaults(id);

        id
    }

    fn insert(&mut self, id: Id<Self>, body: BodyRow) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);
    }

    fn link(&mut self, id: Id<Self>, system: Id<System>) {
        self.system.insert(id, system);
    }

    fn defaults(&mut self, id: Id<Self>) {
        self.position.insert(id, Default::default());
    }
}

impl LinkChild<Surface> for Body {
    fn link_child(&mut self, id: Id<Self>, surface: Id<Surface>) {
        self.surface.insert(id, surface)
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
}

impl CreateLinked<BodyRow> for World {
    type Links = Id<System>;
    type Id = Id<Body>;

    fn create_linked(&mut self, value: BodyRow, system: Id<System>) -> Id<Body> {
        self.state.body.create(&mut self.allocators.body, value, system)
    }
}


#[derive(Debug, Default)]
pub struct Surface {
    pub body: Component<Self, Id<Body>>,
    pub system: Component<Self, Id<System>>,
    pub area: Component<Self, f64>,
    pub temperature: Component<Self, f64>,
    pub albedo: Component<Self, f64>,
}

impl Arena for Surface {
    type Index = u32;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Surface {
    pub fn create(
        &mut self,
        allocator: &mut Allocator<Self>,
        surface: SurfaceRow,
        links: SurfaceLinks,
    ) -> Id<Surface> {
        let id = allocator.create();

        self.insert(id, surface);
        self.link(id, links);
        self.defaults(id);

        id
    }

    fn insert(&mut self, id: Id<Self>, surface: SurfaceRow) {
        self.area.insert(id, surface.area);
        self.albedo.insert(id, surface.albedo);
    }

    fn link(&mut self, id: Id<Self>, links: SurfaceLinks) {
        self.system.insert(id, links.system);
        self.body.insert(id, links.body);
    }

    fn defaults(&mut self, id: Id<Self>) {
        self.temperature.insert(id, Default::default());
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceRow {
    pub area: f64,
    pub albedo: f64,
}

impl CreateLinked<SurfaceRow> for World {
    type Links = SurfaceLinks;
    type Id = Id<Surface>;

    fn create_linked(&mut self, row: SurfaceRow, links: Self::Links) -> Id<Surface> {
        self.state.surface.create(&mut self.allocators.surface, row, links)
    }
}

#[derive(Debug)]
pub struct SurfaceLinks {
    pub body: Id<Body>,
    pub system: Id<System>,
}

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

impl CreateLinked<ColonyRow> for World {
    type Links = ColonyLinks;
    type Id = Id<Colony>;

    fn create_linked(&mut self, row: ColonyRow, links: Self::Links) -> Self::Id {
        self.state.colony.create(&mut self.allocators.colony, row, links)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
    pub government: Id<Government>,
}

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

impl Create<GovernmentRow> for World {
    type Id = Id<Government>;

    fn create(&mut self, row: GovernmentRow) -> Self::Id {
        self.state
            .government
            .create(&mut self.allocators.government, row)
    }
}

#[test]
fn id_sizes() {
    assert_eq!(2, std::mem::size_of::<Id<System>>());
    assert_eq!(4, std::mem::size_of::<Option<Id<System>>>());

    assert_eq!(4, std::mem::size_of::<Id<Body>>());
    assert_eq!(8, std::mem::size_of::<Option<Id<Body>>>());

    assert_eq!(4, std::mem::size_of::<Id<Colony>>());
    assert_eq!(4, std::mem::size_of::<Option<Id<Colony>>>()); // generational indices get option for free
}