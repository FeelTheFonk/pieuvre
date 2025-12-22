use std::path::Path;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // SOTA 2026: Auto-generate ICO from Premium PNG
        let png_path = "icon_premium.png";
        let ico_path = "icon.ico";

        if Path::new(png_path).exists() {
            let img = image::open(png_path).expect("Failed to open premium icon PNG");
            let icon_dir = {
                let mut dir = ico::IconDir::new(ico::ResourceType::Icon);
                for size in [16, 32, 48, 64, 128, 256] {
                    let resized =
                        img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
                    let rgba = resized.to_rgba8();
                    let icon_image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
                    dir.add_entry(ico::IconDirEntry::encode(&icon_image).unwrap());
                }
                dir
            };

            let file = std::fs::File::create(ico_path).expect("Failed to create icon.ico");
            icon_dir.write(file).expect("Failed to write icon.ico");
        }

        let mut res = winres::WindowsResource::new();
        res.set_icon(ico_path);
        res.compile().unwrap();
    }
}
