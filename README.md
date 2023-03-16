# hermes

## Description

A Rust program that connects to a server via FTP, lists the files in a specified directory, and copies files with a given file extension to a local directory. If specified, it can also delete the original files from the server.

The code is divided into two main parts: CLI argument parsing and FTP file copying.

## Dependencies

* `anyhow` crate for error handling
* `clap` crate for CLI argument parsing
* `suppaftp` crate for FTP file handling

## CLI Arguments

The following arguments are supported:

* ip - IP address of the server (required)
* port - port number of the server (optional, defaults to `21`)
* user - username for FTP authentication (optional, defaults to `""`)
* password - password for FTP authentication (optional, defaults to `""`)
* folder - folder on the server to list and copy files from (optional, defaults to `""`)
* extension - file extension to copy (optional, defaults to `"bmp"`)
* output - local folder to copy files to (optional, defaults to `""`)
* delete - flag to indicate whether to delete original files from server after copying (optional, defaults to `false`)

## Code Structure

The `main` function is the entry point of the program. It performs the following steps:

1. Parse CLI arguments using Args::parse() function from the args module.
2. Connect to the FTP server using FtpStream::connect() function from the suppaftp module.
3. Login to the server using FtpStream::login() function from the suppaftp module.
4. Change the working directory on the server using FtpStream::cwd() function from the suppaftp module.
5. List files in the target directory using FtpStream::list() function from the suppaftp module.
6. Iterate over all files in the directory.
7. Check if the file has the desired extension.
8. If it does, retrieve the file using FtpStream::retr_as_buffer() function from the suppaftp module.
9. Write the file to disk using File::create() and File::write_all() functions from the std module.
10. Delete the file from the server (if desired) using FtpStream::rm() function from the 