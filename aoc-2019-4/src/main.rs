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

fn main() {
    let mut count = 0;
    for pw in 372037..905157 {
        if check_pw(pw) {
            count += 1;
        }
    }
    println!("result part1: {}", count);
}
