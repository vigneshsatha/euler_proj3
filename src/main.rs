fn is_prime(n: u128) -> bool {
    let mut i = 2;
    while i < n {
        if n % i == 0{
            return false;
        }
        i+= 1;
    }
    return true;
}

fn get_next_pime(mut n: u128) -> u128 {
    n+=1;
    while !is_prime(n) {
        n+=1;
    }
    return n;
}

fn find_lg_prime_factor(n: u128) -> u128 {
    let mut dividend = n;
    let mut divisor = 2;
    let mut found = false;
    let mut lg_prime_factor = 0;
    
    while !found {
        if dividend % divisor == 0 && dividend != divisor {
            dividend = dividend/divisor;
        } else if dividend != divisor {
            divisor = get_next_pime(divisor);
        } else {
            lg_prime_factor = dividend;
            found = true;
        }
    }
    return lg_prime_factor;
}

fn main() {
    println!("{}", find_lg_prime_factor(600851475143));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(2), "Should return true for 2");
        assert_eq!(true, is_prime(3), "Should return true for 3");
        assert_eq!(false, is_prime(4), "Should return false for 4");
        assert_eq!(true, is_prime(5), "Should return true for 5");
        assert_eq!(false, is_prime(6), "Should return false for 6");
        assert_eq!(true, is_prime(7), "Should return true for 7");
        assert_eq!(false, is_prime(8), "Should return false for 8");
        assert_eq!(false, is_prime(9), "Should return false for 9");
    }

    #[test]
    fn test_get_next_prime(){
        assert_eq!(3, get_next_pime(2), "Should return 3");
        assert_eq!(5, get_next_pime(3), "Should return 5");
        assert_eq!(7, get_next_pime(5), "Should return 7");
        assert_eq!(11, get_next_pime(7), "Should return 11");
    }
}