mod build;
use build::Build;
use clap::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Commands {
    #[structopt(about = "Build the current package as a workflow")]
    Build {
        #[structopt(flatten)]
        command: Build,
    },
}