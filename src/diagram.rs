use std::collections::BTreeSet;
use std::io::{BufWriter, Write};

use super::mcu::{Mcu, Pin};

pub fn draw(mcu: &Mcu, shown_peripherals: &BTreeSet<String>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut writer = BufWriter::new(&mut output);
    let package = match Package::from_str(&mcu.package) {
        Some(package) => package,
        None => panic!("Package {} not yet supported.", mcu.package),
    };

    let mut peripherals = BTreeSet::new();
    for pin in mcu.pins.iter() {
        for function in pin.functions.iter() {
            let peripheral = Pin::split_function(function).0;
            if shown_peripherals.len() != 0 && !shown_peripherals.contains(peripheral) {
                continue;
            }
            if !peripherals.contains(peripheral) {
                peripherals.insert(peripheral.to_owned());
            }
        }
    }
    println!("{} peripherals:", peripherals.len());
    for peripheral in peripherals.iter() {
        println!("{}", peripheral);
    }

    begin_document(&mut writer, &peripherals);
    draw_package_body(&mut writer, package);
    draw_legend(&mut writer, package, &peripherals);
    for pin in mcu.pins.iter() {
        draw_pin_label(&mut writer, package, pin, &peripherals);
    }
    end_document(&mut writer);

    drop(writer);
    output
}

fn begin_document(writer: &mut BufWriter<&mut Vec<u8>>, peripherals: &BTreeSet<String>) {
    write!(
        writer,
        "{}",
        r#"\documentclass[crop,tikz]{standalone}
\usepackage{pgf}
\fontsize{10}{12}\selectfont
\usetikzlibrary{calc, positioning}
\pgfmathsetseed{12345}
"#
    )
    .unwrap();
    // Randomly generate colors for all peripherals.
    for peripheral in peripherals {
        write!(
            writer,
            "\\pgfmathsetmacro{{\\R}}{{random(0,10000)/10000}}
\\pgfmathsetmacro{{\\G}}{{random(0,10000)/10000}}
\\pgfmathsetmacro{{\\B}}{{random(0,10000)/10000}}
\\definecolor{{color{name}}}{{rgb}}{{\\R,\\G,\\B}}
",
            name = peripheral
        )
        .unwrap()
    }
    write!(
        writer,
        "{}",
        r#"\begin{document}
\begin{tikzpicture}[x=12pt,y=12pt]
"#
    )
    .unwrap();
}

fn end_document(writer: &mut BufWriter<&mut Vec<u8>>) {
    write!(
        writer,
        "{}",
        r#"\end{tikzpicture}
\end{document}
"#
    )
    .unwrap();
}

fn draw_package_body(writer: &mut BufWriter<&mut Vec<u8>>, package: Package) {
    if let Package::Qfp(pins) = package {
        let pins_per_side = package.pins_per_side();
        let edge = pins_per_side + 3;
        write!(writer, "\\draw[thick] (0, 0) --\n").unwrap();
        for i in 0..pins_per_side {
            write!(
                writer,
                "  coordinate[pos={pos}] (P{pin})\n",
                pos = (i + 2) as f64 / edge as f64,
                pin = i + 1,
            )
            .unwrap();
        }
        write!(writer, " ({size}, 0);\n", size = edge).unwrap();
        write!(writer, "\\draw[thick] ({size}, 0) --\n", size = edge).unwrap();
        for i in 0..pins_per_side {
            write!(
                writer,
                "  coordinate[pos={pos}] (P{pin})\n",
                pos = (i + 2) as f64 / edge as f64,
                pin = i + pins_per_side + 1,
            )
            .unwrap();
        }
        write!(writer, " ({size}, {size});\n", size = edge).unwrap();
        write!(writer, "\\draw[thick] ({size}, {size}) --\n", size = edge).unwrap();
        for i in 0..pins_per_side {
            write!(
                writer,
                "  coordinate[pos={pos}] (P{pin})\n",
                pos = (i + 2) as f64 / edge as f64,
                pin = i + pins_per_side * 2 + 1,
            )
            .unwrap();
        }
        write!(writer, " (0, {size});\n", size = edge).unwrap();
        write!(writer, "\\draw[thick] (0, {size}) --\n", size = edge).unwrap();
        for i in 0..pins_per_side {
            write!(
                writer,
                "  coordinate[pos={pos}] (P{pin})\n",
                pos = (i + 2) as f64 / edge as f64,
                pin = i + pins_per_side * 3 + 1,
            )
            .unwrap();
        }
        write!(writer, " (0, 0);\n").unwrap();

        // Draw pin numbers.
        for i in 0..pins {
            let side = Side(i / (pins / 4));
            write!(
                writer,
                "\\node[{side} = 0.1 of P{pin}, rotate={rotate}] {{{pin}}};\n",
                side = side.inside_position(),
                rotate = side.rotate(),
                pin = i + 1,
            )
            .unwrap();
        }
    } else {
        panic!("Not yet implemented.");
    }
}

