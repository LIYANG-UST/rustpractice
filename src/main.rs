mod base_types;

use base_types::*;

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

    println!("\n----- Main Function Finished -----\n");
}
