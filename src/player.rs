use crate::character::Character;
use crate::position::Position;
use crate::vector::Vector;


#[derive(Debug)]
pub struct Player {
    pub username: String,
    pub position: Position,
    pub direction: Vector,

    pub character: Character,
}

impl Player {
    pub fn new(username: &str, position: Position, direction: Vector) -> Self {
        Player {
            username: username.to_string(),
            position,
            direction,
            character: Character::PepperRoni, // default for now
        }
    }
}
