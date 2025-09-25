use base64::Engine;
use image::{DynamicImage, Luma};
use qrcode::QrCode;
use std::io::Cursor;

#[tauri::command]
pub fn generate_qr_code() -> String {
    let code = QrCode::new(b"01234567").unwrap();

    // Render to Luma<u8> image buffer
    let image_buffer = code.render::<Luma<u8>>().build();

    // Convert ImageBuffer -> DynamicImage so we can call write_to
    let dyn_img = DynamicImage::ImageLuma8(image_buffer);

    // Encode to PNG in memory
    let mut buffer = Cursor::new(Vec::new());
    dyn_img
        .write_to(&mut buffer, image::ImageFormat::Png)
        .unwrap();

    // Convert to base64
    let base64_png = base64::engine::general_purpose::STANDARD.encode(buffer.into_inner());

    base64_png
}
