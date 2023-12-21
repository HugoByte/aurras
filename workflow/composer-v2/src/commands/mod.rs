mod build;
use build::Build;
mod create;
use clap::StructOpt;
use composer_primitives::{Execute, Result};
mod validate;
use self::{create::Create, validate::Validate};

#[derive(StructOpt, Debug)]
pub enum Commands {
    #[structopt(about = "Build the current package as a workflow")]
    Build {
        #[structopt(flatten)]
        command: Build,
    },

    #[structopt(about = "Create a new package for echo")]
    Create {
        #[structopt(flatten)]
        command: Create,
    },

    #[structopt(about = "Validate the configuration file")]
    Validate {
        #[structopt(flatten)]
        command: Validate,
    },
}
