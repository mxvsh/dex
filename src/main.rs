use std::thread;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};
use clap::Parser;
use indicatif::{
  MultiProgress, ProgressBar, ProgressStyle,
};

#[derive(Parser, Debug)]
#[command(
  version,
  author = "mxvsh",
  about = "simple tool to run a command in multiple directories.",
  long_about = "dex is a simple tool to run a command in multiple directories. It is useful when you have to run a command in multiple directories and you don't want to do it manually."
)]
struct Args {
  #[arg(short, long)]
  command: String,

  #[arg(short, long)]
  dirs: String,
}
fn main() {
  let args = Args::parse();

  let cmd = args.command;
  let dirs: Vec<String> = args
    .dirs
    .split(",")
    .map(|s| s.to_string())
    .collect();

  let mut handles = vec![];
  let multi = MultiProgress::new();

  let spinner_style =
    ProgressStyle::default_spinner()
      .template("{spinner} {prefix} {msg}")
      .unwrap();

  for dir in dirs {
    let cmd: String = cmd.clone();

    let pb =
      multi.add(ProgressBar::new_spinner());
    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("\"{}\" - ", dir));
    pb.enable_steady_tick(Duration::from_millis(
      100,
    ));

    let handle = thread::spawn(move || {
      let mut parts = cmd.split_whitespace();
      let program = parts.next().unwrap();
      let args: Vec<&str> = parts.collect();

      pb.set_message("Running...");

      let now = SystemTime::now();

      let mut child = Command::new(program)
        .args(&args)
        .current_dir(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn");

      child
        .wait()
        .expect("Failed to wait for the child");

      let total_time = now.elapsed().unwrap();

      pb.set_prefix("✔");
      pb.finish_with_message(format!(
        "Completed in {}s",
        total_time.as_secs()
      ));
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
