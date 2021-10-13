use color_eyre::eyre::{eyre, Result, WrapErr};
use directories::UserDirs;
use qwasar_setup::add_exercise;
use qwasar_setup::write;
use std::path::PathBuf;
use structopt::StructOpt;

/// A CLI for the growing and curation of a digital garden
///
#[derive(StructOpt, Debug)]
#[structopt(name = "qwasar")]
struct Opt {
    #[structopt(parse(from_os_str), short = "p", long)]
    path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        #[structopt(short, long)]
        exercise_number: Option<String>,
        #[structopt(short, long)]
        file_name: Option<String>,
    },
}

fn get_default_path() -> Result<PathBuf> {
    // let user_dirs = UserDirs::new().ok_or_else(|| eyre!("Could not find home directory"))?;
    // Ok(user_dirs.home_dir().join(".garden"))
    let default_path = std::env::current_dir().wrap_err("Unable to get current_dir")?;
    Ok(default_path)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();

    let ex_path = match opt.path {
        Some(pathbuf) => Ok(pathbuf),
        None => get_default_path().wrap_err("`path` was not supplied"),
    }?;

    match opt.cmd {
        Command::Add {
            exercise_number,
            file_name,
        } => add_exercise(ex_path, exercise_number, file_name),
    }
}
