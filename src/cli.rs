// use crate::brightness::{detect_brightness, detect_monitors, set_brightness};
use clap::{ArgAction, Args, Parser, Subcommand};

/// A command line tool to control monitor settings
/// Examples:
///     monitorctl b 100
///     monitorctl b -i 10
///     monitorctl b -d 10
#[derive(Debug, Parser)]
#[clap(author, version, verbatim_doc_comment)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum SubCommands {
    /// Change Brightness
    #[clap(visible_alias = "b")]
    Brightness(SubCmdArgs),
    /// Change Contrast
    #[clap(visible_alias = "c")]
    Contrast(SubCmdArgs),
    /// Change Volume
    #[clap(visible_alias = "v")]
    Volume(SubCmdArgs),
    /// Debug Information
    Debug,
}

#[derive(Debug, Args, Clone)]
pub struct SubCmdArgs {
    /// Get current value
    #[clap(short, long, action=ArgAction::SetTrue, conflicts_with_all=["decrease", "increase"])]
    pub get_current_value: bool,

    /// Value needed if '-i'/'--increase' or '-d'/'--decrease' flag is not used
    #[clap(value_parser, value_name = "Value", default_value="5", required_unless_present_any=["get_current_value", "increase", "decrease"])]
    pub value: u8,

    /// Adds the value to the current value
    #[clap(short = 'i', long = "increase", action=ArgAction::SetTrue, conflicts_with_all=["decrease", "get_current_value"])]
    pub increase: bool,

    /// Subs the value from the current value
    #[clap(short = 'd', long = "decrease", action=ArgAction::SetTrue, conflicts_with_all=["increase", "get_current_value"])]
    pub decrease: bool,
}
