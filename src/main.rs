use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Write};
use std::iter::FromIterator;
use std::path::PathBuf;

use structopt::StructOpt;

use mcu::Mcu;

mod diagram;
mod mcu;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "stm32-pin-diagram",
    about = "A program which generates LaTeX pinout diagrams for STM32 microcontrollers"
)]
struct Options {
    /// Model of the MCU (for example "STM32F429ZI").
    model: String,

    /// Output path - if not specified, the TeX code is written to stdout.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// List of peripherals to show in the diagram. If none is specified, all peripherals are
    /// shown.
    #[structopt(short, long)]
    peripheral: Vec<String>,
}

fn main() {
    let options = Options::from_args();

    println!("Loading MCU info...");
    let mcu = match Mcu::load(&options.model) {
        Some(mcu) => mcu,
        None => {
            println!("Could not find the specified MCU. The following MCUs are supported:");
            for mcu in Mcu::list().into_iter() {
                println!("  {}", mcu);
            }
            return;
        }
    };

    println!("Package: {}", mcu.package);
    /*println!("Pins:");
    for pin in mcu.pins.iter() {
        println!(
            "  {}: {} ({}) ({:?}) ({:?})",
            pin.position, pin.name, pin.type_, pin.gpio_modes, pin.functions
        );
    }*/

    // TODO: We could just give draw() an io::Write.

    let diagram = diagram::draw(&mcu, &BTreeSet::from_iter(options.peripheral.into_iter()));

    match options.output {
        Some(output_file) => {
            fs::write(&output_file, &diagram).expect("could not write output file");
        }
        None => {
            io::stdout().write_all(&diagram).unwrap();
        }
    }
}
