extern crate clap;
use clap::{App, SubCommand};
use std::io::Read;

#[derive(Debug)]
struct Data {
    data: Vec<f64>,
    min_val: f64,
    max_val: f64,
}

impl Data {
    fn new(data: Vec<f64>) -> Self {
        let mut max_val = std::f64::MIN;
        let mut min_val = std::f64::MAX;

        for val in data.iter() {
            if *val > max_val {
                max_val = *val;
            }
            if *val < min_val {
                min_val = *val;
            }
        }

        Data {
            data,
            min_val,
            max_val,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn min(&self) -> f64 {
        self.min_val
    }

    fn max(&self) -> f64 {
        self.max_val
    }
}

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

fn main() -> Result<(), i32> {
    let matches = App::new("dhacks")
        .version("0.1")
        .about("Visualize data in the terminal")
        .subcommand(
            SubCommand::with_name("histogram")
                .aliases(&["histo"])
                .about("(alias: \"histo\") Generate histogram from data"),
        )
        .get_matches();

    let data: Data = match load_data() {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e);
            return Err(1);
        }
    };

    if matches.subcommand_matches("histogram").is_some() {
        println!("{:?}", data);
        println!(
            "# NumSamples = {}; Min = {:.2}; Max = {:.2}",
            data.len(),
            data.min(),
            data.max()
        );
    }

    Ok(())
}
