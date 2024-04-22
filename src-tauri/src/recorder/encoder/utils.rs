pub fn flip_image_vertical_bgra(buffer: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let bytes_per_pixel = 4;
    let mut flipped_image = Vec::with_capacity(width * height * bytes_per_pixel);

    for y in (0..height).rev() {
        let row_start = y * width * bytes_per_pixel;
        let row_end = row_start + (width * bytes_per_pixel);
        flipped_image.extend_from_slice(&buffer[row_start..row_end]);
    }

    flipped_image
}
