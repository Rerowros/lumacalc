[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$installDirectory = Join-Path $env:LOCALAPPDATA 'Programs\LumaCalc'
$installedExecutable = Join-Path $installDirectory 'LumaCalc.exe'
$shortcutPath = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\LumaCalc.lnk'
$runKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'

if (Test-Path -LiteralPath $runKey) {
    $entry = Get-ItemProperty -LiteralPath $runKey -Name 'LumaCalc' -ErrorAction SilentlyContinue
    $expected = '"{0}" --background' -f $installedExecutable
    if ($null -ne $entry -and $entry.LumaCalc -eq $expected) {
        Remove-ItemProperty -LiteralPath $runKey -Name 'LumaCalc'
    }
}

if (Test-Path -LiteralPath $shortcutPath) {
    Remove-Item -LiteralPath $shortcutPath -Force
}
if (Test-Path -LiteralPath $installDirectory) {
    $resolved = (Resolve-Path -LiteralPath $installDirectory).Path
    $expectedDirectory = [System.IO.Path]::GetFullPath($installDirectory)
    if ($resolved -ne $expectedDirectory) {
        throw "Отказ удаления неожиданного пути: $resolved"
    }
    Remove-Item -LiteralPath $resolved -Recurse -Force
}

Write-Host 'LumaCalc удалён. Локальная история и настройки пользователя не изменялись.'

