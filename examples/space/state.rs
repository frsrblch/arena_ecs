use super::*;

#[derive(Debug, Default)]
pub struct State {
    pub system: System,
    pub body: Body,
    pub colony: Colony,
    pub government: Government,
}

impl State {
    pub fn print_with_government(&self) {
        let name = self.colony.name.iter();
        let population = self.colony.population.iter();
        let government = self.colony.government.iter();

        let iter = name.zip(population).zip(government);

        self.colony
            .alloc
            .filter_living(iter)
            .for_each(|((colony, pop), govt_id)| {
                if let Some(govt_id) = self.government.alloc.validate(*govt_id) {
                    let govt = self.government.name.get(govt_id);
                    println!("{} ({}): {}", colony, govt, pop);
                }
            });
    }
}
