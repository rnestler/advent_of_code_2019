use std::fs::read;
const START_PATTERN: &[i8] = &[0, 1, 0, -1];

fn generate_patterns(n: usize) -> Vec<Vec<i8>> {
    let mut patterns = vec![];
    patterns.reserve(n);
    for i in 0..n {
        let pattern: Vec<i8> = START_PATTERN
            .iter()
            .flat_map(|d| std::iter::repeat(d).take(i + 1))
            .cycle()
            .skip(1)
            .take(n)
            .copied()
            .collect();
        patterns.push(pattern);
    }
    patterns
}

fn apply_phase(patterns: &Vec<Vec<i8>>, input: &Vec<i8>) -> Vec<i8> {
    let mut output = vec![0i8; input.len()];
    for i in 0..input.len() {
        let pattern = patterns[i].iter().cycle();
        let sum = input
            .iter()
            .zip(pattern)
            .map(|(a, b)| a * b)
            .fold(0i32, |acc, x| acc + x as i32);
        output[i] = (sum % 10).abs() as i8;
    }
    output
}

fn apply_phases(patterns: &Vec<Vec<i8>>, mut input: Vec<i8>, n: usize) -> Vec<i8> {
    for _ in 0..n {
        input = apply_phase(patterns, &input);
    }
    input
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read("input.txt")?;
    let input: Vec<i8> = content
        .iter()
        .copied()
        .filter(u8::is_ascii_digit)
        .map(|v| (v - b'0') as i8)
        .collect();

    let patterns = generate_patterns(input.len());
    let output = apply_phases(&patterns, input, 100);
    println!("Result part 1: {:?}", &output[0..8]);

    /*
    let input: Vec<i8> = input
        .iter()
        .cycle()
        .copied()
        .take(input.len() * 10000)
        .collect();
    let patterns = generate_patterns(input.len());
    */
    //let output = apply_phases(&patterns, input, 100);
    //println!("Result part 2: {:?}", &output[0..8]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_patterns() {
        let patterns = generate_patterns(5);
        assert_eq!(
            patterns,
            vec![
                vec![1, 0, -1, 0, 1],
                vec![0, 1, 1, 0, 0],
                vec![0, 0, 1, 1, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 0, 1],
            ]
        );
    }

    #[test]
    fn test_apply_phase() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let patterns = generate_patterns(input.len());
        let output = apply_phase(&patterns, &input);
        assert_eq!(output, &[4, 8, 2, 2, 6, 1, 5, 8]);
    }
}
