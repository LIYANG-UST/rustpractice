

enum Direction {
    East,
    West,
    North,
    South
}

pub fn pattern_match() {
    let direction = Direction::East;

    match direction {
        Direction::East => println!("This is East"),

        Direction::North | Direction::South => {
            println!("This is North or South");
        },
        
        // Every other potential values
        _ => println!("This is West")
    }
}

pub enum  Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin:: Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin:: Quarter => 25
    }
}