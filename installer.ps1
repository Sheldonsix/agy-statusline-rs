$ErrorActionPreference = "Stop"

$AppName = "agy-statusline-rs"
$Repo = "Sheldonsix/agy-statusline-rs"
$InstallDir = if ($env:AGY_STATUSLINE_INSTALL_DIR) {
    $env:AGY_STATUSLINE_INSTALL_DIR
} else {
    Join-Path $env:LOCALAPPDATA "Programs\agy-statusline-rs\bin"
}
$ConfigDir = Join-Path $env:APPDATA "agy-statusline"
$ConfigPath = Join-Path $ConfigDir "config.json"
$Artifact = "agy-statusline-rs-x86_64-pc-windows-msvc.zip"
$Url = "https://github.com/$Repo/releases/latest/download/$Artifact"

if ($env:PROCESSOR_ARCHITECTURE -notin @("AMD64", "x86_64")) {
    throw "Unsupported Windows architecture: $env:PROCESSOR_ARCHITECTURE"
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString())
$ArchivePath = Join-Path $TempDir $Artifact
$ExtractDir = Join-Path $TempDir "unpacked"

try {
    New-Item -ItemType Directory -Force $TempDir, $ExtractDir, $InstallDir, $ConfigDir | Out-Null

    Write-Host "Downloading $Url"
    Invoke-WebRequest -Uri $Url -OutFile $ArchivePath -UseBasicParsing

    Expand-Archive -Path $ArchivePath -DestinationPath $ExtractDir -Force
    Copy-Item (Join-Path $ExtractDir "$AppName.exe") (Join-Path $InstallDir "$AppName.exe") -Force

    if (-not (Test-Path $ConfigPath)) {
        @'
{
  "modules": {
    "dir": { "enabled": true, "order": 10 },
    "branch": { "enabled": true, "order": 20 },
    "model": { "enabled": true, "order": 30 },
    "tokens": { "enabled": true, "order": 40 },
    "quota": { "enabled": true, "order": 50 }
  },
  "display": {
    "color": "auto",
    "icons": "auto",
    "layout": "auto"
  }
}
'@ | Set-Content -Path $ConfigPath -Encoding UTF8
        Write-Host "Created config: $ConfigPath"
    } else {
        Write-Host "Config already exists: $ConfigPath"
    }

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathParts = @()
    if ($UserPath) {
        $PathParts = $UserPath -split ";"
    }
    if ($PathParts -notcontains $InstallDir) {
        $NewPath = if ($UserPath) { "$UserPath;$InstallDir" } else { $InstallDir }
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        Write-Host "Added to user PATH: $InstallDir"
        Write-Host "Restart your terminal to use $AppName.exe from PATH."
    }

    Write-Host "Installed: $(Join-Path $InstallDir "$AppName.exe")"
    Write-Host "Antigravity statusline command:"
    Write-Host "  $(Join-Path $InstallDir "$AppName.exe")"
} finally {
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir
    }
}
