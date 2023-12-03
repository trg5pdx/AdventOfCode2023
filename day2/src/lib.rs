#[derive(Debug)]
struct CubeInfo {
    red: u32,
    green: u32,
    blue: u32,
}

impl std::ops::AddAssign<CubeInfo> for CubeInfo {
    fn add_assign(&mut self, other: CubeInfo) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
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
    let mut cube = CubeInfo { red: 0, green: 0, blue: 0 };
     
    for i in input.split(' ').collect::<Vec<&str>>() {
        if let Ok(n) = i.parse() {
            num = n;
        }

        match i {
            "red" => cube.red = num,
            "green" => cube.green = num,
            "blue" => cube.blue = num,
            _ => (),
        }
    }
    cube
}

fn check_cubes(input: &str) -> bool {
    for set in input.split(';') {
        let mut cube_info = CubeInfo { red: 0, green: 0, blue: 0 };
        let cubes: Vec<&str> = set.split(',').collect();
        for cube in cubes {
            cube_info += parse_cubes(cube);
            if cube_info.red > 12 || cube_info.green > 13 || cube_info.blue > 14 {
                return false;
            }
        }
    }
    
    true
}

fn pow_min_cubes(input: &str) -> u32 {
    let mut min_cubes = CubeInfo { red: 0, green: 0, blue: 0 };
    let mut power = 1;
    for set in input.split(';') { 
        let mut current_game = CubeInfo { red: 0, green: 0, blue: 0 };
        for cube in set.split(',').collect::<Vec<&str>>() {
            current_game += parse_cubes(cube); 
        }
        
        if min_cubes.red < current_game.red {
            min_cubes.red = current_game.red;
        } 
        if min_cubes.green < current_game.green {
            min_cubes.green = current_game.green;
        } 
        if min_cubes.blue < current_game.blue {
            min_cubes.blue = current_game.blue;
        }
    }
    
    if min_cubes.red != 0 {
        power *= min_cubes.red;
    } 
    if min_cubes.green != 0 {
        power *= min_cubes.green;
    }
    if min_cubes.blue != 0 {
        power *= min_cubes.blue;
    }

    power
}

pub fn sum_of_pow_min_cubes(input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for game in input {
        let game_info: Vec<&str> = game.split(':').collect();
        sum += pow_min_cubes(game_info[1]);
    }

    sum
}

pub fn sum_id_possible_games(input: &Vec<String>) -> u32 {
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
    fn five_games_sum_id_possible_games() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
        ];

        let result = sum_id_possible_games(&input);
        assert_eq!(8, result);
    }

    #[test]
    fn five_games_sum_pow_min_cubes() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
            
        ];

        let result = sum_of_pow_min_cubes(&input);
        assert_eq!(2286, result);
    }
}
