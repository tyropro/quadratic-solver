fn tokenisation(input: String) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut tokens: Vec<f32> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();

    let mut last_char: char = ' ';
    // ignore difference of 2 squares, if no x return invalid and exit program
    // 2x^2 + 3x - 4
    // 1/2x^2
    // [ 2, 3, 4]
    for c in input.chars() {
        if !(last_char == '^' || (last_char == '-' && c == ' ')) {
            if c.is_digit(10) || c == '.' || c == '-' {
                buffer.push(c);
            } else {
                last_char = c;
                if buffer.len() == 0 {
                    continue;
                }
                
                let token = buffer.iter().collect::<String>().parse::<f32>().unwrap();

                tokens.push(token);
                buffer.clear();
            }
        }
        last_char = c;
    }

    return Ok(tokens)
}

fn discriminant(a: f32, b: f32, c: f32) -> f32 {
    (b.powf(2.) - 4.*a*c).sqrt()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("unable to read string");
    input = input.trim().to_string();

    let token_string = format!("{} ", input);
    let tokens = tokenisation(token_string).expect("unable to parse args");

    let a = tokens[0];
    let b = tokens[1];
    let c = tokens[2];

    let descrim = discriminant(a, b, c);

    if descrim.is_nan() {
        println!("No roots!");
        return;
    }

    if descrim == 0. {
        let root = (-b + descrim) / (2.*a);
        println!("Repeated root\nx = {}", root);
        return;
    }

    if descrim > 0. {
        let root_1 = (-b + descrim) / (2.*a);
        let root_2 = (-b - descrim) / (2.*a);

        println!("x = {}\nx = {}", root_1, root_2);
    }
}
