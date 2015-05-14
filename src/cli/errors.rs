pub enum CliError {
    NoCommand
}

pub type CliResult = Result<(), CliError>;
