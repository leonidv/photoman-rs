use std::path::PathBuf;

use clap::Parser;

use photoman::Manager;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

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

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .with(EnvFilter::from_env("PHOTOMAN_LOG"))
    //     .init();

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_env("PHOTOMAN_LOG"))
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_ansi(true)
       // .with_max_level(Level::TRACE)
        .init();

    let mut manager = Manager::new().work_dir(args.work_dir);
    if args.dry_run {
        manager = manager.dry_run();
    }
    manager.arrange_files();

}
