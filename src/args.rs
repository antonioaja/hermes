use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Antonio Aguilar",
    version,
    about = "A CLI ftp client which copies a file with a given extension into specified folder."
)]
pub struct Args {
    /// The ip address of the server
    #[clap(short, long, value_parser)]
    pub ip: String,

    /// The port number to connect to
    #[clap(long, value_parser, default_value_t = 21)]
    pub port: u16,

    /// The extension to copy from the folder
    #[clap(short, long, value_parser, default_value = ".bmp")]
    pub extension: String,

    /// Delete the file from the server after copying
    #[clap(short, long, value_parser, default_value_t = false)]
    pub delete: bool,

    /// The folder to search in
    #[clap(short, long, value_parser, default_value = "")]
    pub folder: String,

    /// The output folder to copy to
    #[clap(short, long, value_parser, default_value = "")]
    pub output: String,

    /// Username of the server
    #[clap(short, long, value_parser, default_value = "")]
    pub user: String,

    /// Password of the server
    #[clap(long, value_parser, default_value = "")]
    pub password: String,
}
