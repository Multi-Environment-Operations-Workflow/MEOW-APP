use qrcode::QrCode;
use image::Luma;

#[tauri::command]
pub fn generate_qr_code() -> String {
    let code = QrCode::new(b"01234567").unwrap();

    // Render to image
    let image = code.render::<Luma<u8>>().build();

    // Save as PNG in app's resource directory
    let path = "../src/assets/qr.png"; // adjust path as needed
    image.save(path).unwrap();

    path.to_string()
}
