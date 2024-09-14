use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use url::Url;
use clap::Parser;

/// Fetches contents of a GitHub folder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub URL of the folder
    url: String,

    /// Timeout for git clone operation
    #[arg(short, long, default_value_t = 5)]
    timeout: u32,

    /// Output directory for the fetched content
    #[arg(short, long, default_value = "output")]
    output_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (repo_url, folder) = parse_github_url(&args.url)?;

    println!("Cloning repository: {}", repo_url);
    println!("Target folder: {}", folder);

    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Clone the repository with the specified timeout
    let status = Command::new("timeout")
        .arg(args.timeout.to_string())
        .arg("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--single-branch")
        .arg("--branch")
        .arg("main")
        .arg(&repo_url)
        .arg(temp_path)
        .status()?;

    if !status.success() {
        return Err(format!("Error: Git clone timed out after {} seconds or failed.", args.timeout).into());
    }

    let target_dir = temp_path.join(&folder);
    if !target_dir.exists() {
        return Err(format!("Error: Folder {} not found in the repository.", folder).into());
    }

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&args.output_dir)?;

    let repo_name = repo_url.split('/').last().unwrap_or("repo").trim_end_matches(".git");
    let output_file_name = format!("{}_{}.txt", repo_name, folder.replace('/', "_"));
    let output_file_path = args.output_dir.join(output_file_name);
    let mut output_file = File::create(&output_file_path)?;

    println!("Processing files...");

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("Processing: {}", file_name);
            writeln!(output_file, "File: {}/{}", folder, file_name)?;
            let content = fs::read_to_string(&path)?;
            writeln!(output_file, "{}\n", content)?;
        }
    }

    println!("Content has been saved to {}", output_file_path.display());

    Ok(())
}

fn parse_github_url(url: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let parsed_url = Url::parse(url)?;
    let path_segments: Vec<&str> = parsed_url.path_segments().ok_or("Invalid URL")?.collect();

    if path_segments.len() < 5 {
        return Err("Invalid GitHub URL format".into());
    }

    let repo_url = format!("https://github.com/{}/{}.git", path_segments[0], path_segments[1]);
    let folder = path_segments[4..].join("/");

    Ok((repo_url, folder))
}