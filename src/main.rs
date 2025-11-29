use pandoc::OutputKind;
use pandoc::OutputFormat;
use pandoc::PandocOption;
use std::path::PathBuf;
use std::fs;
use std::env;

fn main() {
    fs::create_dir_all("output").unwrap();
    env::set_current_dir("output").unwrap();
    let mut pandoc = pandoc::new();
    pandoc.add_input("../example.docx");
    pandoc.add_option(PandocOption::ExtractMedia(PathBuf::from(".")));
    pandoc.set_output_format(OutputFormat::MarkdownGithub, vec![]);
    pandoc.set_output(OutputKind::File(PathBuf::from("example.md")));
    pandoc.execute().unwrap();
}
