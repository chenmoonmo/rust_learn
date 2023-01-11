pub fn _first_word() {
    let s = String::from("hello world");

    let first_world = get_first_word(&s);
    println!("First world of \"{}\" is \"{}\"", &s,&first_world)
}


fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}