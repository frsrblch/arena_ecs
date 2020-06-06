use arena_ecs::*;
use state::*;
use system::*;
use body::*;
use government::*;
use colony::*;

mod state;
mod system;
mod body;
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

    let sol = state.create_system(sol);

    let earth = Planet {
        body: BodyRow {
            name: "Earth".to_string(),
            mass: 5.972e24,
            radius: 6371e3,
            albedo: 0.3,
            orbit: OrbitParams {
                period: 365.25 * 24.0 * 60.0 * 60.0,
                radius: 149.6e9,
                offset: 0.0
            }
        },
        moons: vec![BodyRow {
            name: "Luna".to_string(),
            mass: 7.348e22,
            radius: 1737.1e3,
            albedo: 0.12,
            orbit: OrbitParams {
                period: 27.322 * 24.0 * 60.0 * 60.0,
                radius: 3.48e8,
                offset: 0.0
            },
        }],
    };

    let earth = state.create_planet(earth, sol);

    let usa = GovernmentRow {
        name: "United States of America".to_string(),
    };

    let usa_govt = state.create_government(usa);

    let links = ColonyLinks {
        body: earth.body,
        government: usa_govt,
    };

    let usa = ColonyRow {
        name: "America".to_string(),
        population: 376e6,
    };

    let _usa = state.create_colony(usa, links);

    let china = GovernmentRow {
        name: "People's Republic of China".to_string(),
    };

    let china_govt = state.create_government(china);

    let china = ColonyRow {
        name: "China".to_string(),
        population: 1.657e9,
    };

    let links = ColonyLinks {
        body: earth.body,
        government: china_govt,
    };

    let _china = state.create_colony(china, links);

    state.print_with_government();

    let time = 3600.0;

    state.arenas.body.update_positions(time);
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