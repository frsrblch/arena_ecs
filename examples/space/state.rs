use super::*;

#[derive(Debug, Default)]
pub struct State {
    pub arenas: Arenas,
    pub allocators: Allocators,
}

impl State {
    pub fn print_with_government(&self) {
        self.arenas.colony.name.iter()
            .zip(self.arenas.colony.population.iter())
            .zip(self.arenas.colony.government.iter())
            .zip(self.allocators.colony.living())
            .for_each(|(((colony, pop), govt_id), living)| {
                if living {
                    if let Some(govt_id) = self.allocators.government.validate(govt_id) {
                        let govt = unwrap_return!(self.arenas.government.name.get(govt_id));
                        println!("{} ({}): {}", colony, govt, pop);
                    }
                }
            });
    }
}

#[derive(Debug, Default)]
pub struct Arenas {
    pub system: System,
    pub body: Body,
    pub planet_orbit: PlanetOrbit,
    pub moon_orbit: MoonOrbit,
    pub surface: Surface,
    pub colony: Colony,
    pub government: Government,
}

#[derive(Debug, Default)]
pub struct Allocators {
    pub system: Allocator<System>,
    pub body: Allocator<Body>,
    pub planet_orbit: Allocator<PlanetOrbit>,
    pub moon_orbit: Allocator<MoonOrbit>,
    pub surface: Allocator<Surface>,
    pub colony: Allocator<Colony>,
    pub government: Allocator<Government>,
}
