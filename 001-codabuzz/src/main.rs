fn coda_buzz(number: i32) -> String {
    // using pattern matching to determine the output
    match (number % 3, number % 5) {
        (0, 0) => "CodaBuzz".to_string(),
        (0, _) => "Coda".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => number.to_string(),
    }
}

fn main() {
    for i in 1..=100 {
        println!("{}", coda_buzz(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coda_buzz() {
        assert_eq!(coda_buzz(1), "1");
        assert_eq!(coda_buzz(3), "Coda");
        assert_eq!(coda_buzz(5), "Buzz");
        assert_eq!(coda_buzz(15), "CodaBuzz");
        assert_eq!(coda_buzz(30), "CodaBuzz");
        assert_eq!(coda_buzz(7), "7");
        assert_eq!(coda_buzz(9), "Coda");
        assert_eq!(coda_buzz(10), "Buzz");
        assert_eq!(coda_buzz(45), "CodaBuzz");
        assert_eq!(coda_buzz(100), "Buzz");
    }
}
