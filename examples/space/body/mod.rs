use super::*;
pub use planet::*;
use std::collections::HashMap;
use std::f64::consts::PI;

mod planet;

#[derive(Debug, Copy, Clone)]
pub struct OrbitParams {
    pub period: f64,
    pub radius: f64,
    pub offset: f64,
}

impl OrbitParams {
    pub fn moon(&self, parent: Id<Body>) -> MoonOrbitParams {
        MoonOrbitParams {
            parent,
            period: self.period,
            radius: self.radius,
            offset: self.offset,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MoonOrbitParams {
    pub parent: Id<Body>,
    pub period: f64,
    pub radius: f64,
    pub offset: f64,
}

#[derive(Debug, Default)]
pub struct Body {
    pub alloc: Allocator<Self>,

    pub system: Component<Self, Id<System>>,
    pub name: Component<Self, String>,
    pub mass: Component<Self, f64>,
    pub radius: Component<Self, f64>,
    pub albedo: Component<Self, f64>,

    pub position: Component<Self, (f64, f64)>,
    pub temperature: Component<Self, f64>,

    pub planet_orbit: HashMap<Id<Self>, OrbitParams>,
    pub moon_orbit: HashMap<Id<Self>, MoonOrbitParams>,
}

impl Arena for Body {
    type Allocator = FixedAllocator<Self>;
}

impl Body {
    pub fn create(&mut self, row: BodyRow, links: BodyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.insert(id, row, links);

        id
    }

    fn insert(&mut self, id: Id<Self>, body: BodyRow, links: BodyLinks) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);
        self.albedo.insert(id, body.albedo);
        self.system.insert(id, links.system);

        match links.orbit {
            Orbit::Planet => {
                self.planet_orbit.insert(id, body.orbit);
            }
            Orbit::Moon { parent } => {
                self.moon_orbit.insert(id, body.orbit.moon(parent));
            }
        }

        self.position.insert(id, Default::default());
    }

    pub fn update_positions(&mut self, time: f64) {
        self.update_planet_orbits(time);
        self.update_moon_orbits(time);
    }

    fn update_planet_orbits(&mut self, time: f64) {
        let positions = &mut self.position;

        self.planet_orbit.iter().for_each(|(body, orbit)| {
            let orbit_fraction = time / orbit.period + orbit.offset;
            let angle = orbit_fraction * Self::TWO_PI;

            let pos = positions.get_mut(body);
            pos.0 = angle.sin();
            pos.1 = angle.cos();
        });
    }

    fn update_moon_orbits(&mut self, time: f64) {
        let positions = &mut self.position;

        self.moon_orbit.iter().for_each(|(body, orbit)| {
            let orbit_fraction = time / orbit.period + orbit.offset;
            let angle = orbit_fraction * Self::TWO_PI;

            let parent_pos = *positions.get(orbit.parent);
            let pos = positions.get_mut(body);
            pos.0 = parent_pos.0 + angle.sin();
            pos.1 = parent_pos.1 + angle.cos();
        });
    }

    const TWO_PI: f64 = PI * 2.0;
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
    pub albedo: f64,
    pub orbit: OrbitParams,
}

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub system: Id<System>,
    pub orbit: Orbit,
}

#[derive(Debug, Copy, Clone)]
pub enum Orbit {
    Planet,
    Moon { parent: Id<Body> },
}
