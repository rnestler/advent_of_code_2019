#![allow(incomplete_features)]
#![feature(const_generics)]

use std::fs::read;
use std::io::Read;
use std::mem::MaybeUninit;

fn create_layer<const W: usize, const H: usize>(input: &mut &[u8]) -> [[u8; W]; H] {
    let mut result: [[u8; W]; H] = unsafe { MaybeUninit::uninit().assume_init() };
    for r in 0..H {
        let row = &mut result[r];
        input.read_exact(row).expect("Reading failed");
        row.iter_mut().for_each(|e| *e -= b'0');
    }
    result
}

fn create_layers<const W: usize, const H: usize>(mut input: &[u8]) -> Vec<[[u8; W]; H]> {
    let layers = input.len() / (W * H);
    let mut result = vec![]; // create_layer(width, height); layers];

    for _ in 0..layers {
        result.push(create_layer::<W, H>(&mut input));
    }
    result
}

fn find_layer_with_fewest_zero<const W: usize, const H: usize>(
    layers: &Vec<[[u8; W]; H]>,
) -> &[[u8; W]; H] {
    layers
        .iter()
        .min_by(|l, r| count_elems(l, 0).cmp(&count_elems(r, 0)))
        .unwrap()
}

fn count_elems<const W: usize, const H: usize>(layer: &[[u8; W]; H], what: u8) -> usize {
    layer
        .iter()
        .map(|row| row.iter().filter(|b| **b == what).count())
        .sum()
}

fn combine_layers<const W: usize, const H: usize>(top: &mut [[u8; W]; H], bottom: &[[u8; W]; H]) {
    for (t_row, b_row) in top.iter_mut().zip(bottom.iter()) {
        for (t, b) in t_row.iter_mut().zip(b_row.iter()) {
            *t = if *t == 2 { *b } else { *t }
        }
    }
}

fn stack_layers<const W: usize, const H: usize>(layers: &Vec<[[u8; W]; H]>) -> [[u8; W]; H] {
    let mut output = layers[0].clone();
    for layer in &layers[1..] {
        combine_layers(&mut output, layer);
    }
    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read("input.txt")?;
    let layers = create_layers::<25, 6>(&content);
    let layer = find_layer_with_fewest_zero(&layers);
    let ones = count_elems(layer, 1);
    let twos = count_elems(layer, 2);

    println!("Result part 1: {}", ones * twos);

    let image = stack_layers(&layers);

    for row in &image {
        for pixel in row {
            if *pixel == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_layer() {
        let input = b"123456789012";
        let layer = create_layer::<3, 2>(&mut &input[..]);
        let expected = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(expected, layer);
    }

    #[test]
    fn test_create_layers() {
        let input = b"123456789012";
        let layers = create_layers::<3, 2>(input);
        let expected = vec![[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [0, 1, 2]]];
        assert_eq!(expected, layers);
    }

    #[test]
    fn test_find_layer_with_fewest_zero() {
        let input = b"123456789012";
        let layers = create_layers::<3, 2>(input);
        let layer = find_layer_with_fewest_zero(&layers);
        let expected = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(&expected, layer);
    }
}
