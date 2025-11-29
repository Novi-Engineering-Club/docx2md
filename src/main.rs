use pandoc::OutputFormat;
use pandoc::OutputKind;
use pandoc::PandocOption;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let mut output_dir_name = String::new();
    io::stdin()
        .read_line(&mut output_dir_name)
        .expect("Failed to read line!");

    let output_dir_name = output_dir_name.trim();

    let docx_path = format!("{}.docx", output_dir_name);

    if !PathBuf::from(&docx_path).exists() {
        eprintln!("Error: {} does not exist", docx_path);
        return;
    }

    fs::create_dir_all(&output_dir_name).unwrap();
    env::set_current_dir(&output_dir_name).unwrap();

    let mut pandoc = pandoc::new();

    pandoc.add_input(&format!("../{}.docx", output_dir_name));
    pandoc.add_option(PandocOption::ExtractMedia(PathBuf::from(".")));

    pandoc.set_output_format(OutputFormat::MarkdownGithub, vec![]);

    pandoc.set_output(OutputKind::File(PathBuf::from(format!(
        "{}.md",
        output_dir_name
    ))));
    pandoc.execute().unwrap();
}
