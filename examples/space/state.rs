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
        let iter = self
            .colony
            .name
            .iter()
            .zip(&self.colony.population)
            .zip(&self.colony.government);

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
