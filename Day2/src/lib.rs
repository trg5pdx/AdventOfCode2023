enum CubeInfo {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn get_game_id(input: &str) -> u32 {
    let mut id = 0;
    let game_info: Vec<&str> = input.split(' ').collect();
     
    for (i, c) in game_info[1].chars().rev().enumerate() {
        if let Some(n) = c.to_digit(10) {
            let power: u32 = i.try_into().unwrap();
            let digit_place: u32 = 10_i32.pow(power).try_into().unwrap();
            id += n * digit_place;
        }
    }

    id
}

fn parse_cubes(input: &str) -> CubeInfo {
    let mut num = 0;
    let mut cube = CubeInfo::Red(0);
    
    for i in input.split(' ').collect::<Vec<&str>>() {
        match i.parse() {
            Ok(n) => num = n,
            Err(_) => (),
        }

        match i {
            "red" => cube = CubeInfo::Red(num),
            "blue" => cube = CubeInfo::Blue(num),
            "green" => cube = CubeInfo::Green(num),
            _ => cube = CubeInfo::Red(0),
        }
    }
    cube
}

fn check_cubes(input: &str) -> bool {
    for set in input.split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let cubes: Vec<&str> = set.split(',').collect();
        for cube in cubes {
            let info = parse_cubes(cube);
            match info {
                CubeInfo::Red(i) => red += i,
                CubeInfo::Green(i) => green += i,
                CubeInfo::Blue(i) => blue += i,
            }
            
            if red > 12 || green > 13 || blue > 14 {
                return false;
            }
        }
    }
    
    true
}

pub fn sum_id_possible_games(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for game in input {
        let game_info: Vec<&str> = game.split(':').collect();
        let game_id = get_game_id(game_info[0]);
        if check_cubes(game_info[1]) {
            sum += game_id;
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_games() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
        ];

        let result = sum_id_possible_games(input);
        assert_eq!(8, result);
    }
}
