use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;

    fn display(&self) -> String {
        format!("The Area of {} is {}", self.name(), self.area())
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        (PI * self.radius * self.radius * 100.0).round() / 100.0
    }

    fn name(&self) -> &str {
        "Circle"
    }
}
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) * 0.5
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn name(&self) -> &str {
        "Square"
    }
}

fn print_area<T: Shape>(shape: &T) -> String {
    let message = shape.display();
    println!("{}", message);
    message
}

#[test]
fn test_calc_area() {
    let circle = Circle { radius: 4.00 };
    let triangle = Triangle {
        base: 6.00,
        height: 8.00,
    };
    let square = Square { side: 6.00 };
    assert_eq!(circle.area(), 50.27);
    assert_eq!(print_area(&circle), "The Area of Circle is 50.27");
    assert_eq!(triangle.area(), 24.00);
    assert_eq!(print_area(&triangle), "The Area of Triangle is 24");
    assert_eq!(square.area(), 36.00);
    assert_eq!(print_area(&square), "The Area of Square is 36");
}
