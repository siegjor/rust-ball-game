use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterPlugin)
        .run();
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_races)
            .add_system(print_high_level_characters)
            .add_system(print_low_level_characters)
            .add_system(character_has_class);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Character {
            name: "Turgon Oakshield".to_string(),
            race: Race::Dwarf,
        },
        HighLevel {
            class: Class::Warrior,
        },
    ));
    commands.spawn((
        Character {
            name: "Redmoon Ifrandil".to_string(),
            race: Race::Elf,
        },
        HighLevel {
            class: Class::Cleric,
        },
    ));
    commands.spawn((Character {
        name: "Selena Ventobom".to_string(),
        race: Race::Halfling,
    },));
    commands.spawn((
        Character {
            name: "Boar-horde Estrendor".to_string(),
            race: Race::Orc,
        },
        HighLevel {
            class: Class::Barbarian,
        },
    ));
}

pub fn print_races(character_query: Query<&Character>) {
    for character in character_query.iter() {
        println!("Name: {}\nRace: {:?}\n", character.name, character.race);
    }
}

pub fn print_high_level_characters(character_query: Query<&Character, With<HighLevel>>) {
    for character in character_query.iter() {
        println!("{} has a class!", character.name);
    }
}

pub fn print_low_level_characters(character_query: Query<&Character, Without<HighLevel>>) {
    for character in character_query.iter() {
        println!("{} does not have a class yet!", character.name);
    }
}

pub fn character_has_class(character_query: Query<(&Character, &HighLevel)>) {
    for (character, high_level) in character_query.iter() {
        let class = match high_level.class {
            Class::Barbarian => "Barbarian",
            Class::Warrior => "Warrior",
            Class::Thief => "Thief",
            Class::Mage => "Mage",
            Class::Cleric => "Cleric",
        };

        println!("{0} is a {1}", character.name, class);
    }
}

#[derive(Component)]
pub struct Character {
    pub name: String,
    pub race: Race,
}

#[derive(Component)]
pub struct HighLevel {
    pub class: Class,
}

#[derive(Debug)]
pub enum Race {
    Elf,
    Dwarf,
    Human,
    Halfling,
    Orc,
}

#[derive(Debug)]
pub enum Class {
    Warrior,
    Thief,
    Mage,
    Cleric,
    Barbarian,
}
