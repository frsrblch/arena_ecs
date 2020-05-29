use arena_ecs::*;
use rand::random;
use std::fmt::Formatter;

fn main() {
    let mut state = State::default();

    for _ in 0..20 {
        let ship = state.create_ship(ShipRow::random());

        if random::<f32>() < 0.9 {
            let captain = state.create_captain(CaptainRow::random());
            state.assign_captain(ship, captain);
        }
    }

    for _ in 0..rand_between(0,4) {
        state.create_captain(CaptainRow::random());
    }

    state.print_captained_ships();
    state.print_uncaptained_ships();
    state.print_shipless_captains();
}

#[derive(Debug, Default)]
pub struct State {
    pub arenas: Arenas,
    pub allocators: Allocators,
}

impl State {
    pub fn create_ship(&mut self, ship: ShipRow) -> Id<Ship> {
        self.arenas.ship.create(&mut self.allocators.ship, ship)
    }

    pub fn create_captain(&mut self, captain: CaptainRow) -> Id<Captain> {
        self.arenas.captain.create(&mut self.allocators.captain, captain)
    }

    pub fn assign_captain(&mut self, ship: Id<Ship>, captain: Id<Captain>) {
        self.arenas.ship.captain.insert(ship, captain);
        self.arenas.captain.ship.insert(captain, ship);
    }

    pub fn uncaptained_ships<'a>(&'a self) -> impl Iterator<Item=ValidRef<'a, Ship>> + 'a {
        self.allocators.ship.ids()
            .filter(move |id| self.arenas.ship.captain.get(id).is_none())
    }

    pub fn shipless_captains<'a>(&'a self) -> impl Iterator<Item=ValidRef<'a, Captain>> + 'a {
        self.allocators.captain.ids()
            .filter(move |id| self.arenas.captain.ship.get(id).is_none())
    }

    pub fn print_captained_ships(&mut self) {
        let ship_alloc = &self.allocators.ship;
        let captain_alloc = &self.allocators.captain;

        let ship = &self.arenas.ship;
        let captain = &self.arenas.captain;

        println!("Captained Ships:\n");

        ship.captain.iter()
            .for_each(|(s, c)| {
                if let (Some(s), Some(c)) = (ship_alloc.validate(*s), captain_alloc.validate(*c)) {
                    println!("{}", ship.entity(s));
                    println!("{}", captain.entity(c));
                    println!();
                }
            });

        println!();
    }

    pub fn print_uncaptained_ships(&mut self) {
        let alloc = &self.allocators.ship;
        let ship = &self.arenas.ship;

        println!("Uncaptained Ships:\n");

        alloc.ids()
            .filter(|id| ship.captain.get(id).is_none())
            .for_each(|id| println!("{}", ship.entity(id)));

        println!();
    }

    pub fn print_shipless_captains(&mut self) {
        let alloc = &self.allocators.captain;
        let captain = &self.arenas.captain;

        println!("Shipless Captains:\n");

        alloc.ids()
            .filter(|id| captain.ship.get(id).is_none())
            .for_each(|id| println!("{}", captain.entity(id)));

        println!();
    }
}

#[derive(Debug, Default)]
pub struct Arenas {
    pub ship: Ship,
    pub captain: Captain,
}

#[derive(Debug, Default)]
pub struct Allocators {
    pub ship: Allocator<Ship>,
    pub captain: Allocator<Captain>,
}

#[derive(Debug, Default)]
pub struct Ship {
    pub name: Component<Self, String>,
    pub ship_type: Component<Self, ShipType>,
    pub tonnage: Component<Self, u32>,

    // IdMap used for the sake of example
    pub captain: IdMap<Self, Captain>,
}

impl Arena for Ship {
    type Index = u32;
    type Generation = NonZeroU32;
    type Allocator = DynamicAllocator<Self>;
}

impl Ship {
    pub fn create(&mut self, allocator: &mut Allocator<Self>, row: ShipRow) -> Id<Self> {
        let id = allocator.create();

        self.name.insert(id, row.name);
        self.ship_type.insert(id, row.ship_type);
        self.tonnage.insert(id, row.tonnage);

        id.id()
    }
}

impl DisplayEntity for Ship {
    fn fmt_entity<I: Indexes<Self>>(&self, id: I, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {:?}, Tonnage: {}", self.name.get(id), self.ship_type.get(id), self.tonnage.get(id))
    }
}

pub struct ShipRow {
    pub name: String,
    pub ship_type: ShipType,
    pub tonnage: u32,
}

impl ShipRow {
    pub fn random() -> Self {
        let ship_type = ShipType::random();
        ShipRow {
            name: random::<u8>().to_string(),
            ship_type,
            tonnage: ship_type.random_tonnage(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ShipType {
    Freighter,
    Corvette,
    Survey,
}

impl ShipType {
    pub fn random() -> Self {
        match random::<f32>() {
            f if f > 0.9 => Self::Corvette,
            f if f < 0.2 => Self::Survey,
            _ => Self::Freighter,
        }
    }

    pub fn random_tonnage(&self) -> u32 {
        match self {
            Self::Corvette => rand_between(5, 10) * 1000,
            Self::Survey => rand_between(6, 14) * 500,
            Self::Freighter => rand_between(4, 8) * 10000,
        }
    }
}

fn rand_between(min: u32, max: u32) -> u32 {
    let diff = max - min;
    min + (diff as f32 * random::<f32>()).round() as u32
}

#[derive(Debug, Default)]
pub struct Captain {
    pub name: Component<Self, String>,
    pub age: Component<Self, u8>,
    pub ability: Component<Self, Ability>,

    pub ship: IdMap<Self, Ship>,
}

impl Arena for Captain {
    type Index = u32;
    type Generation = NonZeroU32;
    type Allocator = DynamicAllocator<Self>;
}

impl Captain {
    pub fn create(&mut self, allocator: &mut Allocator<Self>, row: CaptainRow) -> Id<Self> {
        let id = allocator.create();

        self.name.insert(id, row.name);
        self.age.insert(id, row.age);
        self.ability.insert(id, row.ability);

        id.id()
    }
}

impl DisplayEntity for Captain {
    fn fmt_entity<I: Indexes<Self>>(&self, id: I, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - Age: {}, Ability: {:?}", self.name.get(id), self.age.get(id), self.ability.get(id))
    }
}

pub struct CaptainRow {
    pub name: String,
    pub age: u8,
    pub ability: Ability,
}

impl CaptainRow {
    pub fn random() -> Self {
        CaptainRow {
            name: random::<u8>().to_string(),
            age: rand_between(30, 65) as u8,
            ability: Ability::random(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Ability {
    Exceptional,
    Average,
    Poor,
}

impl Ability {
    pub fn random() -> Self {
        match random::<f32>() {
            f if f > 0.9 => Self::Exceptional,
            f if f < 0.1 => Self::Poor,
            _ => Self::Average,
        }
    }
}