fn draw_legend(
    writer: &mut BufWriter<&mut Vec<u8>>,
    package: Package,
    peripherals: &BTreeSet<String>,
) {
    let rows = (package.pins_per_side() as usize - 4) * 2 / 3;
    let pos_x = 3;
    let pos_y = package.pins_per_side() as f64 - 2.0;
    for (i, peripheral) in peripherals.iter().enumerate() {
        let column = i / rows;
        let row = i % rows;
        write!(
            writer,
            "\\node[draw, fill=color{rawname}!30, anchor=west] at ({x}, {y}) {{{name}}};\n",
            x = pos_x + column * 5,
            y = pos_y - row as f64 * 1.5,
            rawname = peripheral,
            name = latex_escape(peripheral),
        )
        .unwrap();
    }
}

fn draw_pin_label(
    writer: &mut BufWriter<&mut Vec<u8>>,
    package: Package,
    pin: &Pin,
    peripherals: &BTreeSet<String>,
) {
    if let Package::Qfp(pins) = package {
        let position = pin.position.parse::<u32>().expect("invalid pin position");
        let side = Side((position - 1) / (pins / 4));
        write!(
            writer,
            "\\node[{side} = 0.1 of P{pin}, rotate={rotate}] (p{pin}label0) {{{name}}};\n",
            side = side.outside_position(),
            rotate = side.rotate(),
            //anchor = side.outside_anchor(),
            pin = position,
            name = latex_escape(&pin.name),
        )
        .unwrap();

        // Draw the functions as well.
        // TODO: Ideally, the functions would be grouped together.
        let mut label_count = 0;
        for function in pin.functions.iter() {
            let (peripheral, signal) = Pin::split_function(function);
            if !peripherals.contains(peripheral) {
                continue;
            }
            write!(
                writer,
                "\\node[draw, fill=color{peripheral}!30, {side} = 0.1 of p{pin}label{label_count}, rotate={rotate}] (p{pin}label{label_count_p_1}) {{{name}}};\n",
                side = side.outside_position(),
                label_count = label_count,
                label_count_p_1 = label_count + 1,
                rotate = side.rotate(),
                //anchor = side.outside_anchor(),
                pin = position,
                peripheral = peripheral,
                name = latex_escape(signal),
            )
            .unwrap();
            label_count += 1;
        }
    } else {
        panic!("Not yet implemented.");
    }
}

#[derive(Copy, Clone)]
struct Side(u32);

impl Side {
    fn inside_anchor(self) -> &'static str {
        match self.0 {
            0 => "west",
            1 => "east",
            2 => "east",
            3 => "west",
            _ => panic!("invalid side"),
        }
    }
    fn outside_anchor(self) -> &'static str {
        Side((self.0 + 2) % 4).inside_anchor()
    }
    fn rotate(self) -> u32 {
        match self.0 {
            0 => 90,
            1 => 0,
            2 => 90,
            3 => 0,
            _ => panic!("invalid side"),
        }
    }
    fn inside_position(self) -> &'static str {
        match self.0 {
            0 => "right",
            1 => "left",
            2 => "left",
            3 => "right",
            _ => panic!("invalid side"),
        }
    }
    fn outside_position(self) -> &'static str {
        Side((self.0 + 2) % 4).inside_position()
    }
}

fn latex_escape(s: &str) -> String {
    s.replace("_", "\\_")
}

#[derive(Clone, Copy)]
enum Package {
    Qfp(u32),
    Bga(u32),
}

impl Package {
    fn from_str(s: &str) -> Option<Package> {
        if s.starts_with("LQFP") {
            let pins = s[4..].parse().ok()?;
            Some(Package::Qfp(pins))
        } else {
            None
        }
    }

    fn pins_per_side(&self) -> u32 {
        match self {
            Package::Qfp(pins) => pins / 4,
            _ => panic!("Not yet implemented."),
        }
    }
    fn pins(&self) -> u32 {
        match self {
            Package::Qfp(pins) => *pins,
            _ => panic!("Not yet implemented."),
        }
    }
}
