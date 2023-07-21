fn main() {
    let c = reverse("Hello");
    println!("{}", c);
}

fn reverse(input: &str) -> String {
    let t = input.chars().rev().collect::<String>();
    return t;
}
