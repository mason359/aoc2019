use crate::Day;
use crate::utils;

pub struct Day8;

impl Day for Day8 {

    fn problem1(&self) -> i64 {
        const W: usize = 25;
        const H: usize = 6;
        const LAYER_SIZE: usize = W * H;
        let input = utils::get_input_string(8);
        let image: Vec<i32> = input
            .chars()
            .map(|char| char.to_digit(10).unwrap() as i32)
            .collect();
        let mut digits = (count_digits(&image, 0), 0, 0);
        let mut zeros;
        for i in (0..image.len()).step_by(LAYER_SIZE) {
            zeros = count_digits(&image[i..(i + LAYER_SIZE)], 0);
            if zeros < digits.0 {
                digits.0 = zeros;
                digits.1 = count_digits(&image[i..(i + LAYER_SIZE)], 1);
                digits.2 = count_digits(&image[i..(i + LAYER_SIZE)], 2);
            }
        }
        (digits.1 * digits.2) as i64
    }
    
    fn problem2(&self) -> i64 {
        const W: usize = 25;
        const H: usize = 6;
        const LAYER_SIZE: usize = W * H;
        let input = utils::get_input_string(8);
        let image: Vec<i32> = input
            .chars()
            .map(|char| char.to_digit(10).unwrap() as i32)
            .collect();
        let decoded: Vec<i32> = (0..LAYER_SIZE)
            .map(|i| {
                *image
                    .iter()
                    .skip(i)
                    .step_by(LAYER_SIZE)
                    .filter(|pixel| **pixel != 2)
                    .next()
                    .unwrap()
            })
            .collect();
        display_message(decoded, W, H);
        0
    }

}

fn count_digits(image: &[i32], digit: i32) -> i32 {
    image
        .iter()
        .filter(|num| **num == digit)
        .count()
        as i32
}

fn display_message(image: Vec<i32>, width: usize, height: usize) {
    for i in (0..(height * width)).step_by(width) {
        let line: String = image[i..(i + width)]
            .iter()
            .map(|digit| if *digit == 1 { '█' } else { ' ' })
            .collect::<String>();
        println!("{}", line);
    }
}