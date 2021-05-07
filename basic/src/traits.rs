trait Geometry {
  fn area(&self) -> f32;
  fn perimeter(&self) -> f32;
}


struct Rectangle {
  width: f32,
  height: f32,
}

struct Circle {
  radius: f32,
}


impl Geometry for Rectangle {
  fn area(&self) -> f32 {
    self.height * self.width
  }

  fn perimeter(&self) -> f32 {
    (self.height + self.width) * 2.0
  }
}

use std::f32::consts::PI;

impl Geometry for Circle {
  fn area(&self) -> f32 {
    PI * self.radius * self.radius
  }

  fn perimeter(&self) -> f32 {
    2.0 * PI * self.radius
  }
}


fn print(rect: impl Geometry) {
  println!("area is {}, perimeter is {}", rect.area(), rect.perimeter());
}

pub fn rect_test() {
  let rect = Rectangle{
    width: 2.0,
    height: 3.2,
  };

  print(rect);
}