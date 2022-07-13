fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

#[test]
fn function_pattern() {
    let point = (3, 5);
    print_coordinates(&point);
}

#[test]
fn refutability() {
    let a = Some(5);
    if let Some(x) = a {
        println!("value is {}", x);
    };
}

enum Message {
    Hello { id: i32 },
}
#[test]
fn binding() {
    let checker = |msg| match msg {
        Message::Hello {
            id: id_variable @ 0..=5,
        } => format!("YES {}", id_variable),
        Message::Hello { id: 6..=10 } => "NO".to_string(),
        Message::Hello { id } if id < 50 => format!("MAYBE {}", id),
        Message::Hello { id } => format!("NA {}", id), // the last pattern should be irrefutable
    };
    assert_eq!("YES 5", checker(Message::Hello { id: 5 }));
    assert_eq!("NO", checker(Message::Hello { id: 6 }));
    assert_eq!("MAYBE 40", checker(Message::Hello { id: 40 }));
    assert_eq!("NA 100", checker(Message::Hello { id: 100 }));
}
