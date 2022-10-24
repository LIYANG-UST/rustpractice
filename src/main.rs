mod base_types;
mod pattern_match;

use base_types::*;
use pattern_match::*;

fn greet_world() {
    let southern_germany = "Grüß Gott";
    let chinese = "世界，你好";
    let english = "Hello, world";

    let regions = [southern_germany, chinese, english];
    // for region in regions.iter() {
    //     println!("{}", &region);
    // }

    for region in regions {
        println!("{}", &region);
    }
}


fn main() {
    println!("\n----- Start Main Function -----\n");

    greet_world();

    base_chartype_1();

    print_base_chartype();

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Dime);

    println!("\n----- Main Function Finished -----\n");
}
