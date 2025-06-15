struct Счётчик {
    count: u32,
}

impl Счётчик {
    fn new() -> Счётчик {
        Счётчик { count: 0 }
    }
}

// ANCHOR: ch19
impl Iterator for Счётчик {
    type Предмет = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        // ANCHOR_END: ch19
        if self.count < 5 {
            self.счётчик += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
