use std::process::Command;
use serde::Deserialize;
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

// RAII 模式示例, 第一版
#[derive(Clone)]
pub struct Letter {
    text: String
}

// 信封
// pub struct Envolpe {
//     // 有无信两种状态
//     letter: Option<Letter>,
// }

// // 表示信被装走
// pub struct PickupLorryHandle {
//     // 表示状态
//     done: bool
// }

// impl Letter {
//     // 写信
//     pub fn new(text: String) -> Self {
//         Letter {
//             text
//         }
//     }
// }

// impl Envolpe {
//     // 装信
//     pub fn wrap(&mut self, letter: &Letter) {
//         self.letter = Some(letter.clone());
//     }
// }

// // 购买带邮戳的信封
// pub fn buy_prestamped_envelope() -> Envolpe {
//     Envolpe {
//         letter: None
//     }
// }

// impl PickupLorryHandle {
//     // 装车
//     pub fn pickup(&mut self, envelope: &Envolpe) {
//         // give letter
//     }

//     // 寄送
//     pub fn done(&mut self) {
//         self.done = true;
//         println!("sent");
//     }
// }

// // 装车准备寄送
// pub fn order_pickup() -> PickupLorryHandle {
//     PickupLorryHandle {
//         done: false,
//         // other handles...
//     }
// }

// 第二版
// 空信封
pub struct EmptyEnvelope {}
// 已装好信件的信封
pub struct ClosedEnvelope { letter: Letter }
pub struct PickupLorryHandle { done: bool }
impl Letter {
    pub fn new(text: String) -> Self {
        Letter {
            text
        }
    }
}

impl EmptyEnvelope {
    // 空信封的方法 - 转移所有权
    pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
        // 实例化为装好信件的信封
        ClosedEnvelope {
            letter
        }
    }
}

// 购买空信封
pub fn buy_prestamped_envelope() -> EmptyEnvelope {
    EmptyEnvelope {}
}

impl PickupLorryHandle {
    // 接收已装好信件的信封
    pub fn pickup(&mut self, envelope: ClosedEnvelope) {
        // give letter
    }

    pub fn done(self) {}
}

impl Drop for PickupLorryHandle {
    fn drop(&mut self) {
        println!("sent");
    }
}

pub fn order_pickup() -> PickupLorryHandle {
    PickupLorryHandle {
        done: false,
        // otherhanle
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

    // RAII 模式示例
    // 第一版:
    // let letter = Letter::new(String::from("Dear RustFest"));
    // let mut envelope = buy_prestamped_envelope();
    // envelope.wrap(&letter);
    // let mut lorry = order_pickup();
    // lorry.pickup(&envelope);
    // lorry.done();

    // 第二版
    let letter = Letter::new(String::from("Dear RustFest"));
    let envelope = buy_prestamped_envelope();
    let closed_envelope = envelope.wrap(letter);
    let mut lorry = order_pickup();
    lorry.pickup(closed_envelope);
    
}
