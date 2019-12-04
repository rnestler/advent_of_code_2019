fn check_pw(pw: u32) -> bool {
    let mut old_digit = b'0';
    let mut two_conservative_same = false;
    let mut digits_increasing = true;
    let pw_str = format!("{}", pw).into_bytes();
    for d in 0..6 {
        if pw_str[d] == old_digit {
            two_conservative_same = true;
        }
        if pw_str[d] < old_digit {
            digits_increasing = false
        }
        old_digit = pw_str[d];
    }
    two_conservative_same && digits_increasing
}

fn check_pw_part2(pw: u32) -> bool {
    let mut consecutive_count = [0; 10];
    let pw_str = format!("{}", pw).into_bytes();
    for digit in 0..10u8 {
        for d in 0..6 {
            if pw_str[d] == digit + b'0' {
                consecutive_count[digit as usize] += 1;
            }
        }
    }
    consecutive_count.contains(&2)
}

fn main() {
    let part1: Vec<_> = (372037..905157).filter(move |pw| check_pw(*pw)).collect();
    println!("result part1: {}", part1.len());

    println!(
        "result part2: {}",
        part1.iter().filter(|pw| check_pw_part2(**pw)).count()
    );
}
