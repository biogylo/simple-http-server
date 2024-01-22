use std::time::Instant;

struct TimeIt {
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
}

impl Drop for TimeIt {
    fn drop(&mut self) {
        let elapsed = self.start_time.elapsed();
        println!("Timer '{}' ran for: {:?}", self.name, elapsed);
    }
}