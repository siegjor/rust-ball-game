use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(print_names)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        race: "Elf".to_string(),
    });
    commands.spawn(Person {
        race: "Dwarf".to_string(),
    });
    commands.spawn(Person {
        race: "Halfling".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Race: {}", person.race);
    }
}

#[derive(Component)]
pub struct Person {
    pub race: String,
}
