use std::fs;

fn get_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let mut sizes: Vec<u32> = vec![l, w, h];
    sizes.sort_unstable();
    let mins: (u32, u32) = (sizes[0], sizes[1]);
    let extra: u32 = mins.0 + mins.0 + mins.1 + mins.1;
    let cubic: u32 = sizes.iter().product();
    extra + cubic
}

fn get_ribbon_from_line(line: &str) -> u32 {
    let values_as_string: Vec<&str> = line.split('x').collect();
    
    get_ribbon(
        values_as_string[0].parse::<u32>().unwrap_or(0),
        values_as_string[1].parse::<u32>().unwrap_or(0),
        values_as_string[2].parse::<u32>().unwrap_or(0),
    )
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap_or_default();
    let mut ribbon: u32 = 0;
    for line in input.lines() {
        ribbon += get_ribbon_from_line(&String::from(line));
    }
    dbg!(ribbon);
}
