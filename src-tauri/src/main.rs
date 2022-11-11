#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn calculate(number1: &str, number2: &str, operation: &str) -> String {
    if number1.is_empty() || number2.is_empty() {
        format!("Nenhum dos campos pode estar vazio!")
    } else {
        let number_1: f64 = number1.trim().parse().unwrap();

        let number_2: f64 = number2.trim().parse().unwrap();

        if operation == "+" {
            format!("{} + {} = {}", number_1, number_2, (number_1 + number_2))
        } else if operation == "-" {
            format!("{} - {} = {}", number_1, number_2, (number_1 - number_2))
        } else if operation == "*" {
            format!("{} * {} = {}", number_1, number_2, (number_1 * number_2))
        } else if operation == "/" {
            if number_2 == 0_f64 {
                format!("Você não pode dividir por ZERO!")
            } else {
                format!("{} / {} = {:.2}", number_1, number_2, (number_1 / number_2))
            }
        } else {
            format!("A operação \"{}\" é inválida!", operation)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
