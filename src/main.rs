use anyhow::*;
use clap::Parser;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::result::Result::Ok;
use std::{convert::TryFrom, fs::File};
use suppaftp::{list, FtpStream};

pub mod args;

fn main() -> Result<()> {
    // Parse CLI arguments
    let args: args::Args = args::Args::parse();

    // Connect to the server
    let mut stream = FtpStream::connect(format!("{}:{}", &args.ip, &args.port)).context(
        format!("Could not connect to {} on port {}", &args.ip, &args.port),
    )?;
    stream.login(&args.user, &args.password).context(format!(
        "Could not login with these credentials:\nUser: {}\nPassword: {}",
        &args.user, &args.password
    ))?;

    // Change into target directory
    stream
        .cwd(&args.folder)
        .context(format!("Could not change directory into {}", &args.folder))?;

    // Parse name of files
    let files: Vec<list::File> = stream
        .list(None)
        .ok()
        .context(format!(
            "Could not list files in {:?}",
            stream
                .pwd()
                .context("Could not return the current directory")?
        ))?
        .iter()
        .map(|x| {
            list::File::try_from(x.as_str())
                .ok()
                .context(format!(
                    "Could not list files in {:?}",
                    stream
                        .pwd()
                        .context("Could not return the current directory")
                        .unwrap()
                ))
                .unwrap()
        })
        .collect();

    // Iterate over all files
    for i in files {
        let potential = i.name();

        // Check if we wanna copy a file
        let copyable = get_extension(potential) == args.extension;

        if copyable {
            // Read file into buffer
            let buffer = stream
                .retr_as_buffer(i.name())
                .context(format!("{} could not be retrieved", potential))?;

            // Create and write to file
            let mut file = File::create(format!("{}{}", &args.output, potential))
                .context(format!("Could not create {}{}", &args.output, potential))?;
            file.write_all(&buffer.into_inner())
                .context(format!("Could write to {}{}", &args.output, potential))?;

            // Delete file from server if desired
            if args.delete {
                stream
                    .rm(potential)
                    .context(format!("Could not removed {} from the server", potential))?;
            }
        }
    }

    Ok(())
}

/// Returns extension (if available) an input string
/// Example: "test.png" => "png"
fn get_extension(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
}
