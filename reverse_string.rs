fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let foo = "Vamos";
    let reversed = reverse(foo);

    println!("Original: {}", foo);
    println!("Reversed: {}", reversed);
}
