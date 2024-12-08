use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set_manifest_file("manifest.xml");
        res.set("FileDescription", "SitePi SDWAN Client");
        res.set("ProductName", "SitePi SDWAN");
        res.set("CompanyName", "SitePi");
        res.set("LegalCopyright", "Copyright (C) SitePi");
        res.set("OriginalFilename", "sitepi.exe");
        res.compile().unwrap();
    }
}
