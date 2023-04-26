

use std::fmt::Debug;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(name = "DAC App")]
#[clap(author = "Vijaya Manohar D. <vdogiparthi@tudelft.nl>")]
#[clap(version = "1.0")]
#[clap(about = "Set the analog voltages for the selected DAC channels", long_about = None)]
#[clap(allow_negative_numbers = false)]

struct Args {
    /// Operation on the DAC Channel
    #[clap(short, long, value_parser)]
    #[clap(arg_enum, value_parser)]
    operation:OperationMode,

    /// Index of the DAC Channel
    #[clap(short, long, value_parser)]
    channel_index: u8,

    /// Voltage to be set
    #[clap(short, long, value_parser, default_value_t = 2.5)]
    voltage: f32,

    /// Voltage span
    #[clap(short, long, value_parser, default_value_t = 5.0)]
    span_voltage: f32,

    /// Enable the verbose mode: provides the execution sequence
    #[clap(short, long, action, default_value_t=0)]
    mode_verbose: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OperationMode {
    On,
    Off,
}


fn main() {
    let args = Args::parse();

    println!("{}", args.channel_index);
    //println!("{}", args.operation);
    println!("{}", args.voltage);
    println!("{}", args.span_voltage);
}
