[CmdletBinding()]
param(
    [string]$SourceExecutable = (Join-Path $PSScriptRoot '..\target\release\calculator-desktop.exe'),
    [switch]$EnableStartup
)

$ErrorActionPreference = 'Stop'
$source = (Resolve-Path -LiteralPath $SourceExecutable).Path
$installDirectory = Join-Path $env:LOCALAPPDATA 'Programs\LumaCalc'
$installedExecutable = Join-Path $installDirectory 'LumaCalc.exe'
$startMenuDirectory = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs'
$shortcutPath = Join-Path $startMenuDirectory 'LumaCalc.lnk'

New-Item -ItemType Directory -Path $installDirectory -Force | Out-Null
Copy-Item -LiteralPath $source -Destination $installedExecutable -Force

$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = $installedExecutable
$shortcut.WorkingDirectory = $installDirectory
$shortcut.Description = 'LumaCalc — офлайн-калькулятор и конвертер'
$shortcut.Save()

if ($EnableStartup) {
    $command = '"{0}" --background' -f $installedExecutable
    New-ItemProperty `
        -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run' `
        -Name 'LumaCalc' `
        -Value $command `
        -PropertyType String `
        -Force | Out-Null
}

Write-Host "LumaCalc установлен: $installedExecutable"
Write-Host "Ярлык создан: $shortcutPath"
if (-not $EnableStartup) {
    Write-Host 'Автозапуск не включён. Его можно включить в параметрах LumaCalc.'
}

