use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

/// Kill a process and all its children (needed for PyInstaller --onefile).
fn kill_process_tree(pid: u32) {
    // First, find and kill children
    if let Ok(output) = std::process::Command::new("pgrep")
        .args(["-P", &pid.to_string()])
        .output()
    {
        let children = String::from_utf8_lossy(&output.stdout);
        for child_pid in children.lines() {
            if let Ok(cpid) = child_pid.trim().parse::<u32>() {
                kill_process_tree(cpid);
            }
        }
    }
    // Then kill this process
    let _ = std::process::Command::new("kill")
        .args([&pid.to_string()])
        .status();
}

type SidecarState = Arc<Mutex<Option<u32>>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sidecar_pid: SidecarState = Arc::new(Mutex::new(None));
    let pid_for_window = Arc::clone(&sidecar_pid);
    let pid_for_exit = Arc::clone(&sidecar_pid);

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let sidecar = app.shell().sidecar("fastapi-server")?;
            let (_rx, child) = sidecar.spawn()?;
            let pid = child.pid();

            // Store the PID for cleanup
            *pid_for_window.lock().unwrap() = Some(pid);

            // Also handle window close (red button)
            let pid_for_close = Arc::clone(&pid_for_window);
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    if let Some(pid) = pid_for_close.lock().unwrap().take() {
                        kill_process_tree(pid);
                    }
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // Handle Cmd+Q / app quit
    app.run(move |_app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            if let Some(pid) = pid_for_exit.lock().unwrap().take() {
                kill_process_tree(pid);
            }
        }
    });
}
