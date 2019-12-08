use std::fs::read;
use std::io::Read;

fn create_layer(mut input: &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut result = vec![];
    for _ in 0..height {
        let mut row = vec![0u8; width];
        input.read_exact(&mut row).expect("Reading failed");
        result.push(row);
    }
    result
}

fn create_layers(mut input: &[u8], width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
    let layers = input.len() / (width * height);
    let mut result = vec![]; // create_layer(width, height); layers];

    for _ in 0..layers {
        result.push(create_layer(input, width, height));
        input = &input[width * height..];
    }
    result
}

fn find_layer_with_fewest_zero(layers: &Vec<Vec<Vec<u8>>>) -> &Vec<Vec<u8>> {
    layers
        .iter()
        .min_by(|l, r| count_elems(l, b'0').cmp(&count_elems(r, b'0')))
        .unwrap()
}

fn count_elems(layer: &Vec<Vec<u8>>, what: u8) -> usize {
    layer
        .iter()
        .map(|row| row.iter().filter(|b| **b == what).count())
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read("input.txt")?;
    let layers = create_layers(&content, 25, 6);
    let layer = find_layer_with_fewest_zero(&layers);
    let ones = count_elems(layer, b'1');
    let twos = count_elems(layer, b'2');

    println!("Result day 1: {}", ones * twos);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_layer() {
        let input = b"123456789012";
        let layer = create_layer(input, 3, 2);
        let expected = vec![vec![b'1', b'2', b'3'], vec![b'4', b'5', b'6']];
        assert_eq!(expected, layer);
    }

    #[test]
    fn test_create_layers() {
        let input = b"123456789012";
        let layers = create_layers(input, 3, 2);
        let expected = vec![
            vec![vec![b'1', b'2', b'3'], vec![b'4', b'5', b'6']],
            vec![vec![b'7', b'8', b'9'], vec![b'0', b'1', b'2']],
        ];
        assert_eq!(expected, layers);
    }

    #[test]
    fn test_find_layer_with_fewest_zero() {
        let input = b"123456789012";
        let layers = create_layers(input, 3, 2);
        let layer = find_layer_with_fewest_zero(&layers);
        let expected = vec![vec![b'1', b'2', b'3'], vec![b'4', b'5', b'6']];
        assert_eq!(&expected, layer);
    }
}
