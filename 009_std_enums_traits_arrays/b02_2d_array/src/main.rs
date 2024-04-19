fn main() {
    let mut map = [[false; 5]; 5];

    map[4][2] = true;
    map[1][2] = true;
    map[3][3] = true;
    map[0][2] = true;
    map[1][4] = true;

    for row in map {
        println!("{row:?}");
    }
}
