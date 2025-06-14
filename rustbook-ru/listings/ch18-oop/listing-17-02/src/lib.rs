pub struct AveragedCollection {
    список: Vec<i32>,
    average: f64,
}

// ANCHOR: here
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.список.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.список.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.список.iter().sum();
        self.average = total as f64 / self.список.len() as f64;
    }
}
// ANCHOR_END: here
