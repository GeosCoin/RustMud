use std::sync::Mutex;

pub struct Singleton {
    data: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            data: 42
        }
    }

    pub fn get_data(&self) -> i32 {
        self.data
    }
}

lazy_static! {
    pub static ref SINGLETON: Mutex<Singleton> = Mutex::new(Singleton::new());
}
