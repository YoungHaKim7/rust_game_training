#[derive(Debug)]
enum Gem {
    Diamond,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl Gem {
    fn from_number(num: u8) -> Option<Gem> {
        match num {
            1 => Some(Gem::Diamond),
            2 => Some(Gem::Sapphire),
            3 => Some(Gem::Ruby),
            4 => Some(Gem::Topaz),
            5 => Some(Gem::Onyx),
            6 => Some(Gem::Jade),
            _ => None,
        }
    }
}

fn main() {
    let mut map = [[0; 5]; 5];

    map[4][2] = 1;
    map[1][2] = 2;
    map[3][3] = 3;
    map[0][2] = 4;
    map[1][4] = 5;

    for row in map {
        println!("{row:?}");
    }

    let mut found: Vec<Gem> = Vec::new();
    let data = map[1][4];
    found.push(Gem::from_number(data).expect("Invalid number"));

    println!("{found:?}")
}
