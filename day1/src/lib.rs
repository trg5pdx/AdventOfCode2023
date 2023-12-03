pub fn calibration_sum(lines: Vec<String>) -> u64 {
    let mut sum: u64 = 0; 

    for l in lines {
        let mut left: u64 = 0;
        let mut right: u64 = 0;
        for c in l.chars() {
            if let Some(digit) = c.to_digit(10) {
                if left == 0 {
                    left = digit.into();
                } else {
                    right = digit.into();
                }
            }
        }
        if left != 0 && right == 0 {
            sum += left * 10 + left;
        } else if left != 0 && right != 0 {
            sum += left * 10 + right;
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let mut input = Vec::new();
        input.push(String::from("1abc2"));
        input.push(String::from("pqr3stu8vwx"));
        input.push(String::from("a1b2c3d4e5f"));
        input.push(String::from("treb7uchet"));

        let result = calibration_sum(input);

        assert_eq!(result, 142);
    }
}
