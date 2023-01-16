pub fn borrow() {
    let x = 5;
    let y = &5;

    let s =  String::from("hello world");
    let s2 = &s;
    assert_eq!("hello world", s2);

    let mut s = String::from("hello");

    can_change(&mut s)
    
}


// fn can_not_change (s: &String) {
//     s.push_str(",world"); // 无法修改
// }

fn can_change (s:  &mut String) {
    s.push_str(",world"); 
}