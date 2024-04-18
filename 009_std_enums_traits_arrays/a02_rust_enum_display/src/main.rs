use std::fmt;

enum Gem {
    Diamond,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}

fn main() {
    let gems = [
        (Gem::Onyx, 25.00),
        (Gem::Diamond, 100.00),
        (Gem::Topaz, 5.00),
        (Gem::Onyx, 50.00),
    ];

    for gem in gems {
        println!("This gem is worth {}", gem.0);
    }
}
