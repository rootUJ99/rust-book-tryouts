fn main() {
    let song: [&str; 12] = [
        // "My good friends brought to me",
        "All their good wishes",
        "Gifts for one and all",
        "Some mistletoe",
        "A guardian angel",
        "Gold and silver tinsel",
        "Candles a-glowing",
        "Little silver bells",
        "A shining star",
        "Four colored lights",
        "Three boughs of holly",
        "Two candy canes",
        "And a song for the Christmas tree"
    ];
    // println!("Hello, world!");
    let mut count: u32 = 0;
    for c in (0..count){
        for element in song {
            println!("{}",element);
        }
        break
    }
}
