fn main() {
    let mut world = World::default();

    let sol = SystemRow {
        name: "Sol".to_string(),
        radius: 696340e3,
        temperature: 5778.0,
        mass: 1.989e30,
    };

    let sol = world.create_system(sol);
    
    let earth = Planet {
        body: BodyRow {
            name: "Earth".to_string(),
            mass: 5.972e24,
            radius: 6371e3,
        },
        surface: Some(SurfaceRow {
            area: 510.1e6,
            albedo: 0.3,
        })
    };

    let _earth = world.create_planet(earth, sol);
}

use ecs_traits::*;

#[derive(Debug, Default)]
pub struct World {
    pub state: State,
    pub allocators: Allocators,
}

impl World {
    pub fn create_system(&mut self, system: SystemRow) -> Id<System> {
        self.state.system.create(&mut self.allocators.system, system)
    }

    pub fn create_planet(&mut self, planet: Planet, system: Id<System>) -> PlanetIds {
        let body = self.state.body.create(&mut self.allocators.body, planet.body, system);

        let surface = planet.surface.map(|surface| {
            let links = SurfaceLinks { body, system };
            self.state.surface.create(&mut self.allocators.surface, surface, links)
        });

        PlanetIds { body, surface }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub system: System,
    pub body: Body,
    pub surface: Surface,
    pub colony: Colony,
}

#[derive(Debug, Default)]
pub struct Allocators {
    pub system: Allocator<System>,
    pub body: Allocator<Body>,
    pub surface: Allocator<Surface>,
    pub colony: Allocator<Colony>,
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
    type Generations = Self::Index;
    type Dead = ();
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

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub surface: Option<SurfaceRow>,
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
}

impl Arena for Body {
    type Index = u32;
    type Generation = ();
    type Generations = Self::Index;
    type Dead = Vec<Self::Index>;
}

impl Body {
    pub fn create(&mut self, allocator: &mut Allocator<Self>, body: BodyRow, system: Id<System>) -> Id<Self> {
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

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
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
    type Generations = Self::Index;
    type Dead = ();
}

impl Surface {
    pub fn create(&mut self, allocator: &mut Allocator<Self>, surface: SurfaceRow, links: SurfaceLinks) -> Id<Surface> {
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
}

impl Arena for Colony {
    type Index = u16;
    type Generation = NonZeroU16;
    type Generations = Vec<Self::Generation>;
    type Dead = Vec<Self::Index>;
}

impl Colony {
    pub fn create(&mut self, allocator: &mut Allocator<Self>, colony: ColonyRow, body: Id<Body>) -> Id<Self> {
        let id = allocator.create();

        self.insert(id, colony);
        self.link(id, body);
        self.defaults(id);

        id.id
    }

    fn insert(&mut self, id: Valid<Self>, colony: ColonyRow) {
        self.name.insert(id, colony.name);
        self.population.insert(id, colony.population);
    }

    fn link(&mut self, id: Valid<Self>, body: Id<Body>) {
        self.body.insert(id, body);
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

#[test]
fn id_sizes() {
    assert_eq!(2, std::mem::size_of::<Id<System>>());
    assert_eq!(4, std::mem::size_of::<Option<Id<System>>>());

    assert_eq!(4, std::mem::size_of::<Id<Body>>());
    assert_eq!(8, std::mem::size_of::<Option<Id<Body>>>());

    assert_eq!(4, std::mem::size_of::<Id<Colony>>());
    assert_eq!(4, std::mem::size_of::<Option<Id<Colony>>>()); // generational indices get option for free
}