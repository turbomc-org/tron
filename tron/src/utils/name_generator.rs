use names::Generator;
use rand::{Rng, rng};

pub fn generate() -> String {
    let mut generator = Generator::default();
    let mut range = rng();
    let mut name = generator.next().unwrap();
    name = name.replace("-", "_");
    let num: u16 = range.random_range(10..999);
    name.push_str(&num.to_string());

    name
}
