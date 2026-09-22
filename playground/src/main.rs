fn main() {
    // `if` produces a value, so it can go on the right of `let`
    let year = 2026;
    let kind = if year % 4 == 0 { "leap" } else { "normal" };
    println!("{year} is a {kind} year.");

    // `for` over a range: 2024, 2025, ... 2028
    for y in 2024..=2028 {
        let kind = if y % 4 == 0 { "leap" } else { "normal" };
        println!("{y}: {kind}");
    }

    // `while` needs a `mut` counter, which is where `mut` earns its keep
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("Go!");
}
