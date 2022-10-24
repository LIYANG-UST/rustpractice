use std::mem::size_of_val;

pub fn base_chartype_1() {
    println!("\n--- Start Function Base Char Type 1 ---\n");

    // Character should use '' rather than ""
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("\n--- Base Char Type 1 Success! ---\n");
}


pub fn print_base_chartype() {
    println!("\n--- Start Function Print Base Char Type ---\n");

    let c1 = '中';
    print_char(c1);

    println!("\n--- Print Base Char Type Success! ---\n");
}


pub fn base_booltype() {
    let _f: bool = false;

    let t = true;

    if !t {
        println!("Success!");
    }
}


fn print_char(c : char) {
    println!("Print Char: {}", c);
}