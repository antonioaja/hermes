use anyhow::*;
use clap::Parser;
use std::convert::TryFrom;
use suppaftp::{list, FtpStream};

fn main() -> Result<()> {
    // Parse CLI arguments
    let args: Args = Args::parse();

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

#[derive(Parser, Debug)]
#[clap(
    author = "Antonio Aguilar",
    version,
    about = "A CLI ftp client which copies a file with a given extension into specified folder."
)]
struct Args {
    /// The ip address of the server
    #[clap(short, long, value_parser)]
    ip: String,

    /// The port number to connect to
    #[clap(short, long, value_parser, default_value_t = 21)]
    port: u16,

    /// The extension to copy from the folder
    #[clap(short, long, value_parser, default_value = ".bmp")]
    extension: String,

    /// Delete the file from the server after copying
    #[clap(short, long, value_parser, default_value_t = false)]
    delete: bool,

    /// The folder to search in
    #[clap(short, long, value_parser, default_value = "")]
    folder: String,

    /// The output folder to copy to
    #[clap(short, long, value_parser, default_value = "")]
    output: String,

    /// Username of the server
    #[clap(short, long, value_parser, default_value = "")]
    user: String,

    /// Password of the server
    #[clap(short, long, value_parser, default_value = "")]
    password: String,
}
