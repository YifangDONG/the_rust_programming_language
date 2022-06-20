#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_cover(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn of(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area_tuple(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

#[test]
fn calculate_area() {
    assert_eq!(150, area(15, 10));
}

#[test]
fn calculate_area_tuple() {
    let rect = (15, 10);
    assert_eq!(150, area_tuple(rect));
}

#[test]
fn calculate_area_struct() {
    let rect = Rectangle {
        width: 15,
        length: 10,
    };
    assert_eq!(150, area_struct(&rect));
    print!("{:?} ", rect);
}

#[test]
fn struct_area_function() {
    let rect = Rectangle {
        width: 15,
        length: 10,
    };
    assert_eq!(150, rect.area());
}

#[test]
fn rect1_cover_rect2() {
    let rect1 = Rectangle {
        width: 15,
        length: 10,
    };

    let rect2 = Rectangle {
        width: 10,
        length: 5,
    };

    assert_eq!(true, rect1.can_cover(&rect2));
    assert_eq!(false, rect2.can_cover(&rect1));
}

#[test]
fn associated_function() {
    let rect = Rectangle::of(15, 10);
    assert_eq!(150, rect.area());
}
