use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        
        // 设置 windres 工具路径
        // res.set_toolkit_path("x86_64-w64-mingw32-");  // 修正方法名
        
        res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0008);  // v0.0.8
        res.set_version_info(winres::VersionInfo::FILEVERSION, 0x0008);
        
        res.set("FileDescription", "SitePi SDWAN Client")
            .set("ProductName", "SitePi SDWAN")
            .set("CompanyName", "SitePi")
            .set("LegalCopyright", "Copyright (C) SitePi")
            .set("OriginalFilename", "sitepi.exe")
            .set("InternalName", "sitepi.exe")
            .set("FileVersion", "0.0.8")
            .set("ProductVersion", "0.0.8");
        
        match res.compile() {
            Ok(_) => println!("Resource compilation successful"),
            Err(e) => eprintln!("Resource compilation failed: {}", e),
        }
    }
}