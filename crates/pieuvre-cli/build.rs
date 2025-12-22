fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set("ProductName", "Pieuvre");
        res.set(
            "FileDescription",
            "Outil SOTA d'alignement système Windows 11",
        );
        res.set("LegalCopyright", "Copyright (c) 2025 Pieuvre Contributors");
        res.compile().unwrap();
    }
}
