extern crate rand;

use rand::Rng;

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

#[derive(Debug)]
pub struct Data {
    pub data: Vec<f64>,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub variance: f64,
    pub median: f64,
}

impl Data {
    pub fn new(mut data: Vec<f64>) -> Self {
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

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn stddev(&self) -> f64 {
        self.variance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use permutohedron;

    #[test]
    fn test_find_nth() {
        let mut v: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let permutations = permutohedron::Heap::new(&mut v);
        for mut vp in permutations {
            for i in 0..5 {
                for _ in 0..100 {
                    assert_eq!(find_nth(0, vp.len(), i, &mut vp), i as f64 + 1.);
                }
            }
        }
    }

    #[test]
    fn median_even() {
        for _ in 0..1000 {
            let mut v: Vec<f64> = vec![1., 2., 3., 4.];
            assert_eq!(find_median(&mut v), 2.5);
        }
    }

    #[test]
    fn median_odd() {
        for _ in 0..1000 {
            let mut v: Vec<f64> = vec![1., 2., 3.];
            assert_eq!(find_median(&mut v), 2.);
        }
    }
}
