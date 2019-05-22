mod dhacks;

extern crate clap;

use clap::{App, ArgMatches, SubCommand};
use std::io::Read;

use dhacks::data::Data;
use dhacks::histogram::Histogram;

fn load_data() -> std::io::Result<Data> {
    let mut data: Vec<f64> = Vec::new();
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    for line in buffer.split("\n") {
        let trimmed = line.trim();
        if trimmed == "" {
            // Skip empty lines.
            continue;
        }

        match trimmed.parse::<f64>() {
            Ok(val) => data.push(val),
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("could not parse '{}' as f64", trimmed),
                ))
            }
        }
    }

    Ok(Data::new(data))
}

fn parse_command_line() -> ArgMatches<'static> {
    App::new("dhacks")
        .version("0.1")
        .about("Visualize data in the terminal")
        .subcommand(
            SubCommand::with_name("histogram")
                .aliases(&["histo"])
                .about("(alias: \"histo\") Generate histogram from data"),
        )
        .get_matches()
}

fn print_aggregates(data: &Data) {
    println!(
        "# NumSamples = {}; Min = {:.2}; Max = {:.2}",
        data.len(),
        data.min,
        data.max,
    );
    println!(
        "# Mean = {:.6}; Variance = {:.6}; SD = {:.6}; Median = {:.6}",
        data.mean,
        data.variance,
        data.stddev(),
        data.median,
    );
}

fn main() -> Result<(), i32> {
    let matches = parse_command_line();

    let data: Data = match load_data() {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e);
            return Err(1);
        }
    };

    print_aggregates(&data);

    if matches.subcommand_matches("histogram").is_some() {
        let histo = Histogram::new(&data, 10);
        histo.render();
    }

    Ok(())
}
