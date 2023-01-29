

use std::path::PathBuf;

use clap::Parser;

use photoman::Manager;





#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// working directory, default = current directory
    #[arg(default_value=".")]
    work_dir: PathBuf,
    /// output command without execution
    #[arg(long, action = clap::ArgAction::SetTrue, default_value="false")]
    dry_run: bool    
}

fn main() {
    let args = Args::parse();


    let mut manager = Manager::new()
        .work_dir(args.work_dir);
    if args.dry_run {
        manager = manager.dry_run();
    }   
    manager.arrange_files();

}
