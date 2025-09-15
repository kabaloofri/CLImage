use std::env;
use image::GenericImageView;
use minifb::{Window, WindowOptions};
use term_size;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image-path>", args[0]);
        return;
    }

    // Load image
    let img_path = &args[1];
    let img = image::open(img_path).expect("Failed to open image");

    // Terminal size (approximate)
    let (term_w, term_h) = term_size::dimensions().unwrap_or((80, 24));

    // Calculate max window size
    let max_w = term_w * 10;
    let max_h = term_h * 20;

    // Preserve aspect ratio
    let (orig_w, orig_h) = img.dimensions();
    let scale = f32::min(max_w as f32 / orig_w as f32, max_h as f32 / orig_h as f32);
    let width = (orig_w as f32 * scale) as u32;
    let height = (orig_h as f32 * scale) as u32;

    // Resize image to exact window size
    let img = img.resize_exact(width, height, image::imageops::FilterType::Nearest);

    // Convert RGBA image to u32 buffer
    let rgba = img.to_rgba8();
    let buffer: Vec<u32> = rgba
        .chunks(4)
        .map(|px| 0xFF_000000 | ((px[0] as u32) << 16) | ((px[1] as u32) << 8) | (px[2] as u32))
        .collect();

    // Ensure buffer length matches window size
    assert_eq!(buffer.len(), (width * height) as usize);

    
    {
        // Create window
        let mut window = Window::new(
            "CLImage viewer",
            width as usize,
            height as usize,
            WindowOptions::default(),
            
        ).expect("Failed to create window");

        // Display image until window closed
        while window.is_open() {
            
            if !window.get_keys_pressed(minifb::KeyRepeat::No).is_empty() {
                break;
            }
            
            window.update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window buffer");
        }
    }   
}
