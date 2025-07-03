use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn добавить(self, иной: Meters) -> Millimeters {
        Millimeters(self.0 + (иной.0 * 1000))
    }
}
