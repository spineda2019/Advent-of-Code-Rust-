use std::path::PathBuf;
use argparse::{ArgumentParser, Store};

pub fn get_file() -> PathBuf {
    let mut file_name: PathBuf = PathBuf::new();

    {
        let mut parser: ArgumentParser<'_> = ArgumentParser::new();
        parser.set_description("Send SCPI Commands Over UDP Multicast");
        parser
            .refer(&mut file_name)
            .add_option(&["-f", "--file"], Store, "Text File Name");
        parser.parse_args_or_exit();
    }

    let file_name: PathBuf = file_name;

    file_name
}

