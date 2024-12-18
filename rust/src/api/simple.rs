use image::{self, DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::time::Instant;
use mozjpeg::{ColorSpace, Compress};
use std::panic::catch_unwind;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

/// Merges multiple images vertically, then compresses the result using JPEG encoding.
/// An optional max size in KB can be specified.
///
/// * `image_buffers` - A list of byte vectors, each representing an image (PNG/JPEG, etc.).
/// * `max_size_kb` - Optional max size in KB. Attempts to stay within limit but not guaranteed.
///
/// Returns an `Option<Vec<u8>>` containing the compressed bytes if successful,
/// or `None` if loading/compressing fails.
pub fn merge_images_vertically(
    image_buffers: &[Vec<u8>],
    max_size_kb: Option<u64>,
) -> Option<Vec<u8>> {
    let start_time = Instant::now();

    // 1. Quick check: if there's no input, return None.
    if image_buffers.is_empty() {
        return None;
    }

    // 2. Load images in parallel and filter out any invalid data.
    let images: Vec<DynamicImage> = image_buffers
        .par_iter()
        .filter_map(|bytes| {
            image::load_from_memory(bytes)
                .map_err(|err| {
                    eprintln!("Failed to load image from bytes: {}", err);
                    err
                })
                .ok()
        })
        .collect();

    // If all data was invalid, return None.
    if images.is_empty() {
        return None;
    }

    // 3. Find the maximum width among all images, and sum up their heights.
    let final_width = images.par_iter().map(|img| img.width()).max().unwrap();
    let total_height: u32 = images.par_iter().map(|img| img.height()).sum();

    // 4. Allocate a new ImageBuffer for the merged image with a white background.
    let mut merged_buffer =
        ImageBuffer::from_pixel(final_width, total_height, Rgba([255, 255, 255, 255]));

    // 5. Convert all images to RGBA8 and prepare for merging.
    let rgba_images: Vec<_> = images.par_iter().map(|img| img.to_rgba8()).collect();
    let mut current_offset_y = 0;

    // Merge images one by one onto the newly created ImageBuffer.
    for (idx, rgba_img) in rgba_images.iter().enumerate() {
        let single_image_start = Instant::now();

        for y in 0..rgba_img.height() {
            for x in 0..rgba_img.width() {
                let pixel: &Rgba<u8> = rgba_img.get_pixel(x, y);
                merged_buffer.put_pixel(x, y + current_offset_y, *pixel);
            }
        }
        current_offset_y += rgba_img.height();

        println!(
            "Image {} processing time: {:?}",
            idx + 1,
            single_image_start.elapsed()
        );
    }

    println!("Total merging time: {:?}", start_time.elapsed());

    // 6. Convert the merged buffer to a DynamicImage in RGB8 format.
    let merged_rgb = DynamicImage::ImageRgba8(merged_buffer).to_rgb8();

    // 7. Compress the merged image. This example uses JPEG with optional dynamic quality adjustment.
    let compressed_data = compress_to_jpeg_with_limit(&merged_rgb, max_size_kb)?;

    Some(compressed_data)
}

/// Compresses the image to JPEG. If max_size_kb is specified, tries to adjust quality until
/// the size fits, or until the quality is too low.
///
/// * `img`         - The image to compress (in RgbImage format).
/// * `max_size_kb` - The maximum size in KB. If `None`, there's no size limit.
///
/// Returns Some(Vec<u8>) if compression succeeds, or None otherwise.
fn compress_to_jpeg_with_limit(
    img: &image::RgbImage,
    max_size_kb: Option<u64>,
) -> Option<Vec<u8>> {
    // Start with high quality compression (90%)
    let mut quality = 90f32;
    let (width, height) = img.dimensions();
    // Convert image data to raw bytes
    let raw_data: Vec<u8> = img.as_raw().to_vec();

    loop {
        eprintln!("Debug => Current quality: {}", quality);

        // Use catch_unwind to safely handle any panics during compression
        let result = catch_unwind(|| {
            // Initialize MozJPEG compressor with RGB color space
            let mut comp = Compress::new(ColorSpace::JCS_RGB);
            // Set image dimensions
            comp.set_size(width as usize, height as usize);
            // Set compression quality (0-100)
            comp.set_quality(quality);

            // Start compression with a new output buffer
            let mut comp = comp.start_compress(Vec::new()).unwrap();
            // Write image data to compressor
            comp.write_scanlines(&raw_data).unwrap();

            // Finalize compression and return the compressed data
            comp.finish().unwrap()
        });

        match result {
            Ok(buf) => {
                eprintln!("Debug => Compressed buffer size: {} bytes", buf.len());

                if let Some(kb_limit) = max_size_kb {
                    // Return if size is within limit or reached minimum quality
                    if (buf.len() as u64) <= kb_limit * 1024 || quality <= 10.0 {
                        return Some(buf);
                    }
                    // Decrease quality by 10% and try again, but not below 10%
                    quality = (quality - 10.0).max(10.0);
                } else {
                    // If no size limit specified, return the compressed data
                    return Some(buf);
                }
            }
            // Return None if compression fails
            Err(_) => return None,
        }
    }
}
