pub trait Visitor<T> {
    fn visit(&mut self, t: &T);
}

pub enum Entity {
    File(String),
    Dir(String, Vec<Entity>),
}

pub struct ConcreteFileVisitor;

impl Visitor<Entity> for ConcreteFileVisitor {

    fn visit(&mut self, e: &Entity) {
        use Entity::*;

        match *e {
            File(ref name) => println!("file: {}", name),
            Dir(ref name, ref files) => {
                println!("dir: {}", name);
                for file in files {
                    self.visit(file)
                }
            }
        }
    }
}
