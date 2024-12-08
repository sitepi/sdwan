extern crate winres;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    if cfg!(target_os = "windows") || std::env::var("TARGET").unwrap_or_default().contains("windows") {
        let mut res = winres::WindowsResource::new();
        
        // 检查是否在 Linux 下交叉编译
        if !cfg!(target_os = "windows") {
            // 设置工具链路径
            res.set_ar_path("/usr/bin/x86_64-w64-mingw32-ar");
            res.set_windres_path("/usr/bin/x86_64-w64-mingw32-windres");
            println!("cargo:warning=Set windres path to x86_64-w64-mingw32-windres");
        }
        
        // 设置版本信息和详细元数据
        res.set("FileVersion", "0.0.8.0")
           .set("ProductVersion", "0.0.8.0")
           .set("FileDescription", "SitePi SDWAN Client - Secure Network Connection Tool")
           .set("ProductName", "SitePi SDWAN")
           .set("OriginalFilename", "sitepi.exe")
           .set("LegalCopyright", "Copyright © 2024 SitePi")
           .set("CompanyName", "SitePi")
           .set("InternalName", "sitepi")
           .set("Comments", "SitePi SDWAN Client for secure and reliable network connections")
           .set("Language", "0804")  // 简体中文
           .set("CharacterSet", "04E4");  // GB2312

        // 编译资源
        match res.compile() {
            Ok(_) => println!("cargo:warning=Resource compilation successful"),
            Err(e) => {
                eprintln!("cargo:warning=Resource compilation failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}