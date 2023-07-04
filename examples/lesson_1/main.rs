#[allow(non_snake_case)]

mod outer_layer;

use outer_layer::{inner_layer::print_A_to_z, print_a_to_Z};

fn main() {
    println!("\n\n---第一题---");
    print_a_to_Z();

    println!("\n\n---第二题---");
    print_A_to_z();

    println!("\n")
}
