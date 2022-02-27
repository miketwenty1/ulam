pub fn generate(x_size: u32, y_size: u32) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    if x_size == 0 || y_size == 0 {
        return image::ImageBuffer::new(0, 0);
    }
    // assert!(x_size > 0 && y_size > 0, "inputs must be greater than 0");

    let mut img = image::ImageBuffer::new(x_size, y_size);
    let x_size = x_size as i32;
    let y_size = y_size as i32;

    for x in (0..x_size).map(|x| x - x_size / 2) {
        for y in (0..y_size).map(|y| y - y_size / 2) {
            let coord = crate::Coord::new(x, y);
            let deets = crate::get_ulam_deets(&coord);
            let value = deets.value;
            if primal::is_prime(value as u64) {
                let pixel = image::Luma::from([255 as u8]);
                img.put_pixel(
                    (x_size / 2 + x) as u32,
                    ((y_size - y_size / 2) - y - 1) as u32,
                    pixel,
                );
            }
        }
    }
    img
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_sq() {
        let odd = generate(5, 5);
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

        let even = generate(4, 4);
        let mut expected_even = image::ImageBuffer::new(4, 4);
        expected_even.put_pixel(0, 0, image::Luma::from([255 as u8]));
        expected_even.put_pixel(2, 0, image::Luma::from([255 as u8]));
        expected_even.put_pixel(3, 0, image::Luma::from([255 as u8]));
        expected_even.put_pixel(1, 1, image::Luma::from([255 as u8]));
        expected_even.put_pixel(0, 2, image::Luma::from([255 as u8]));
        expected_even.put_pixel(2, 2, image::Luma::from([255 as u8]));
        expected_even.put_pixel(3, 3, image::Luma::from([255 as u8]));
        assert_eq!(even, expected_even);
    }
    #[test]
    fn check_bad() {
        let bad = generate(0, 1);
        let img = image::ImageBuffer::new(0, 0);
        assert_eq!(bad, img);
    }
    #[test]
    fn check_rect() {
        let tall = generate(5, 3);
        let mut expected_tall = image::ImageBuffer::new(5, 3);
        expected_tall.put_pixel(0, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(2, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(3, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(4, 0, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(1, 1, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(0, 2, image::Luma::from([255 as u8]));
        expected_tall.put_pixel(2, 2, image::Luma::from([255 as u8]));
        assert_eq!(tall, expected_tall);

        let wide = generate(3, 5);
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
