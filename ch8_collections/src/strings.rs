pub fn demo_strings() {
    let mut s = "initial contents".to_string();

    println!("{s}");

    s.push_str(" with more contents");

    println!("{s}")
}