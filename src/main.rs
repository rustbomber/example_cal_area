use std::f32::consts::PI;

trait Shape {
    fn get_area(&self) -> f32;
}

struct Circular {
    r: f32,
}
struct Triangle {
    height: f32,
    width: f32,
}
struct Square {
    width: f32,
    height: f32,
}

impl Shape for Circular {
    fn get_area(&self) -> f32 {
        PI * self.r * self.r
    }
}
impl Shape for Triangle {
    fn get_area(&self) -> f32 {
        (self.height * self.width) / 2_f32
    }
}
impl Shape for Square {
    fn get_area(&self) -> f32 {
        self.height * self.width
    }
}

fn cal_area<T: Shape>(input: T) -> f32 {
    let area = input.get_area();
    area
}

fn main() {
    let circular = Circular { r: 4.0 };
    let circular_area = cal_area(circular);
    assert_eq!(circular_area, PI * 4.0 * 4.0);
    println!("the circular area is: {}", circular_area);

    let triangle = Triangle {
        height: 3.0,
        width: 4.0,
    };
    let triangle_area = cal_area(triangle);
    assert_eq!(triangle_area, 3.0 * 4.0 / 2.0);
    println!("the triangle area is: {}", triangle_area);

    let square = Square {
        height: 12.0,
        width: 15.0,
    };
    let square_area = cal_area(square);
    assert_eq!(square_area, 12.0 * 15.0);
    println!("the square area is: {}", square_area);
}
