fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    println!("{r1}"); // r1's last use — done with it before r2 shows up

    let r2 = &mut s;
    r2.push_str(", world");
    println!("{r2}");
}
