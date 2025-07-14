// 作者：异次元
// 链接：https://zhuanlan.zhihu.com/p/595218713
// 来源：知乎
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

pub trait System {
    fn run(&mut self);
}

pub struct App {
    systems: Vec<Box<dyn System>>,
}

impl App {
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run(&mut self) {
        for item in self.systems.iter_mut() {
            item.run();
        }
    }
}

impl<T: FnMut()> System for T {
    fn run(&mut self) {
        self();
    }
}

fn main() {
    App::new()
        .add_system(test_system)
        .add_system(test_system2)
        .run();
}

fn test_system() {
    println!("test_system");
}

fn test_system2() {
    println!("test_system2");
}
