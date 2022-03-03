use std::{cmp, error::Error};

pub fn generate(
    x_size: u32,
    y_size: u32,
) -> Result<image::ImageBuffer<image::Luma<u8>, Vec<u8>>, Box<dyn Error>> {
    let mut img = image::ImageBuffer::new(x_size, y_size);
    let total = u64::from(cmp::max(x_size, y_size)).pow(2).try_into()?;
    let sieve = primal::Sieve::new(total);
    let pixel = image::Luma::from([255]);

    for prime in sieve.primes_from(0).take_while(|x| *x <= total) {

        let coord = crate::calc_coord::calc(prime.try_into()?);
        let pos_x: i64 = i64::from(x_size) - i64::from(x_size) / 2 + i64::from(coord.x);
        let pos_y: i64 = (i64::from(y_size) / 2 - i64::from(coord.y)) + 1;
        if pos_x < 1
            || u32::try_from(pos_x)? > x_size
            || pos_y < 1
            || u32::try_from(pos_y)? > y_size
        {
            // A rectangle is being generated and this prime lies outside it.
            continue;
        }
        img.put_pixel(u32::try_from(pos_x)? - 1, u32::try_from(pos_y)? - 1, pixel);
    }

    Ok(img)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_sq() {
        let odd = generate(5, 5).unwrap();
        let mut expected_odd = image::ImageBuffer::new(5, 5);
        expected_odd.put_pixel(3, 0, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(0, 1, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(2, 1, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(3, 1, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(4, 1, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(1, 2, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(0, 3, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(2, 3, image::Luma::from([255 as u8]));
        expected_odd.put_pixel(3, 4, image::Luma::from([255 as u8]));
        assert_eq!(odd, expected_odd);

        let even = generate(4, 4).unwrap();
        let mut expected_even = image::ImageBuffer::new(4, 4);
        expected_even.put_pixel(2, 0, image::Luma::from([255 as u8]));
        expected_even.put_pixel(1, 1, image::Luma::from([255 as u8]));
        expected_even.put_pixel(2, 1, image::Luma::from([255 as u8]));
        expected_even.put_pixel(3, 1, image::Luma::from([255 as u8]));
        expected_even.put_pixel(0, 2, image::Luma::from([255 as u8]));
        expected_even.put_pixel(1, 3, image::Luma::from([255 as u8]));
        assert_eq!(even, expected_even);
    }
    #[test]
    fn check_bad() {
        let bad = generate(0, 1).unwrap();
        let expected = image::ImageBuffer::new(0, 1);
        assert_eq!(bad, expected);
    }
    #[test]
    fn check_rect() {
        let tall = generate(5, 3).unwrap();
        let mut expected_tall = image::ImageBuffer::new(5, 3);
        expected_tall.put_pixel(0, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(2, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(3, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(4, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(1, 1, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(0, 2, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(2, 2, image::Luma::from([255 as u8]));
        assert_eq!(tall, expected_tall);

        let wide = generate(3, 5).unwrap();
        let mut expected_wide = image::ImageBuffer::new(3, 5);
        expected_wide.put_pixel(2, 0, image::Luma::from([255 as u8]));
        expected_wide.put_pixel(1, 1, image::Luma::from([255 as u8]));
        expected_wide.put_pixel(2, 1, image::Luma::from([255 as u8]));
        expected_wide.put_pixel(0, 2, image::Luma::from([255 as u8]));
        expected_wide.put_pixel(1, 3, image::Luma::from([255 as u8]));
        expected_wide.put_pixel(2, 4, image::Luma::from([255 as u8]));
        assert_eq!(wide, expected_wide);
    }
}
