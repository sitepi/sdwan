# SitePi SDWAN Windows 客户端构建脚本

param(
    [switch]$Release,
    [switch]$Clean,
    [switch]$Run,
    [string]$Interface = "wg0",
    [string]$Config = ""
)

$ErrorActionPreference = "Stop"

Write-Host "SitePi SDWAN Windows Build Script" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# 检查是否在正确的目录
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Error: Cargo.toml not found. Please run this script from the windows directory." -ForegroundColor Red
    exit 1
}

# 清理
if ($Clean) {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Clean failed!" -ForegroundColor Red
        exit 1
    }
    Write-Host "Clean completed successfully." -ForegroundColor Green
    Write-Host ""
}

# 构建
$buildType = if ($Release) { "release" } else { "debug" }
$buildFlag = if ($Release) { "--release" } else { "" }

Write-Host "Building in $buildType mode..." -ForegroundColor Yellow
cargo build $buildFlag

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Build completed successfully!" -ForegroundColor Green

# 获取可执行文件路径
$exePath = "target\$buildType\sitepi.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "Error: Executable not found at $exePath" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Executable: $exePath" -ForegroundColor Cyan
$fileInfo = Get-Item $exePath
Write-Host "Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Cyan
Write-Host ""

# 运行
if ($Run) {
    # 检查管理员权限
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    
    if (-not $isAdmin) {
        Write-Host "Warning: This application requires administrator privileges!" -ForegroundColor Yellow
        Write-Host "Please run this script as Administrator to execute the application." -ForegroundColor Yellow
        exit 0
    }
    
    Write-Host "Running application..." -ForegroundColor Yellow
    Write-Host ""
    
    # 构建参数
    $args = @("--interface", $Interface)
    
    if ($Config -ne "") {
        $args += @("--config", $Config)
    }
    
    # 显示运行命令
    Write-Host "Command: $exePath $($args -join ' ')" -ForegroundColor Cyan
    Write-Host ""
    
    # 运行程序
    & $exePath $args
}
else {
    Write-Host "To run the application (requires Administrator):" -ForegroundColor Yellow
    Write-Host "  .\$exePath --interface $Interface" -ForegroundColor White
    Write-Host ""
    Write-Host "Or use this script with -Run flag:" -ForegroundColor Yellow
    Write-Host "  .\build.ps1 -Release -Run -Interface wg0" -ForegroundColor White
}
