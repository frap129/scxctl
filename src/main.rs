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
    #[command(about = "Get the current scheduler and mode")]
    Get,
    #[command(about = "List all supported schedulers")]
    List,
    #[command(about = "Start a scheduler in a mode or with arguments")]
    Start {
        #[arg(short, long, help = "Scheduler to start")]
        sched: String,
        #[arg(
            short,
            long,
            value_enum,
            default_value = "auto",
            conflicts_with = "args",
            help = "Mode to start in"
        )]
        mode: Option<Mode>,
        #[arg(
            short,
            long,
            value_delimiter(','),
            conflicts_with = "mode",
            help = "Arguments to run scheduler with"
        )]
        args: Option<Vec<String>>,
    },
    #[command(about = "Switch schedulers or modes, optionally with arguments")]
    Switch {
        #[arg(short, long, help = "Scheduler to switch to")]
        sched: Option<String>,
        #[arg(
            short,
            long,
            value_enum,
            conflicts_with = "args",
            help = "Mode to switch to"
        )]
        mode: Option<Mode>,
        #[arg(
            short,
            long,
            value_delimiter(','),
            conflicts_with = "mode",
            help = "Arguments to run scheduler with"
        )]
        args: Option<Vec<String>>,
    },
    #[command(about = "Stop the current scheduler")]
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
        Commands::Start { sched, mode, args } => match args {
            Some(args) => {
                let (sched, args) = scx_loader.start_with_args(sched, args)?;
                println!("started {} with arguments \"{}\"", sched, args.join(" "));
            }
            None => {
                let (sched, mode) = scx_loader.start(sched, mode)?;
                println!("started {} in {} mode", sched, mode.as_str());
            }
        },
        Commands::Switch { sched, mode, args } => match args {
            Some(args) => {
                let (sched, args) = scx_loader.switch_with_args(sched, args)?;
                println!(
                    "switched to {} with arguments \"{}\"",
                    sched,
                    args.join(" ")
                );
            }
            None => {
                let (sched, mode) = scx_loader.switch(sched, mode)?;
                println!("switched to {} in {} mode", sched, mode.as_str());
            }
        },
        Commands::Stop => {
            scx_loader.stop()?;
            println!("stopped");
        }
    }

    Ok(())
}
