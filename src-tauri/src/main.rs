// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(serde::Serialize)]
struct Output {
    stdout: String,
    stderr: String,
    status: i32,
}

#[tauri::command]
async fn turn_off_monitor(delay_secs: i64, app_handle: tauri::AppHandle) -> Output {
    // println!("{}", std::env::current_exe().unwrap().display());
    let nircmd_resource_path = app_handle
        .path_resolver()
        .resolve_resource("resources\\nircmd-x86_64-pc-windows-msvc.exe")
        .expect("Failed to get app dir");
    println!("app.path_resolver");
    println!("{}", nircmd_resource_path.display());

    // Convert the PathBuf to a Vec of &str
    let components: Vec<&str> = nircmd_resource_path
        .iter()
        .map(|os_str| os_str.to_str().unwrap())
        .collect();

    // Add double quotes around each component
    let escaped_components: Vec<String> = components
        .iter()
        .enumerate()
        .map(|(count, component)| {
            // ignore \\? and \C: in the path
            if count > 1 {
                format!("\"{}\"", component)
            } else {
                component.to_string()
            }
        })
        .collect();

    // Join the components back together with backslashes
    let escaped_path = escaped_components.join("\\");

    println!("escaped path");
    println!("{}", escaped_path);

    let command_to_execute = format!(
        "Start-Job {{
                        cmd /c start timeout /t {delay_secs} \n {escaped_path} monitor off
                      }} | Receive-Job -Wait -AutoRemoveJob"
    );

    let output = tauri::api::process::Command::new("powershell")
        .args(&[command_to_execute.as_str()])
        .output()
        .unwrap();

    return Output {
        // stdout: nircmd_resource_path_str.to_string(),
        stdout: output.stdout,
        stderr: output.stderr,
        status: output.status.code().unwrap_or_default(),
    };
}

// #[tauri::command]
// async fn run_command(command: String, args: Vec<String>) -> Output {
//     // validate the command first
//     // then run it
//     let output = tauri::api::process::Command::new("cmd")
//         .args(args)
//         .output()
//         // TODO: handle error
//         .unwrap();
//     return Output {
//         stdout: output.stdout,
//         stderr: output.stderr,
//         status: output.status.code().unwrap_or_default(),
//     };
// }

// #[tauri::command]
// fn callTurnOffWindowCommand(delaySecs: i64) -> String {
//     let output = Command::new("cmd")
//         .args(&[
//             "/C",
//             "timeout /t 1\n
//         C:\\nircmd monitor off",
//         ])
//         .output()
//         .expect("failed to execute process");
//     let s = String::from_utf8_lossy(&output.stderr);

//     return s.to_lowercase();
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![turn_off_monitor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
