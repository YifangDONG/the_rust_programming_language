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

use std::rc::Rc;
#[derive(Debug)]
enum SharedList {
    Node(i32, Rc<SharedList>),
    Nil,
}
#[test]
fn create_shared_const_list() {
    let a = Rc::new(SharedList::Node(
        1,
        Rc::new(SharedList::Node(2, Rc::new(SharedList::Nil))),
    ));
    let b = SharedList::Node(3, Rc::clone(&a));
    let c = SharedList::Node(4, Rc::clone(&a));
    println!("{:?}", b);
    println!("{:?}", c);
}

trait Notifier {
    fn send(&self, msg: &str);
}

struct TextNotifier {}

impl Notifier for TextNotifier {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}
struct LimitService<'a, T: Notifier> {
    notifier: &'a T,
    value: u32,
    quota: u32,
}

impl<'a, T: Notifier> LimitService<'a, T> {
    pub fn new(notifier: &T, quota: u32) -> LimitService<T> {
        LimitService {
            notifier,
            value: 0,
            quota,
        }
    }

    fn reserve(&mut self, value: u32) {
        self.value = value;
        let percentage = self.value as f64 / self.quota as f64;
        if percentage >= 1.0 {
            self.notifier.send("Error: over quota");
        } else if percentage >= 0.75 {
            self.notifier.send("Warning: used 75% of quota");
        } else {
            self.notifier.send("OK");
        }
    }
}

#[test]
fn send_message_when_over_quota() {
    let notifier = TextNotifier {};
    let mut limit_service = LimitService::new(&notifier, 10);
    limit_service.reserve(20);
    limit_service.reserve(8);
    limit_service.reserve(5);
}

// use mock notifier to check the sended message
use std::cell::RefCell;
struct MockNotifier {
    messages: RefCell<String>,
}
impl MockNotifier {
    fn new() -> MockNotifier {
        MockNotifier {
            messages: RefCell::new(String::new()),
        }
    }
}
impl Notifier for MockNotifier {
    fn send(&self, msg: &str) {
        // because in the trait, the self is defined as immutable, so in the test we cannot create a mock change the self to set the message.
        // use RefCell to change the immutable self.
        self.messages.replace(msg.to_string());
        // can't do *(self.messages) = msg.to_string(); because RefCell does't implement DeRef
    }
}
#[test]
fn send_message() {
    let notifier = MockNotifier::new();
    let mut limit_service = LimitService::new(&notifier, 10);
    limit_service.reserve(20);
    assert_eq!(
        "Error: over quota".to_string(),
        notifier.messages.clone().into_inner() // need clone because into_inner move borrow, TODO check how to avoid clone
    );
    limit_service.reserve(8);
    assert_eq!(
        "Warning: used 75% of quota".to_string(),
        notifier.messages.clone().borrow().to_string()
    );
    limit_service.reserve(5);
    assert_eq!("OK".to_string(), notifier.messages.into_inner());
}

#[derive(Debug)]
enum MutSharedList {
    Node(Rc<RefCell<i32>>, Rc<MutSharedList>),
    Nil,
}
#[test]
fn create_shared_const_list_then_modify_it() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(MutSharedList::Node(
        Rc::clone(&value),
        Rc::new(MutSharedList::Nil),
    ));
    let b = MutSharedList::Node(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = MutSharedList::Node(Rc::new(RefCell::new(3)), Rc::clone(&a));

    *value.borrow_mut() += 10; // can do this because Rc implements the DeRef but not RefCell

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
