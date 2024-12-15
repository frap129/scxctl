use crate::modes::Mode;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
