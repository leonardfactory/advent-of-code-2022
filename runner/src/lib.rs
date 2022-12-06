use std::time::Instant;

#[derive(Clone, Copy)]
pub struct Runner {
    start: Instant,
}

impl Runner {
    // Starts timing
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn end(&self) {
        println!(
            "Elapsed: {:.3}ms",
            self.start.elapsed().as_micros() as f64 / 1_000.0
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
