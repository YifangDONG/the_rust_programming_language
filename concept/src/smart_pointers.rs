use std::{fmt::format, ops::Deref};
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[test]
fn box_implement_deref_trait() {
    let x = 5;
    let a = &x;
    let b = Box::new(x);
    let c = MyBox::new(x); // *(x.deref())

    assert_eq!(5, x);
    assert_eq!(5, *a);
    assert_eq!(5, *b);
    assert_eq!(5, *c);
}

fn hello(x: &str) -> String {
    format!("hello {}", x)
}

#[test]
fn deref_coercion() {
    let rust = MyBox::new(String::from("rust"));
    // &MyBox<String> => deref => &String => deref => &str match the parameter in hello
    let result = hello(&rust);
    assert_eq!("hello rust".to_string(), result);
}

enum List {
    Node(i32, Box<List>),
    Nil,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Nil => write!(f, ""),
            List::Node(a, b) => match &**b {
                // compiler asks me to do &**b but I don't understand why
                List::Nil => write!(f, "{}", a),
                List::Node(_, _) => write!(f, "{},{}", a, b),
            },
        }
    }
}

#[test]
fn create_const_list() {
    let list = List::Node(
        1, //
        Box::new(List::Node(
            2, //
            Box::new(List::Node(
                3, //
                Box::new(List::Nil),
            )),
        )),
    );

    let output = format!("{}", list);
    assert_eq!("1,2,3".to_string(), output);
}
