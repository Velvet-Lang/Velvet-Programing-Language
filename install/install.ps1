Write-Host "[INFO] Downloading binary file..."
$TempPath = "$env:TEMP\weave"
curl -L -o $TempPath https://github.com/Velvet-Lang/Velvet-Programing-Language/releases/download/v0.1/weave
$InstallPath = "$env:ProgramFiles\Weave"
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath | Out-Null
}
Move-Item $TempPath "$InstallPath\weave.exe" -Force
Write-Host "[DONE] Now run weave help"
for ($i = 10; $i -ge 1; $i--) {
    Write-Host "$i seconds to clear" -NoNewline
    Start-Sleep -Seconds 1
    Write-Host "`r" -NoNewline
}
Clear-Host
