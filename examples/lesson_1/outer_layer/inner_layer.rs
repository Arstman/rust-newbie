pub fn print_A_to_z() {
    for c in ('A'..='Z').chain('a'..='z') {
        print!("{c}");
    }
}
