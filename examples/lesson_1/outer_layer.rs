pub mod inner_layer;


pub fn print_a_to_Z() {
    for c in ('a'..='z').chain('A'..='Z') {
        print!("{c}");
    }
}
