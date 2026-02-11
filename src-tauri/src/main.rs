#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::process::Command;

#[tauri::command]
fn scan_driver_updates() -> Result<String, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
if (-not (Get-Module -ListAvailable -Name PSWindowsUpdate)) {
  Install-Module PSWindowsUpdate -Force -Scope CurrentUser
}
Import-Module PSWindowsUpdate -Force
Get-WindowsUpdate -MicrosoftUpdate -Verbose | Out-String
"#;

    run_powershell(script)
}

#[tauri::command]
fn install_driver_updates() -> Result<String, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
if (-not (Get-Module -ListAvailable -Name PSWindowsUpdate)) {
  Install-Module PSWindowsUpdate -Force -Scope CurrentUser
}
Import-Module PSWindowsUpdate -Force
Install-WindowsUpdate -MicrosoftUpdate -AcceptAll -AutoReboot -Verbose | Out-String
"#;

    run_powershell(script)
}

#[tauri::command]
fn backup_drivers(destination: String) -> Result<String, String> {
    if destination.trim().is_empty() {
        return Err("Debes indicar una ruta de respaldo válida.".to_string());
    }

    let destination_path = Path::new(&destination);
    let escaped_path = destination_path
        .to_str()
        .ok_or_else(|| "La ruta de respaldo no es válida.".to_string())?
        .replace('"', "`\"");

    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$dest = "{}"
if (-not (Test-Path $dest)) {{
  New-Item -Path $dest -ItemType Directory | Out-Null
}}
Export-WindowsDriver -Online -Destination $dest | Out-String
"#,
        escaped_path
    );

    run_powershell(&script)
}

#[tauri::command]
fn reinstall_drivers(source: String) -> Result<String, String> {
    if source.trim().is_empty() {
        return Err("Debes indicar la carpeta del respaldo para reinstalar.".to_string());
    }

    let source_path = Path::new(&source);
    if !source_path.exists() {
        return Err("La carpeta indicada no existe.".to_string());
    }

    let escaped = source_path
        .to_str()
        .ok_or_else(|| "La ruta del respaldo no es válida.".to_string())?
        .replace('"', "`\"");

    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$src = "{}"
pnputil /add-driver "$src\*.inf" /subdirs /install | Out-String
"#,
        escaped
    );

    run_powershell(&script)
}

#[tauri::command]
fn validate_backup_folder(source: String) -> Result<String, String> {
    if source.trim().is_empty() {
        return Err("Debes indicar una ruta para validar.".to_string());
    }

    let path = Path::new(&source);
    if !path.exists() {
        return Err("La carpeta no existe.".to_string());
    }

    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$src = "{}"
$files = Get-ChildItem -Path $src -Filter *.inf -Recurse
"Total INF encontrados: $($files.Count)"
"#,
        source.replace('"', "`\"")
    );

    run_powershell(&script)
}

fn run_powershell(script: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
            .output()
            .map_err(|e| format!("No se pudo ejecutar PowerShell: {e}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = script;
        let _ = Command::new("true");
        Err("Esta aplicación solo puede ejecutar gestión de drivers en Windows.".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_driver_updates,
            install_driver_updates,
            backup_drivers,
            reinstall_drivers,
            validate_backup_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
