fn main() {
    let mut map = [[0; 5]; 5];

    map[4][2] = 1;
    map[1][2] = 1;
    map[3][3] = 1;
    map[0][2] = 1;
    map[1][4] = 1;

    for row in map {
        println!("{row:?}");
    }
}
