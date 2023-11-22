#![allow(unused)]

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use statrs::statistics::Statistics;

/// The benchmark dataset from a ReBench run.
pub struct CandidateDataset {
    /// A map of benchmark names to their measurements.
    benchmarks: BTreeMap<String, BenchmarkMeasurements>,
}

impl CandidateDataset {
    pub fn from_file<P>(path: &P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(path.as_ref())?;
        let iter = contents
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with("#"));

        let mut dataset = BTreeMap::<String, Vec<f64>>::new();
        for line in iter {
            let items = (|| {
                let mut iter = line.split_whitespace();

                let _ = iter.next()?;
                let _ = iter.next()?;
                let value = iter.next()?;
                let unit = iter.next()?;
                let name = iter.next()?;
                let bench = iter.next()?;
                let _ = iter.next()?;

                Some((name, value, unit, bench))
            })();

            let Some((name, value, unit, bench)) = items else {
                eprintln!("discarded record due to bad layout: '{}'", line);
                continue;
            };

            if name != "total" {
                continue;
            }

            let Ok(value) = value.parse() else {
                eprintln!("discarded record due to unparsable value: '{}'", value);
                continue;
            };

            let value = match unit {
                "s" => value * 1_000f64,
                "ms" => value,
                "us" => value / 1_000f64,
                "ns" => value / 1_000_000f64,
                unit => {
                    eprintln!("discarded record due to unknown unit: '{}'", unit);
                    continue;
                }
            };

            dataset
                .entry(bench.to_string())
                .and_modify(|data| data.push(value))
                .or_insert_with(|| vec![value]);
        }

        let benchmarks = dataset
            .into_iter()
            .map(|(bench_name, measurements)| (bench_name, BenchmarkMeasurements { measurements }))
            .collect();

        Ok(CandidateDataset { benchmarks })
    }

    pub fn is_empty(&self) -> bool {
        self.benchmarks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.benchmarks.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &BenchmarkMeasurements)> + '_ {
        self.benchmarks.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> + '_ {
        self.benchmarks.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &BenchmarkMeasurements> + '_ {
        self.benchmarks.values()
    }

    pub fn get(&self, benchmark_name: &str) -> Option<&BenchmarkMeasurements> {
        self.benchmarks.get(benchmark_name)
    }
}

/// The time measurements of a benchmark
pub struct BenchmarkMeasurements {
    /// The actual time measurements of each run.
    measurements: Vec<f64>,
}

impl BenchmarkMeasurements {
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        self.measurements.iter().copied()
    }

    pub fn min(&self) -> f64 {
        assert!(!self.measurements.is_empty());
        Statistics::min(self.measurements.iter())
    }

    pub fn max(&self) -> f64 {
        assert!(!self.measurements.is_empty());
        Statistics::max(self.measurements.iter())
    }

    pub fn mean(&self) -> f64 {
        assert!(!self.measurements.is_empty());
        Statistics::mean(self.measurements.iter())
    }

    pub fn standard_deviation(&self) -> f64 {
        assert!(!self.measurements.is_empty());
        Statistics::std_dev(self.measurements.iter())
    }
}
