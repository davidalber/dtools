extern crate clap;
use clap::{App, SubCommand};
use std::io::Read;

#[derive(Debug)]
struct Data {
    data: Vec<f64>,
    min: f64,
    max: f64,
    mean: f64,
}

impl Data {
    fn new(data: Vec<f64>) -> Self {
        let mut max = std::f64::MIN;
        let mut min = std::f64::MAX;
        let len = data.len();
        let mut agg: f64 = 0.;

        for val in data.iter() {
            agg += *val;

            if *val > max {
                max = *val;
            }
            if *val < min {
                min = *val;
            }
        }

        Data {
            data,
            min,
            max,
            mean: agg / len as f64,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
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
            data.min,
            data.max,
        );
        println!("# Mean = {:.6}", data.mean);
    }

    Ok(())
}
