use std::{
    fs::File,
    io::{BufReader, Error, Read, stdin},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    #[arg(short = 'l', long = "lines")]
    lines: bool,

    #[arg(short = 'w', long = "words")]
    words: bool,

    file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let bytes_flag: bool = cli.bytes;
    let lines_flag: bool = cli.lines;
    let words_flag: bool = cli.words;

    let file_path = cli.file;
    // If no file given, read from stdin
    // we can box the Read because both stdin and file
    // give Read types (though not the same)
    let mut reader: Box<dyn Read> = match &file_path {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(stdin()),
    };

    let mut line_count: usize = 0;
    let mut word_count: usize = 0;
    let mut bytes_count: usize = 0;
    let mut in_word = false;
    let mut buf = [0_u8; 8192];

    loop {
        // read bytes into buffer, and get number of bytes read
        let read = reader.read(&mut buf)?;
        // done reading bytes?
        if read == 0 {
            break;
        }

        // Count overall bytes read
        bytes_count += read;

        // we want to borrow from the start of bu  up to but not including index (read)
        // because read may or may not fill whole buffer at a time
        for &byte in &buf[..read] {
            // newline bytes are what we use to determine total lines (like wc)
            if byte == b'\n' {
                line_count += 1;
            }

            // Essentially everytime we get to a whitespace we know we aren't in a word, so we mark
            // it
            // next time around we know we are in a word, so increase word count, and toggle back
            // idk if this is the most efficient, but we aren't going for pure efficiency here
            if byte.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word {
                word_count += 1;
                in_word = true;
            }
        }
    }

    let print_default = !bytes_flag && !lines_flag && !words_flag;

    if print_default || lines_flag {
        print!("{}\t", line_count);
    }
    if print_default || words_flag {
        print!("{}\t", word_count);
    }
    if print_default || bytes_flag {
        print!("{}\t", bytes_count);
    }

    match &file_path {
        Some(path) => println!("{}", path.display()),
        None => println!(),
    }

    Ok(())
}
