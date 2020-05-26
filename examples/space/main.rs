use arena_ecs::*;
use state::*;
use system::*;
use body::*;
use orbit::*;
use surface::*;
use government::*;
use colony::*;

mod state;
mod system;
mod body;
mod orbit;
mod surface;
mod government;
mod colony;

fn main() {
    let mut state = State::default();

    let sol = SystemRow {
        name: "Sol".to_string(),
        radius: 696340e3,
        temperature: 5778.0,
        mass: 1.989e30,
    };

    let sol = state.create(sol);

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
        orbit: PlanetOrbitRow {
            period: 365.25 * 24.0 * 60.0 * 60.0,
            radius: 149.6e9,
        },
        moons: vec![Moon { 
            body: BodyRow {
                name: "Luna".to_string(),
                mass: 7.348e22,
                radius: 1737.1e3
            },
            surface: SurfaceRow {
                area: 14.6e6,
                albedo: 0.12
            },
            orbit: MoonOrbitRow {
                period: 27.322 * 24.0 * 60.0 * 60.0,
                radius: 3.48e8,
            },
        }],
    };

    let earth = state.create_linked(earth, sol);

    let usa = GovernmentRow {
        name: "United States of America".to_string(),
    };

    let usa_govt = state.create(usa);

    let links = ColonyLinks {
        body: earth.body,
        government: usa_govt,
    };

    let usa = ColonyRow {
        name: "America".to_string(),
        population: 376e6,
    };

    let _usa = state.create_linked(usa, links);

    let china = GovernmentRow {
        name: "People's Republic of China".to_string(),
    };

    let china_govt = state.create(china);

    let china = ColonyRow {
        name: "China".to_string(),
        population: 1.657e9,
    };

    let links = ColonyLinks {
        body: earth.body,
        government: china_govt,
    };

    let _china = state.create_linked(china, links);

    state.print_with_government();

    let time = 3600.0;
    state.arenas.planet_orbit.update(&mut state.arenas.body, time);
    state.arenas.moon_orbit.update(&mut state.arenas.body, time);

    dbg!(state.arenas.body.position);
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