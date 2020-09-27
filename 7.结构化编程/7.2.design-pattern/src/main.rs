use std::process::Command;
// 建造者模式示例
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * (self.radius * self.radius) }

    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }
}

impl CircleBuilder {
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn build(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

fn main() {
    // 建造者模式示例
    let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
    assert_eq!(c.area(), 12.566370614359172);
    assert_eq!(c.x, 1.0);
    assert_eq!(c.y, 2.0);

    // `std::process::Command` 使用示例
    Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
}
