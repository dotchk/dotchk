# dotchk Installation Script for Windows
# Run with: powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"

# Colors for output
function Write-Error-Custom {
    param($Message)
    Write-Host "ERROR: $Message" -ForegroundColor Red
}

function Write-Success {
    param($Message)
    Write-Host "[OK] $Message" -ForegroundColor Green
}

function Write-Info {
    param($Message)
    Write-Host "INFO: $Message" -ForegroundColor Yellow
}

Write-Info "dotchk Installation Script for Windows"

# Check if Rust is installed
try {
    $rustVersion = rustc --version 2>$null
    if ($rustVersion) {
        Write-Success "Rust is already installed: $rustVersion"
    } else {
        throw "Rust not found"
    }
} catch {
    Write-Info "Rust is not installed. Installing Rust..."
    
    # Download and run rustup-init.exe
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    Write-Info "Downloading Rust installer..."
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    
    Write-Info "Running Rust installer..."
    Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait
    
    # Refresh environment variables
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    
    Write-Success "Rust installed successfully"
}

# Build and install dotchk
Write-Info "Building and installing dotchk..."
try {
    cargo install --path . --force
    Write-Success "dotchk installed successfully!"
} catch {
    Write-Error-Custom "Installation failed: $_"
    exit 1
}

# Verify installation
try {
    $version = dotchk --version
    Write-Success "Installation verified: $version"
} catch {
    Write-Error-Custom "dotchk was installed but cannot be run"
    Write-Info "You may need to add %USERPROFILE%\.cargo\bin to your PATH"
    Write-Info "Then restart your terminal"
}

# Show usage
Write-Host ""
Write-Success "Installation complete"
Write-Host ""
Write-Host "Quick start:" -ForegroundColor Cyan
Write-Host "  dotchk example.com"
Write-Host "  dotchk pattern `"[a-z]{3}.com`" --limit 100"
Write-Host "  dotchk tld mybrand --popular"
Write-Host ""
Write-Host "For more information, run: dotchk --help"