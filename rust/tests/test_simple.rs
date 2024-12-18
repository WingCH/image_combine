use image::{codecs::png::PngEncoder, ExtendedColorType, ImageBuffer, ImageEncoder, Rgba};
use image_combine::api::simple::merge_images_vertically;
use std::fs::{File, create_dir_all};
use std::io::Write;
use image::codecs::jpeg::JpegEncoder;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

/// Test case 1: Input an empty array, should return None
#[test]
fn test_empty_input() {
    let result = merge_images_vertically(&[], None);
    assert_eq!(result, None, "Expected None when input is empty");
}

/// Test case 2: Input invalid image data (unreadable or corrupted), should return None
#[test]
fn test_invalid_images() {
    let invalid_data = vec![0, 1, 2, 3, 4, 5]; // Random or insufficient data
    let result = merge_images_vertically(&[invalid_data], None);
    assert_eq!(result, None, "Expected None when input is invalid");
}

/// Test case 3: Merge multiple valid images, should return Some(Vec<u8>)
#[test]
fn test_multiple_images() {

    // Create two images with different dimensions
    let buffer1 = ImageBuffer::from_pixel(2, 2, Rgba([255, 0, 0, 255])); // Red 2x2
    let buffer2 = ImageBuffer::from_pixel(2, 3, Rgba([0, 255, 0, 255])); // Green 2x3

    // Convert each image to bytes (PNG encoding)
    let mut bytes1 = Vec::new();
    let mut bytes2 = Vec::new();
    {
        let encoder1 = PngEncoder::new(&mut bytes1);
        encoder1
            .write_image(
                &buffer1,
                buffer1.width(),
                buffer1.height(),
                ExtendedColorType::Rgba8,
            )
            .unwrap();

        let encoder2 = PngEncoder::new(&mut bytes2);
        encoder2
            .write_image(
                &buffer2,
                buffer2.width(),
                buffer2.height(),
                ExtendedColorType::Rgba8,
            )
            .unwrap();
    }

    // Call the function to be tested
    let result = merge_images_vertically(&[bytes1, bytes2], None);
    assert!(
        result.is_some(),
        "Expected Some(...) for multiple valid images"
    );

    // Further check the merged image dimensions
    if let Some(encoded_jpeg) = result {
        // Decode the merged JPEG data
        let merged_img =
            image::load_from_memory(&encoded_jpeg).expect("Failed to decode merged JPEG data");

        // Check if the width is 2 and the height is 5 (2 + 3)
        assert_eq!(merged_img.width(), 2, "Merged image width should be 2");
        assert_eq!(merged_img.height(), 5, "Merged image height should be 5");
    }
}


#[test]
fn test_merge_images_with_max_size() {
    // use cargo test --release, to run this test, debug mode is too slow
    // Print the current working directory for debugging
    println!("******Current working directory: {:?}", std::env::current_dir().unwrap());

    // Preload images as byte arrays
    let receipt_1_path = include_bytes!("sample/receipt_1.jpeg");
    let receipt_1_1_path = include_bytes!("sample/receipt_1.1.jpeg");
    let receipt_2_path = include_bytes!("sample/receipt_2.jpeg");

    // Decode the images from memory
    let buffer1 = image::load_from_memory(receipt_1_path)
        .expect("Failed to decode receipt_1.jpeg")
        .to_rgb8();
    let buffer2 = image::load_from_memory(receipt_1_1_path)
        .expect("Failed to decode receipt_1.1.jpeg")
        .to_rgb8();
    let buffer3 = image::load_from_memory(receipt_2_path)
        .expect("Failed to decode receipt_2.jpeg")
        .to_rgb8();

    // Define a function to encode an image buffer to JPEG format
    fn encode_to_jpeg(buffer: &image::RgbImage) -> Vec<u8> {
        let mut bytes = Vec::new();
        let encoder = JpegEncoder::new(&mut bytes);
        encoder
            .write_image(
                buffer,
                buffer.width(),
                buffer.height(),
                ExtendedColorType::Rgb8,
            )
            .expect("Failed to encode image");
        bytes
    }

    // Encode the images in parallel
    let encoded_images: Vec<Vec<u8>> = vec![buffer1, buffer2, buffer3]
        .into_par_iter()
        .map(|buffer| encode_to_jpeg(&buffer))
        .collect();

    // Print the size of each encoded image (Debug mode)
    if cfg!(debug_assertions) {
        for (i, encoded_image) in encoded_images.iter().enumerate() {
            println!(
                "Size of buffer{}: {} bytes ({} KB)",
                i + 1,
                encoded_image.len(),
                encoded_image.len() as f64 / 1024.0
            );
        }
    }

    // Set the maximum allowed size limit for the merged image (in KB)
    let max_size_kb = Some(500);

    // Test the merge_images_vertically function
    let result = merge_images_vertically(&encoded_images, max_size_kb);
    assert!(
        result.is_some(),
        "Expected Some(...) for multiple valid images with max size limit"
    );

    // Verify the size of the merged image
    if let Some(encoded_jpeg) = result {
        if cfg!(debug_assertions) {
            println!(
                "Size of merged image: {} bytes ({} KB)",
                encoded_jpeg.len(),
                encoded_jpeg.len() as f64 / 1024.0
            );
        }
        assert!(
            (encoded_jpeg.len() as u64) <= max_size_kb.unwrap() * 1024,
            "Merged image size should be within the max size limit"
        );

        // Save the merged image for debugging purposes
        save_debug_image("debug_images", "merged_debug.jpg", &encoded_jpeg).unwrap();
    }
}

/// Saves an image to a specified directory and filename.
fn save_debug_image(folder_path: &str, filename: &str, image_bytes: &[u8]) -> std::io::Result<()> {
    // Create the directory if it does not already exist
    create_dir_all(folder_path)?;

    // Construct the full file path
    let full_path = format!("{}/{}", folder_path, filename);

    // Create and write to the file
    let mut file = File::create(&full_path)?;
    file.write_all(image_bytes)?;
    println!("Debug image saved to: {}", full_path);
    Ok(())
}
