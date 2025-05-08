use std::fs::File;
use std::io::Write;
use std::process::Command;
use tauri;

/// Crée un script PowerShell pour désactiver l’historique du presse-papiers et le lance avec élévation
#[tauri::command]
pub fn disable_windows_clipboard_history() -> Result<(), String> {
    let script = r#"
Set-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Clipboard' -Name 'IsCloudAndHistoryFeatureAvailable' -Value 0 -Type DWord -Force
Stop-Process -Name explorer -Force
Start-Process explorer
"#;
    run_script_with_elevation(script)
}

/// Idem pour activer l’historique
#[tauri::command]
pub fn enable_windows_clipboard_history() -> Result<(), String> {
    let script = r#"
Set-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Clipboard' -Name 'IsCloudAndHistoryFeatureAvailable' -Value 1 -Type DWord -Force
Stop-Process -Name explorer -Force
Start-Process explorer
"#;
    run_script_with_elevation(script)
}

/// Génère un fichier .ps1 temporaire, le lance avec élévation, puis le supprime
fn run_script_with_elevation(script_content: &str) -> Result<(), String> {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("tacticlip_elevate.ps1");

    let mut file = File::create(&script_path)
        .map_err(|e| format!("Erreur création script PowerShell : {}", e))?;
    file.write_all(script_content.as_bytes())
        .map_err(|e| format!("Erreur écriture script PowerShell : {}", e))?;

    Command::new("powershell")
        .args([
            "-Command",
            &format!("Start-Process powershell -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File \"{}\"' -Verb RunAs", script_path.display())
        ])
        .spawn()
        .map_err(|e| format!("Erreur lancement script avec élévation : {}", e))?;

    Ok(())
}
