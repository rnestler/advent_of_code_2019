use std::fs::read;
use std::io::Read;

fn create_layer(input: &mut &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut result = vec![];
    for _ in 0..height {
        let mut row = vec![0u8; width];
        input.read_exact(&mut row).expect("Reading failed");
        row.iter_mut().for_each(|e| *e -= b'0');
        result.push(row);
    }
    result
}

fn create_layers(mut input: &[u8], width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
    let layers = input.len() / (width * height);
    let mut result = vec![]; // create_layer(width, height); layers];

    for _ in 0..layers {
        result.push(create_layer(&mut input, width, height));
    }
    result
}

fn find_layer_with_fewest_zero(layers: &Vec<Vec<Vec<u8>>>) -> &Vec<Vec<u8>> {
    layers
        .iter()
        .min_by(|l, r| count_elems(l, 0).cmp(&count_elems(r, 0)))
        .unwrap()
}

fn count_elems(layer: &Vec<Vec<u8>>, what: u8) -> usize {
    layer
        .iter()
        .map(|row| row.iter().filter(|b| **b == what).count())
        .sum()
}

fn combine_layers(top: &mut Vec<Vec<u8>>, bottom: &Vec<Vec<u8>>) {
    for (t_row, b_row) in top.iter_mut().zip(bottom.iter()) {
        for (t, b) in t_row.iter_mut().zip(b_row.iter()) {
            *t = if *t == 2 { *b } else { *t }
        }
    }
}

fn stack_layers(layers: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let mut output = layers[0].clone();
    for layer in &layers[1..] {
        combine_layers(&mut output, layer);
    }
    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read("input.txt")?;
    let layers = create_layers(&content, 25, 6);
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
        let layer = create_layer(&mut &input[..], 3, 2);
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(expected, layer);
    }

    #[test]
    fn test_create_layers() {
        let input = b"123456789012";
        let layers = create_layers(input, 3, 2);
        let expected = vec![
            vec![vec![1, 2, 3], vec![4, 5, 6]],
            vec![vec![7, 8, 9], vec![0, 1, 2]],
        ];
        assert_eq!(expected, layers);
    }

    #[test]
    fn test_find_layer_with_fewest_zero() {
        let input = b"123456789012";
        let layers = create_layers(input, 3, 2);
        let layer = find_layer_with_fewest_zero(&layers);
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(&expected, layer);
    }
}
