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

    pub fn insert(&mut self, id: Id<PlanetOrbit>, orbit: PlanetOrbitRow, body: Id<Body>) {
        self.period.insert(id, orbit.period);
        self.radius.insert(id, orbit.radius);
        self.body.insert(id, body);
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

                let parent_position = *bodies.position.get(parent);
                let position = bodies.position.get_mut(body);

                *position = (x + parent_position.0, y + parent_position.1);
            })
    }

    pub fn insert(&mut self, id: Id<Self>, orbit: MoonOrbitRow, links: MoonOrbitLinks) {
        self.period.insert(id, orbit.period);
        self.radius.insert(id, orbit.radius);

        self.body.insert(id, links.body);
        self.parent.insert(id, links.parent);
    }
}

#[derive(Debug, Clone)]
pub struct MoonOrbitRow {
    pub period: f64,
    pub radius: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct MoonOrbitLinks {
    pub body: Id<Body>,
    pub parent: Id<Body>,
}
