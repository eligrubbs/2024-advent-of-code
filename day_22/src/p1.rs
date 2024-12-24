use std::env;
use std::fs::read_to_string;

pub fn day_22_p1_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut secs: Vec<u64> = parse_input(&raw);

    calc_x_next_for_all(&mut secs, 2000);
    secs.iter().sum()
}

pub fn parse_input(content: &str) -> Vec<u64> {
    content.lines().map(|num| num.parse::<u64>().unwrap()).collect()
}

pub fn calc_x_next_for_all(secs: &mut Vec<u64>, times: u32) {
    for _ in 0..times {
        calc_next_for_all(secs);
    }
}

pub fn calc_next_for_all(secs: &mut Vec<u64>) {
    *secs = secs.iter().map(|&num| next_secret(num)).collect();
}

pub fn next_secret(secret: u64) -> u64 {
    let mut secret: u64 = ( (secret << 6) ^ secret) & 16777215;
    secret = ( (secret >> 5) ^ secret) & 16777215;
    ( (secret << 11) ^ secret) & 16777215
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_secret() {
        let mut secret: u64 = 123;
        let next_secrets: Vec<u64> = vec![15887950,16495136, 527345,
        704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254];

        for verify in next_secrets {
            secret = next_secret(secret);
            assert_eq!(verify, secret);
        }
    }
}