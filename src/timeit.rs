use std::time::Instant;

pub struct TimeIt {
    name: String,
    start_time: Instant,
}

impl TimeIt {
    pub fn start(name: &str) -> Self {
        println!("Starting timer: {}", name);
        Self {
            name: String::from(name),
            start_time: Instant::now(),
        }
    }
    pub fn end(self) {
        let elapsed = self.start_time.elapsed();
        println!("Timer '{}' ran for: {:?}", self.name, elapsed);
    }
}