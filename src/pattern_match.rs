

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

enum IpAddr {
    IPV4,
    IPV6
}

fn match_ip() {
    let ip1 = IpAddr::IPV4;

    let ip_str = match ip1 {
        IpAddr::IPV4 => "127.0.0.1",
        _ => "::1",
    };

    println!("IP address: {}", ip_str);
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn match_action () {
    let actions = [
        Action::Say("Hello".to_string()),
        Action::MoveTo(10, 20),
        Action::ChangeColorRGB(255, 0, 0),

    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("Say: {}", s);
            },

            Action::MoveTo(x, y) => {
                println!("Point from (0,0) move to ({}, {})", x, y);
            },

            Action::ChangeColorRGB(r,g, _) => {
                println!("Change color to RGB({},{},{})", r, g, 0);
            }
            
        }
    }
}


enum MyEnum {
    Foo,
    Bar
}

fn matches() {
    let v = vec![MyEnum::Foo, MyEnum::Bar];

    v.iter().filter(|x| matches!(x, MyEnum::Foo));
}