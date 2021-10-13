use assert_cmd::cargo;
use cargo_toml::Manifest;
use color_eyre::{eyre::WrapErr, owo_colors::OwoColorize, Result};
use edit::edit_file;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub fn add_exercise(
    root_path: PathBuf,
    exercise_number: Option<String>,
    file_name: Option<String>,
) -> Result<()> {
    let provided_exercise = match exercise_number {
        Some(raw_file) => format_ex_name(&raw_file),
        None => prompt_with_message("your ex directory (ex00, ex01, etc)")
            .wrap_err("Unable to get ex dir from user")?,
    };

    let provided_file = match file_name {
        Some(raw_file) => format_ex_name(&raw_file),
        None => prompt_with_message("your qwasar file name: (my_rust_journey_primitive_types)")
            .wrap_err("Unable to get ex dir from user")?,
    };

    let cargo_file = Manifest::from_path(root_path).wrap_err("Unable to fetch cargo.toml")?;

    dbg!(cargo_file.workspace);
    Ok(())
}

fn format_cargo_toml_path(cargo_path: PathBuf) -> PathBuf {
    match cargo_path.ends_with("cargo.toml") {
        true => cargo_path,
        false => format!("{}/cargo.toml", cargo_path),
    }
}

fn prompt_with_message(message: &str) -> Result<String> {
    let new_message = format!(
        "\
    Enter {}
    >",
        message
    );

    // colorizing message requires a format
    rprompt::prompt_reply_stderr(&format!("{}", new_message.blue().bold()))
        .wrap_err("Failed to get filename")
        .map(|response| {
            let formatted_response = format_ex_name(&response);
            slug::slugify(formatted_response)
        })
}

fn confirm_response(message: &str, raw_formatted_response: &str) -> Result<String> {
    loop {
        // prompt defaults to uppercase character in question
        // this is a convention, not a requirement enforced by the code
        let result = rprompt::prompt_reply_stderr(&format!(
            "\
{} {}
Do you want a different entry? (y/N): ",
            "current entry:".green().bold(),
            raw_formatted_response,
        ))
        .wrap_err("Failed to get input for y/n question")?;

        match result.as_str() {
            "y" | "Y" => break prompt_with_message(message),
            "n" | "N" | "" => break Ok(slug::slugify(raw_formatted_response)),
            _ => {
                // ask again
            }
        }
    }
}

fn format_ex_name(raw_name: &str) -> String {
    match raw_name.starts_with("ex") {
        true => raw_name.to_string(),
        false => format!("ex{}", raw_name),
    }
}
