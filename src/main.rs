mod command;
mod factory;
mod visitor;
mod composite;

use visitor::Entity::{Dir,File};
use factory::{ConcreteProductX, Factory};
use command::{Command, Invoker};

// Usecase

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }

    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn set_direction(&mut self, d: (i32, i32)) {
        self.dx = d.0;
        self.dy = d.1;
    }

    fn get_direction(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}

enum RoboCommand {
    MoveForward,
    TurnRight,
    TurnLeft,
}

impl Command<Robot> for RoboCommand {

    fn execute(&self, r: &mut Robot) {
        use RoboCommand::*;
        match *self {
            MoveForward => r.move_forward(),
            TurnRight => {
                let (dx, dy) = r.get_direction();
                r.set_direction((dy, -dx))  // (0,1) => (1, 0)
            }
            TurnLeft => {
                let (dx, dy) = r.get_direction();
                r.set_direction((-dy, dx))  // (0,1) => (-1, 0)
            }
        }
    }

    fn undo(&self, r: &mut Robot) {
        use RoboCommand::*;
        match *self {
            MoveForward => {
                let c1 = TurnRight;
                c1.execute(r);  
                c1.execute(r);  // 180度回転
                self.execute(r);    // 前進
                c1.execute(r);
                c1.execute(r);  // 180度回転
            }
            TurnRight => {
                let c = TurnLeft;
                c.execute(r);
            }
            TurnLeft => {
                let c = TurnRight;
                c.execute(r);
            }
        }
    }
}

fn main() {
    // Command Pattern
    let mut r= Robot::new();

    let mut invoker = Invoker::new(&mut r);
    assert_eq!(*invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy:1
        }
    );
    {
        use RoboCommand::*;
        invoker.append_command(TurnRight);
        invoker.append_command(TurnLeft);
        invoker.append_command(MoveForward)
    }

    invoker.execute_all_commands();
    assert_eq!(*invoker.target(),
        Robot {
            x:0,
            y:1,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(*invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    // Factory
    let f = Factory;
    println!("{}", f.convert(String::from("hogehoge piyopiyo"), || ConcreteProductX));

    // visitor
    use visitor::*;
    let e = Dir(String::from("/"), vec![File(String::from("etc")), File(String::from("usr"))]);
    let mut visitor = ConcreteFileVisitor;
    visitor.visit(&e);
}
