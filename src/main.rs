use anyhow::*;
use clap::Parser;
use std::convert::TryFrom;
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
        println!("{}", i.name());
    }

    Ok(())
}
