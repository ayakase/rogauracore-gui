use std::process::Command;

pub fn set_color(r: u8, g: u8, b: u8) {
    let hex = format!("{:02x}{:02x}{:02x}", r, g, b);

    match Command::new("rogauracore")
        .arg("single_static")
        .arg(&hex)
        .status()
    {
        Ok(status) if status.success() => {
            println!("Applied {}", hex);
        }
        Ok(status) => {
            eprintln!("rogauracore exited with {}", status);
        }
        Err(e) => {
            eprintln!("Failed to execute rogauracore: {}", e);
        }
    }
}