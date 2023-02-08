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
        .map(|x| list::File::try_from(x.as_str()).ok().unwrap())
        .collect();
    for i in files {
        let copyable = match get_extension(i.name()) {
            Ok(ext) => {
                if ext == "bmp" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        if copyable {
            let buffer = stream
                .retr_as_buffer(i.name())
                .context(format!("{} could not be retrieved", i.name()))?;

            let mut file = File::create(format!("{}{}", &args.output, i.name()))?;

            file.write_all(&buffer.into_inner())?;
        }
    }

    Ok(())
}

fn get_extension(filename: &str) -> Result<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .context(format!("Could not find extension for {}", filename))
}
