
use rfd::FileDialog;
use std::fs::{self};
use std::{io};
use std::io::Write;
use std::process;

fn encrypt(contents: Vec<u8>) -> Vec<u8> {
    //let bin: Vec<u8> = string_to_decimals(&contents).unwrap();
    //println!("Enter Key >> ");
    //let mut user_input: String = String::new();
    //io::stdin().read_line(&mut user_input).expect("failed to get user input.");
    //let user_int: u8 = user_input.trim().parse().unwrap();
    contents.iter()
        .map(|byte| {
            byte.wrapping_add(69)
        })
        .collect()
    //let encryped_text: String = binary_to_string(&bin).unwrap();
}

fn decrypt(contents: Vec<u8>) -> Vec<u8> {
    contents.iter()
        .map(|byte| {
            byte.wrapping_sub(69)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\n\t\tXcrypt\n");
        println!("  1) encrypt file");
        println!("  2) decrypt file");
        println!("  3) quit");

        println!("\nenter choice below: ");
        let mut user = String::new();
        io::stdin().read_line(&mut user).expect("failed to get user input.");
        let input = user.as_str().trim();

        match input {
            "1" => {
                let f = FileDialog::new().pick_file().unwrap();
                let trimmed_path = f.display().to_string();
                let path = trimmed_path.as_str().trim();
                match fs::read(path) {
                    Ok(contents) => {
                        let encrypted_text = encrypt(contents);
                        //println!("Enter Key >> ");
                        //let mut user_input: String = String::new();
                        //io::stdin().read_line(&mut user_input).expect("failed to get user input.");
                        //let key: Vec<u8> = string_to_decimals(&user_input).unwrap();
                        //let final_text: Vec<u8> = encrypted_text.iter().zip(key.iter()).map(|(&byte1, &byte2)| byte1 ^ byte2).collect();
                        let mut new_file = fs::OpenOptions::new()
                            .write(true)
                            .open(path)
                            .unwrap();
                        if let Err(e) = new_file.write_all(&encrypted_text) {
                            println!("Error writting to file: {e}")
                        }
                    }
                    Err(e) => {
                        println!("could not open file: {e}");
                    }
                }
                //let mut file = File::open(f).expect("cant open file");
                //let mut contents = String::new();
                //file.read_to_string(&mut contents).expect("cant read file.");

                //let mut new_f = File::create(trimmed_path)?;
                //let encrpted_vec = encrypt(contents);
                //println!("{:?}", encrpted_vec);
                //let encrypted_text: String = decimals_to_string(&encrpted_vec).unwrap();
                //write!(new_f, "{encrypted_text}")?;

            }   

            "2" => {
                let f = FileDialog::new().pick_file().unwrap();
                let trimmed_path = f.display().to_string();
                let path = trimmed_path.as_str().trim();
                match fs::read(path) {
                    Ok(contents) => {
                        let encrypted_text = decrypt(contents);
                        let mut new_file = fs::OpenOptions::new()
                            .write(true)
                            .open(path)
                            .unwrap();
                        if let Err(e) = new_file.write_all(&encrypted_text) {
                            println!("Error writting to file: {e}")
                        }
                    }
                    Err(e) => {
                        println!("could not open file: {e}");
                    }
                }
            }

            "3" => {
                process::exit(1);
            }

            _ => {
                println!("invalid input.")
            }
        }
    }
}

