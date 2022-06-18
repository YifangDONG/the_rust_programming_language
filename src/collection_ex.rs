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


