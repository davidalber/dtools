extern crate clap;
extern crate rand;

use clap::{App, SubCommand};
use rand::Rng;
use std::io::Read;

struct Welford {
    count: usize,
    mean: f64,
    m2: f64,
}

impl Welford {
    fn new() -> Self {
        Welford {
            count: 0,
            mean: 0.,
            m2: 0.,
        }
    }

    fn update(&mut self, value: f64) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;
    }

    fn sample_variance(&self) -> f64 {
        self.m2 / self.count as f64
    }
}

fn find_nth(start_ind: usize, stop_ind: usize, target: usize, mut vec: &mut Vec<f64>) -> f64 {
    // stop_ind is exclusive
    let mut rng = rand::thread_rng();
    let pivot = rng.gen_range(start_ind, stop_ind);
    let pivot_val = *vec.get(pivot).unwrap();

    // Swap pivot into tail position.
    vec.swap(pivot, stop_ind - 1);

    let mut i = start_ind;
    for j in i..stop_ind - 1 {
        if *vec.get(j).unwrap() < pivot_val {
            vec.swap(i, j);
            i += 1;
        }
    }

    // Swap the pivot into the correct position.
    vec.swap(stop_ind - 1, i);

    // At this point, the pivot value is its sorted position.
    if i == target {
        // The current pivot is the target value.
        pivot_val
    } else if i < target {
        // Recursively search the right partition.
        find_nth(i + 1, stop_ind, target, &mut vec)
    } else {
        // Recursively search the left partition.
        find_nth(start_ind, i, target, &mut vec)
    }
}

fn find_median(mut vec: &mut Vec<f64>) -> f64 {
    // Find median through incomplete quicksort.
    let len = *&vec.len();
    match len % 2 {
        0 => {
            // even
            let low = find_nth(0, len, len / 2 - 1, &mut vec);
            let high = find_nth(0, len, len / 2, &mut vec);
            (low + high) / 2.
        }
        _ => {
            // odd
            find_nth(0, len, len / 2, &mut vec)
        }
    }
}

#[derive(Debug)]
struct Data {
    data: Vec<f64>,
    min: f64,
    max: f64,
    mean: f64,
    variance: f64,
    median: f64,
}

impl Data {
    fn new(mut data: Vec<f64>) -> Self {
        let mut max = std::f64::MIN;
        let mut min = std::f64::MAX;
        let mut welford = Welford::new();

        for val in data.iter() {
            if *val > max {
                max = *val;
            }
            if *val < min {
                min = *val;
            }

            welford.update(*val);
        }

        let median = find_median(&mut data);

        Data {
            data,
            min,
            max,
            mean: welford.mean,
            variance: welford.sample_variance(),
            median,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn stddev(&self) -> f64 {
        self.variance.sqrt()
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

    Ok(())
}
