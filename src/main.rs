use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, GetAsyncKeyState, VK_MENU};
use std::{thread, time::Duration, io};

const VK_Q: i32 = 0x51; // Q key virtual key code

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interval_us = get_valid_input("Enter the desired interval between clicks in microseconds", 
                                           "請輸入點擊間隔的微秒數");

    let num_clicks = get_valid_input("Enter the number of clicks (or enter 0 for infinite clicks):", 
                                          "請輸入點擊次數（輸入 0 表示無限點擊）");

    let infinite_clicks = num_clicks == 0;

    println!("AutoClicker started. Press Alt+Q to stop.");
    println!("自動點擊器已啟動。按 Alt+Q 停止。");

    if !infinite_clicks {
        for _ in 0..num_clicks {
            if is_alt_q_pressed() {
                break;
            }
            // Simulate left click
            unsafe {
                mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            }
            thread::sleep(Duration::from_micros(interval_us));
        }
    } else {
        loop {
            if is_alt_q_pressed() {
                break;
            }
            // Simulate left click
            unsafe {
                mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            }
            thread::sleep(Duration::from_micros(interval_us));
        }
    }

    println!("AutoClicker stopped.");
    println!("自動點擊器已停止。");
    Ok(())
}

fn get_valid_input(prompt_en: &str, prompt_zh: &str) -> u64 {
    loop {
        println!("{}", prompt_en);
        println!("{}", prompt_zh);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<u64>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                println!("無效輸入。請輸入有效的數字。");
            }
        }
    }
}

fn is_alt_q_pressed() -> bool {
    unsafe {
        (GetAsyncKeyState(VK_MENU) as u16 & 0x8000) != 0 && (GetAsyncKeyState(VK_Q) as u16 & 0x8000) != 0
    }
}