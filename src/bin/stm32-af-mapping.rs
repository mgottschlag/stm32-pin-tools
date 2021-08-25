use std::collections::{BTreeMap, BTreeSet};

use structopt::StructOpt;

use stm32_pin_tools::mcu::Mcu;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "stm32-af-mapping",
    about = "A program which lists alternate function mappints for groups of STM32 MCUs"
)]
struct Options {
    /// Peripheral or AF pattern (for example "ART" matches all UART or USART AF pins).
    af_pattern: String,

    /// MCU pattern (for example "STM32F4" matches all F4 MCUs).
    mcu_pattern: String,
}

fn main() {
    let options = Options::from_args();

    println!("Loading MCU info...");
    let mut models = Mcu::all_models()
        .into_iter()
        .filter(|x| x.contains(&options.mcu_pattern))
        .collect::<Vec<_>>();
    models.sort();
    // GPIOs are identical for all families (except that some are not present in smaller packages),
    // so we group by family. The first 9 characters specify the family.
    let mut families = BTreeSet::new();
    // (AF name) -> ((GPIO, index) -> Vec<Family)
    let mut af_info = BTreeMap::new();
    for model in models.iter() {
        let family = model[0..9].to_string();
        families.insert(family.clone());
        let mcu = Mcu::load(model).expect("could not load mcu");
        for pin in mcu.pins {
            for af in pin.functions.iter() {
                if af.contains(&options.af_pattern) {
                    let af_entry = af_info.entry(af.to_string()).or_insert(BTreeMap::new());
                    let mapping = af_entry.entry(pin.name.clone()).or_insert(BTreeSet::new());
                    mapping.insert(family.clone());
                }
            }
        }
    }

    println!("\nMCU families matching the specified pattern:");
    for model in families {
        println!("  {}", model);
    }

    println!("\nAlternate functions matching the specified pattern:");
    for af in af_info.keys() {
        println!("  {}", af);
    }

    println!("");
    for (af, mappings) in af_info {
        println!("{}:", af);
        for (gpio, families) in mappings {
            println!(
                "  {} ({})",
                gpio,
                families
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Mapping(String, usize);
