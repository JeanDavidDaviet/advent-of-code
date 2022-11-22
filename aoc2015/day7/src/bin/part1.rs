#[derive(Debug)]
enum Ops {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
}

fn parse(line: &str) -> Vec<&str> {
    let line = line.replace(" -> ", "|");
    let statements: Vec<&str> = line.split('|').collect();
    statements
}

fn get_tokens(line: &str) {
    let tokens: Vec<&str> = line.split(' ').collect();
    for (i, token) in tokens.iter().enumerate() {
        if token.parse::<i32>().is_ok() {
            let value = token.parse::<i32>().unwrap();
            dbg!(value);
        } else if token.len() == 1 {
            let var = token.chars().next().unwrap();
            dbg!(var);
        } else if token == &"->" {
            let dest = token;
            let dest_var = &tokens.get(i + 1).expect("no destination variable");
            dbg!(dest, dest_var);
        } else {
            let op = match token {
                &"AND" => Some(Ops::AND),
                &"OR" => Some(Ops::OR),
                &"NOT" => Some(Ops::NOT),
                &"LSHIFT" => Some(Ops::LSHIFT),
                &"RSHIFT" => Some(Ops::RSHIFT),
                _ => None,
            };
            dbg!(op.unwrap());
        }
    }
}

fn process(a: i32, b: i32, op: Ops) -> i32 {
    match op {
        Ops::AND => a & b,
        Ops::OR => a | b,
        Ops::NOT => ! a,
        Ops::LSHIFT => a >> b,
        Ops::RSHIFT => a << b,
    }
}

fn main() {
    let input = String::from("123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i");

    for line in input.lines() {
        let statements = parse(line);
        // let tokens = get_tokens(line);
    }
}
