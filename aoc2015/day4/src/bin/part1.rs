fn main() {
    let puzzle = "iwrupvqb";
    for i in 0.. {
        let puzzle_number = String::from(puzzle) + &i.to_string();
        let md5 = md5::compute(&puzzle_number);
        if hex::encode(*md5).to_string().starts_with("000000") {
            dbg!(puzzle_number, md5);
            break;
        }
    }   
}