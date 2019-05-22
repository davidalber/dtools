mod dhacks;

extern crate clap;

use clap::{App, SubCommand};
use std::io::Read;

use dhacks::data::Data;

struct Buckets {
    buckets: Vec<usize>,
    min: f64,
    max: f64,
    bucket_separation: f64,
}

impl Buckets {
    fn new(num_buckets: usize, min: f64, max: f64) -> Self {
        let mut buckets: Vec<usize> = Vec::with_capacity(num_buckets);
        for _ in 0..num_buckets {
            buckets.push(0);
        }

        Buckets {
            buckets,
            min,
            max,
            bucket_separation: (max - min) / num_buckets as f64,
        }
    }

    fn insert(&mut self, val: f64) {
        let bucket_index = match val == self.max {
            true => self.buckets.len() - 1,
            false => self.bucket_index(val),
        };
        self.buckets[bucket_index] += 1;
    }

    fn bucket_index(&self, val: f64) -> usize {
        ((val - self.min) / self.bucket_separation).floor() as usize
    }

    fn bucket_range(&self, bucket_id: usize) -> (f64, f64) {
        let low = self.min + self.bucket_separation * bucket_id as f64;
        (low, low + self.bucket_separation)
    }

    fn get_samples_per_block(&self) -> usize {
        const BAR_WIDTH: usize = 75;

        // Find the maximum number of samples in a bucket.
        let max_bucket_elements = self.buckets.iter().fold(0, |acc, x| std::cmp::max(acc, *x));
        max_bucket_elements / BAR_WIDTH
    }
}

struct Histogram {
    buckets: Buckets,
    num_samples: usize,
}

impl Histogram {
    fn new(data: &Data, num_buckets: usize) -> Self {
        let mut buckets = Buckets::new(num_buckets, data.min, data.max);
        for val in data.data.iter() {
            buckets.insert(*val);
        }

        Histogram {
            buckets,
            num_samples: data.len(),
        }
    }

    fn render(&self) {
        let samples_per_block = self.buckets.get_samples_per_block();
        println!("# each ∎ represents a count of {}", samples_per_block);

        let num_samples_digits = (self.num_samples as f64).log10().ceil() as usize + 1;

        for bucket_ind in 0..self.buckets.buckets.len() {
            let (low, high) = self.buckets.bucket_range(bucket_ind);
            println!(
                "{:10.4} - {:10.4} [{:3$}]: {4}",
                low,
                high,
                self.buckets.buckets[bucket_ind],
                num_samples_digits,
                "∎".repeat(self.buckets.buckets[bucket_ind] / samples_per_block)
            );
        }
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

    if matches.subcommand_matches("histogram").is_some() {
        let histo = Histogram::new(&data, 10);
        histo.render();
    }

    Ok(())
}
