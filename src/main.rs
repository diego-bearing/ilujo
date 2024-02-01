use ilujo::{Command, CommandProcessor, Config};

fn main() {
    let config = Config::build().unwrap();

    let command = Command::build(&config.args).unwrap();

    let processor = CommandProcessor::new(config);

    processor.process(command).unwrap();
}
