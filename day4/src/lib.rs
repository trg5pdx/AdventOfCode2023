fn get_winning_numbers(input: &str) -> Vec<u32> {
    
    let mut winning_numbers = Vec::new();
    for number in input.split(' ') {
        if let Ok(num) = number.parse() {
            winning_numbers.push(num);
        }
    }
    
    winning_numbers
}

fn parse_cards(input: &String) -> (Vec<u32>, Vec<u32>) { 
    let card: Vec<&str> = input.split('|').collect();
    let winning_numbers = get_winning_numbers(card[0]);
    let my_numbers: Vec<u32> = card[1].split(' ').map(|n| {
        match n.parse() {
            Ok(i) => i,
            Err(_) => 0,
        }
    }).collect();

    (winning_numbers, my_numbers)
}

/* Could first check how many matches there are per card and keep note of it
 * for use later in calculation of the number of cards won
 * */
pub fn number_of_cards_won(input: &Vec<String>) -> usize {
    let mut wins = 0;
    // value: # of matches on that card and the number of copies in that row
    let cards: usize = input.len();
    let mut card_wins_copies: Vec<(usize, usize)> = Vec::new(); 


    for lines in input.iter() {
        let (winning_numbers, my_numbers) = parse_cards(lines);

        let mut matches = 0;
        for i in my_numbers {
            for j in &winning_numbers {
                if i == *j {
                    matches += 1; 
                }
            }
        }
        card_wins_copies.push((matches, 1));
    }
    
    let set_increment = |current: usize, matches: usize, copies: usize, end: usize| {
        if current + matches > end {
            return ((matches * copies) / (end - current), end - current);
        }
        (1 * copies, current + matches)
    };

    for i in 0..cards {
        let (curr_matches, copies) = card_wins_copies[i];
        wins += copies;
        let (incr, len) = set_increment(i, curr_matches, copies, cards);
        for j in i..=len {
            let (m, c) = card_wins_copies[j];
            card_wins_copies[j] = (m, c + incr);
        }
    }
    wins
}

pub fn sum_of_winning_cards(input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
     
    for line in input {
        let mut points: u32 = 0;
        let (winning_numbers, my_numbers) = parse_cards(line);

        for i in my_numbers {
            for j in &winning_numbers {
                if i == *j {
                    if points == 0 {
                        points += 1;
                    } else {
                        points *= 2;
                    }
                }
            }
        }
        sum += points;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sum_of_winning_cards() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        assert_eq!(13, sum_of_winning_cards(&input));
    }

    #[test]
    fn check_number_of_cards_won() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        assert_eq!(30, number_of_cards_won(&input));
    }
}
