mod modes;
mod scx_loader;

use clap::{Parser, Subcommand};
use dbus::blocking::Connection;
use modes::Mode;
use scx_loader::ScxLoader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get,
    List,
    Start {
        #[arg(short, long)]
        sched: String,
        #[arg(short, long, value_enum)]
        mode: Option<Mode>,
    },
    /*StartSchedWithArgs {
        #[arg(short, long)]
        sched: String,
        args: Vec<String>,
    },*/
    Switch {
        #[arg(short, long)]
        sched: Option<String>,
        #[arg(short, long, value_enum)]
        mode: Option<Mode>,
    },
    /*SwitchSchedWithArgs {
        #[arg(short, long)]
        sched: String,
        args: Vec<String>,
    },*/
    Stop,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::new_system()?;
    let scx_loader = ScxLoader::new(&conn)?;

    match cli.command {
        Commands::Get => {
            let current_scheduler: String = scx_loader.get_sched()?;
            let sched_mode = scx_loader.get_mode()?.as_str();
            match current_scheduler.as_str() {
                "unknown" => println!("no scx scheduler running"),
                _ => println!("running {} in {} mode", current_scheduler, sched_mode),
            }
        }
        Commands::List => {
            let supported_scheds = scx_loader.get_supported_schedulers()?;
            println!("supported schedulers: {:?}", supported_scheds);
        }
        Commands::Start { sched, mode } => {
            let (sched, mode) = scx_loader.start(sched, mode)?;
            println!("started {} in {} mode", sched, mode.as_str());
        }
        Commands::Switch { sched, mode } => {
            let (sched, mode) = scx_loader.switch(sched, mode)?;
            println!("switched to {} in {} mode", sched, mode.as_str());
        }
        Commands::Stop => {
            scx_loader.stop()?;
            println!("stopped");
        }
    }

    Ok(())
}