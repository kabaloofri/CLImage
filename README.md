## 🖼️ Mini Image Viewer

A tiny, cross-platform image viewer written in Rust.
Displays PNG/JPG images in a simple window that scales to your terminal size.

## Features ✨

Cross-platform: Linux (X11/Wayland), Windows, macOS

Scales images to fit your terminal while preserving aspect ratio

Closes on any key press or window close

No external dependencies besides minifb and image

## Installation 🛠️

#### Clone the repository:

git clone https://github.com/yourusername/mini_image_viewer.git
cd mini_image_viewer


#### Build with Cargo:

cargo build --release

Usage 🚀
cargo run --release -- <path-to-image>


## Example:

cargo run --release -- ./my_picture.png


Press any key to close the window.

## Dependencies 📦

minifb
 — simple framebuffer window library

image
 — image decoding and manipulation

term_size
 — read terminal size
