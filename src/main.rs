mod cli;
mod scx_loader;

use clap::Parser;
use cli::{Cli, Commands};
use dbus::blocking::Connection;
use scx_loader::{ScxLoaderMode, ScxLoader};

fn cmd_get(scx_loader: ScxLoader) -> Result<(), Box<dyn std::error::Error>> {
    let current_scheduler: String = scx_loader.get_sched()?;
    let sched_mode = scx_loader.get_mode()?.as_str();
    match current_scheduler.as_str() {
        "unknown" => println!("no scx scheduler running"),
        _ => println!("running {} in {} mode", current_scheduler, sched_mode),
    }
    Ok(())
}

fn cmd_list(scx_loader: ScxLoader) -> Result<(), Box<dyn std::error::Error>> {
    let supported_scheds = scx_loader.get_supported_schedulers()?;
    println!("supported schedulers: {:?}", supported_scheds);
    Ok(())
}

fn cmd_start(
    scx_loader: ScxLoader,
    sched: String,
    mode: Option<ScxLoaderMode>,
    args: Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        Some(args) => {
            let (sched, args) = scx_loader.start_with_args(sched, args)?;
            println!("started {} with arguments \"{}\"", sched, args.join(" "));
        }
        None => {
            let (sched, mode) = scx_loader.start(sched, mode)?;
            println!("started {} in {} mode", sched, mode.as_str());
        }
    }
    Ok(())
}

fn cmd_switch(
    scx_loader: ScxLoader,
    sched: Option<String>,
    mode: Option<ScxLoaderMode>,
    args: Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match args {
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
    }
    Ok(())
}

fn cmd_stop(scx_loader: ScxLoader) -> Result<(), Box<dyn std::error::Error>> {
    scx_loader.stop()?;
    println!("stopped");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::new_system()?;
    let scx_loader = ScxLoader::new(&conn)?;

    match cli.command {
        Commands::Get => cmd_get(scx_loader)?,
        Commands::List => cmd_list(scx_loader)?,
        Commands::Start { args } => cmd_start(scx_loader, args.sched, args.mode, args.args)?,
        Commands::Switch { args } => cmd_switch(scx_loader, args.sched, args.mode, args.args)?,
        Commands::Stop => cmd_stop(scx_loader)?,
    }

    Ok(())
}
