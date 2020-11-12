use iter_context::ContextualIterator;

use body::*;
use colony::*;
use gen_id::*;
use government::*;
use state::*;
use system::*;

pub mod body;
pub mod colony;
pub mod government;
pub mod state;
pub mod system;

fn main() {
    let mut state = State::default();

    let sol = SystemRow {
        name: "Sol".to_string(),
        radius: 696340e3,
        temperature: 5778.0,
        mass: 1.989e30,
    };

    let sol = state.system.create(sol);

    let earth = Planet {
        body: BodyRow {
            name: "Earth".to_string(),
            mass: 5.972e24,
            radius: 6371e3,
            albedo: 0.3,
            orbit: OrbitParams {
                period: 365.25 * 24.0 * 60.0 * 60.0,
                radius: 149.6e9,
                offset: 0.0,
            },
        },
        moons: vec![BodyRow {
            name: "Luna".to_string(),
            mass: 7.348e22,
            radius: 1737.1e3,
            albedo: 0.12,
            orbit: OrbitParams {
                period: 27.322 * 24.0 * 60.0 * 60.0,
                radius: 3.48e8,
                offset: 0.0,
            },
        }],
    };

    let earth = state.create_planet(earth, sol);

    let usa = GovernmentRow {
        name: "United States of America".to_string(),
    };

    let usa_govt = state.government.create(usa);

    let links = ColonyLinks {
        body: earth.body,
        government: usa_govt,
    };

    let usa = ColonyRow {
        name: "America".to_string(),
        population: 376e6,
    };

    let _usa = state.colony.create(usa, links);

    let china = GovernmentRow {
        name: "People's Republic of China".to_string(),
    };

    let china_govt = state.government.create(china);

    let china = ColonyRow {
        name: "China".to_string(),
        population: 1.657e9,
    };

    let links = ColonyLinks {
        body: earth.body,
        government: china_govt,
    };

    let _china = state.colony.create(china, links);

    state.print_with_government();

    let time = 3600.0;

    state.body.update_positions(time);
}

#[test]
fn id_sizes() {
    assert_eq!(8, std::mem::size_of::<Id<System>>());
    assert_eq!(8, std::mem::size_of::<Option<Id<System>>>());

    assert_eq!(8, std::mem::size_of::<Id<Body>>());
    assert_eq!(8, std::mem::size_of::<Option<Id<Body>>>());

    assert_eq!(8, std::mem::size_of::<Id<Colony>>());
    assert_eq!(8, std::mem::size_of::<Option<Id<Colony>>>()); // generational indices get option for free
}
