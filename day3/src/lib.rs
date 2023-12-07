/* Not the biggest fan of how I did this problem, but I wanted to get it done
 * and move on to the next problem. I probably could have split up the code
 * more to reduce reuse but I was spending too much time thinking about that so 
 * I decided to just copy paste what I needed and edit it accordingly for part 2
 *
 * */
use std::collections::HashMap;

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
) -> Option<(usize, usize)> {
    let mut adjacent = None;
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
                adjacent = Some((i, j));
            }
        }
    }
    adjacent
}

pub fn sum_gear_ratio(input: &Vec<Vec<char>>) -> u32 {
    let (mut x, mut y) = (0, 0);
    let mut build_num = Vec::new();
    let (x_limit, y_limit) = (input[0].len(), input.len()); 
    
    let mut possible_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    while y < y_limit {
        if let Some(n) = input[y][x].to_digit(10) {
            build_num.push(n);
        }
        if (!input[y][x].is_ascii_digit() || x + 1 == x_limit) && !build_num.is_empty() {
            let b_len = build_num.len();
            let conv = convert_to_number(&build_num, b_len.try_into().unwrap());
            build_num = Vec::new();
            if let Some((i, j)) = check_symbols(
                &input, 
                y_limit, 
                x, y, b_len, 
                |c| c == '*'
            ) {
                let _ = match possible_gears.get(&(i, j)) {
                    Some(nums) => {
                        let mut new_nums = nums.clone();
                        new_nums.push(conv);
                        possible_gears.insert((i, j), new_nums)
                    },
                    None => possible_gears.insert((i, j), vec![conv]),
                };
            }
        } 

        if x == x_limit - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }

    }
    
    possible_gears
        .iter()
        .fold(0, |acc, ((_, _), nums)| {
            if nums.len() == 2 {
                return acc + nums[0] * nums[1];
            }
            acc
        })
}

pub fn sum_part_numbers(input: &Vec<Vec<char>>) -> u32 {
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
            ).is_some() {
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
        
        let result = sum_part_numbers(&input);

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
        
        let result = sum_gear_ratio(&input);

        assert_eq!(result, 467835);
    }
}
