fn is_prime(number: i32) -> bool {
    // deal with simple cases first
    if number <= 1 {
        return false;
    }
    if number <= 3 {
        return true;
    }

    // deal with multiples of 2 and 3
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    // test divisors of the form 6k ± 1 up to sqrt(number)
    // complexity of O(√n)
    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("{}", is_prime(7)); // true
    println!("{}", is_prime(10)); // false
    println!("{}", is_prime(13)); // true
    println!("{}", is_prime(1)); // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        // Nombres non premiers
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(!is_prime(15));
        assert!(!is_prime(25));

        // Nombres premiers
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(is_prime(29));
        assert!(is_prime(97));
    }
}
