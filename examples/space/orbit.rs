use super::*;

#[derive(Debug, Default)]
pub struct PlanetOrbit {
    pub body: Component<Self, Id<Body>>,
    pub period: Component<Self, f64>,
    pub radius: Component<Self, f64>,
}

impl Arena for PlanetOrbit {
    type Index = <Body as Arena>::Index;
    type Generation = <Body as Arena>::Generation;
    type Allocator = FixedAllocator<Self>;
}

impl PlanetOrbit {
    pub fn update(&mut self, bodies: &mut Body, time: f64) {
        self.body.iter()
            .zip(self.period.iter())
            .zip(self.radius.iter())
            .for_each(|((body, &period), &radius)| {
                let angle = 2.0 * std::f64::consts::PI  * period / time;
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;

                let position = bodies.position.get_mut(body);

                *position = (x, y);
            })
    }

    pub fn create(&mut self, allocator: &mut Allocator<Self>, orbit: PlanetOrbitRow, body: Id<Body>) -> Id<Self> {
        let id = allocator.create();

        self.period.insert(id, orbit.period);
        self.radius.insert(id, orbit.radius);
        self.body.insert(id, body);

        id
    }
}

impl CreateLinked<PlanetOrbitRow> for State {
    type Links = Id<Body>;
    type Id = Id<PlanetOrbit>;

    fn create_linked(&mut self, row: PlanetOrbitRow, links: Self::Links) -> Self::Id {
        self.arenas.planet_orbit.create(&mut self.allocators.planet_orbit, row, links)
    }
}

#[derive(Debug, Clone)]
pub struct PlanetOrbitRow {
    pub period: f64,
    pub radius: f64,
}

#[derive(Debug, Default)]
pub struct MoonOrbit {
    pub body: Component<Self, Id<Body>>,
    pub parent: Component<Self, Id<Body>>,
    pub period: Component<Self, f64>,
    pub radius: Component<Self, f64>,
}

impl Arena for MoonOrbit {
    type Index = <Body as Arena>::Index;
    type Generation = <Body as Arena>::Generation;
    type Allocator = FixedAllocator<Self>;
}

impl MoonOrbit {
    pub fn update(&mut self, bodies: &mut Body, time: f64) {
        self.body.iter()
            .zip(self.parent.iter())
            .zip(self.period.iter())
            .zip(self.radius.iter())
            .for_each(|(((body, parent), &period), &radius)| {
                let angle = 2.0 * std::f64::consts::PI  * period / time;
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;

                println!("x: {}, y: {}", x, y);

                let parent_position = *bodies.position.get(parent);
                let position = bodies.position.get_mut(body);

                *position = (x + parent_position.0, y + parent_position.1);
            })
    }

    pub fn create(&mut self, allocator: &mut Allocator<Self>, orbit: MoonOrbitRow, links: MoonOrbitLinks) -> Id<Self> {
        let id = allocator.create();

        self.period.insert(id, orbit.period);
        self.radius.insert(id, orbit.radius);

        self.body.insert(id, links.body);
        self.parent.insert(id, links.parent);

        id
    }
}

impl CreateLinked<MoonOrbitRow> for State {
    type Links = MoonOrbitLinks;
    type Id = Id<MoonOrbit>;

    fn create_linked(&mut self, row: MoonOrbitRow, links: Self::Links) -> Self::Id {
        self.arenas.moon_orbit.create(&mut self.allocators.moon_orbit, row, links)
    }
}

#[derive(Debug, Clone)]
pub struct MoonOrbitRow {
    pub period: f64,
    pub radius: f64,
}

pub struct MoonOrbitLinks {
    pub body: Id<Body>,
    pub parent: Id<Body>,
}
