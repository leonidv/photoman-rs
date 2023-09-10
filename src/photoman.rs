use std::path::PathBuf;

use clap::Parser;

use photoman::Manager;
use tracing::info;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, filter::LevelFilter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// working directory, default = current directory
    #[arg(default_value = ".")]
    work_dir: PathBuf,
    /// output command without execution
    #[arg(long, action = clap::ArgAction::SetTrue, default_value="false")]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

   let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("PHOTOMAN_LOG")
        .from_env_lossy();

    tracing_subscriber::fmt()
        .log_internal_errors(true)
        .compact()
        .with_env_filter(env_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_ansi(true)
        .init();


    let mut manager = Manager::new().work_dir(args.work_dir);
    if args.dry_run {
        manager = manager.dry_run();
    }
    manager.arrange_files();

}
