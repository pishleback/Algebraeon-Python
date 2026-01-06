use clap::Parser as ClapParser;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use walkdir::WalkDir;

fn extract_python_blocks(markdown: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<String> = None;

    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.as_ref().starts_with("python") =>
            {
                current = Some(String::new());
            }
            Event::Text(text) => {
                if let Some(ref mut block) = current {
                    block.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some(block) = current.take() {
                    blocks.push(block);
                }
            }
            _ => {}
        }
    }

    blocks
}

fn run_and_stream(command: &str) -> std::io::Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let stdout_thread = std::thread::spawn(move || {
        for line in stdout.lines() {
            println!("{}", line.unwrap());
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        for line in stderr.lines() {
            eprintln!("{}", line.unwrap());
        }
    });

    let status = child.wait()?;

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    if !status.success() {
        panic!("Command failed with status: {}", status);
    }

    Ok(())
}

/// Example program with optional boolean flags
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Build in release mode
    #[arg(long)]
    release: bool,

    /// Skip rebuilding
    #[arg(long)]
    dont_rebuild: bool,
}

fn main() {
    let args = Args::parse();

    let venv_path = "../algebraeon/.env/bin/activate";

    if !args.dont_rebuild {
        run_and_stream(
            format!(
                "source {venv_path} && cd ../algebraeon && maturin develop{}",
                if args.release { " --release" } else { "" }
            )
            .as_str(),
        )
        .unwrap();
    }

    for entry in WalkDir::new("../guide/src")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.path().extension().and_then(|s| s.to_str()) != Some("md") {
                None
            } else {
                Some(entry.into_path())
            }
        })
        .chain([PathBuf::from_str("../README.md").unwrap()].into_iter())
    {
        println!("📄 Running blocks in `{}`", entry.to_str().unwrap());

        let markdown = fs::read_to_string(entry).unwrap();
        let blocks = extract_python_blocks(&markdown);

        for (i, block) in blocks.iter().enumerate() {
            println!("🧪 Block #{}", i + 1);

            let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
            write!(temp_file, "{}", block).expect("Failed to write Python code");

            run_and_stream(
                format!(
                    "source {venv_path} && python3 {}",
                    temp_file.path().to_str().unwrap()
                )
                .as_str(),
            )
            .unwrap();
        }
    }
}
