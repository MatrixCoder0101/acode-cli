use crate::{command::Command, error::CliError, open::open};
use clap::Args;

const DOC: &str = "https://acode.foxdebug.com/plugin-docs";

#[derive(Debug, Args)]
pub struct Docs {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the docs url ( will not open )
    show: bool,
}

impl Command for Docs {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        if self.show {
            show();
        } else {
            open(DOC)?;
        }
        Ok(())
    }
}

fn show() {
    println!("Acode Plugun Docs: {}", DOC);
}
