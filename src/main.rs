use std::io::{self};

fn pow_end(power: u64) -> i32 {
    match power {
        1 => 5,
        _ => 25
    }
}

fn main() {
    let stdio = io::stdin();
    let mut buffer = String::new();

    stdio.read_line(&mut buffer).unwrap();

    let number: u64 = buffer.trim_end().parse().unwrap();

    println!("{}", pow_end(number));
}

#[cfg(test)]
mod tests {
    use crate::pow_end;

    #[test]
    fn test_1() {
        assert_eq!(pow_end(1), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(pow_end(2), 25);
    }

    #[test]
    fn test_3() {
        assert_eq!(pow_end(200), 25);
    }
}
