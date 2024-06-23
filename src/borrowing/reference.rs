pub fn reference_and_dereference1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn reference_and_dereference2() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn reference_and_dereference3() {
    let mut s1 = String::from("hello");
    // s1 传入不可变借用
    println!("s1: {}.", s1);

    let s2 = change(&mut s1);
    // error: s2 为可变借用 s1 为不可变借用
    // println!("s1: {}.", s1);
    println!("s2: {}.", s2);
}

fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(", world");
    some_string
}