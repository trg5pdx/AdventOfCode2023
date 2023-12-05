fn convert_to_number(input: &Vec<u32>, len: u32) -> u32 {
    if input.is_empty() {
        return 0;
    }
    (*input)
        .clone()
        .into_iter()
        .rev()
        .zip(0_u32..len)
        .fold(0, |accum, (c, i)| accum + c * 10_u32.pow(i))
}

fn check_symbols(
    input: &[Vec<char>], 
    y_limit: usize, 
    x: usize, 
    y: usize, 
    num_len: usize,
    check: fn(char) -> bool,
) -> bool {
    let mut adjacent = false;
    let (mut x1, mut y1) = (0, 0);
    let (x2, mut y2) = (x, y);
    if x > num_len {
        x1 = x - num_len - 1;
    }     
    if y != 0 {
        y1 = y - 1;
    }      
    if y + 1 < y_limit {
        y2 = y + 1;
    }
    
    for j in y1..=y2 {
        for i in x1..=x2 {
            if check(input[j][i]) {
                adjacent = true;
            }
        }
    }
    adjacent
}

pub fn sum_gear_ratio(input: Vec<Vec<char>>) -> u32 {
    let (mut x, mut y) = (0, 0);
    let mut sum: u32 = 0;    
    let mut build_num = Vec::new();
    let (x_limit, y_limit) = (input[0].len(), input.len()); 

    while y < y_limit {
        if let Some(n) = input[y][x].to_digit(10) {
            build_num.push(n);
        }

        if x == x_limit - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }

    }

    0_u32
}

pub fn sum_part_numbers(input: Vec<Vec<char>>) -> u32 {
    let (mut x, mut y) = (0, 0);
    let mut sum: u32 = 0;
    let mut build_num = Vec::new();
    let (x_limit, y_limit) = (input[0].len(), input.len()); 
    
    while y < y_limit {
        if let Some(n) = input[y][x].to_digit(10) {
            build_num.push(n);
        } 
        if (!input[y][x].is_ascii_digit() || x + 1 == x_limit) && !build_num.is_empty() {
            let b_len = build_num.len();
            let conv = convert_to_number(&build_num, b_len.try_into().unwrap());
            build_num = Vec::new();
            if check_symbols(
                &input, 
                y_limit, 
                x, y, b_len, 
                |c| c != '.' && !c.is_ascii_digit()
            ) {
                sum += conv;
            }
        } 

        if x == x_limit - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }

    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sum_part_numbers() {
        let input: Vec<Vec<char>> = vec![
            "467..114..".chars().collect(),
            "...*......".chars().collect(),
            "..35..633.".chars().collect(),
            "......#...".chars().collect(),
            "617*......".chars().collect(),
            ".....+.58.".chars().collect(),
            "..592.....".chars().collect(),
            "......755.".chars().collect(),
            "...$.*....".chars().collect(),
            ".664.598..".chars().collect(),
        ];
        
        let result = sum_part_numbers(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn check_sum_gear_ratio() {
        let input: Vec<Vec<char>> = vec![
            "467..114..".chars().collect(),
            "...*......".chars().collect(),
            "..35..633.".chars().collect(),
            "......#...".chars().collect(),
            "617*......".chars().collect(),
            ".....+.58.".chars().collect(),
            "..592.....".chars().collect(),
            "......755.".chars().collect(),
            "...$.*....".chars().collect(),
            ".664.598..".chars().collect(),
        ];
        
        let result = sum_gear_ratio(input);

        assert_eq!(result, 467835);
    }
}
