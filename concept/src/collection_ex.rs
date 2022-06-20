use std::collections::HashMap;

#[test]
fn create_vec() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let vec2 = vec![1, 2, 3];
    assert_eq!(vec, vec2);
}

#[test]
fn read_from_vec() {
    let v = vec![0, 1, 2, 3, 4];
    let two = v[2];
    assert_eq!(2, two);
}

#[test]
fn access_vec_by_reference() {
    let v = vec![0, 1, 2, 3, 4];

    let two = &v[2];
    assert_eq!(2, *two); //de-reference

    match v.get(2) {
        Some(num) => assert_eq!(2, *num),
        None => (),
    };
}

#[test]
fn borrow_mut() {
    let mut v = vec![0, 1, 2];
    // immutable borrow
    let first = &v[0];
    println!("The first element is {}", first);
    // immutable borrow is out of scope
    v.push(3);
}

#[test]
fn double_vec() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i = *i * 2;
    }

    for i in &v {
        print!("{} ", i);
    }
}

#[test]
fn zip_vec_to_map() {
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let scores = vec![10, 50];

    let team_to_score: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("{:?}", team_to_score);
}

#[test]
fn change_ownership_after_insert_to_hashmap() {
    let key = String::from("FYI");
    let value = String::from("For Your Information");
    let mut map = HashMap::new();
    map.insert(key, value);

    // println!("{}: {}", key, value); cannot compile because ownership change
}

#[test]
fn insert_if_absent() {
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.entry("Blue".to_string()).or_insert(20);
    assert_eq!(10, *map.get("Blue").unwrap());
}

#[test]
fn count_chars() {
    let text = "hello world";
    let mut map = HashMap::new();
    for char in text.chars() {
        let count = map.entry(char).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
