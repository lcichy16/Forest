use crate::forest::{Cell, Forest};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const SIMULATIONS: usize = 100;
const THREADS: usize = 8;

pub fn analyze_all_densities() -> std::io::Result<()> {
    let mut summary_file = File::create("results.csv")?;
    writeln!(summary_file, "density,average_burned")?;

    for density in 1..=100 {
        let density_f64 = density as f64;
        let (tx, rx) = mpsc::channel();

        let mut handles = vec![];

        for thread_id in 0..THREADS {
            let tx = tx.clone();

            let simulations_per_thread = SIMULATIONS / THREADS;
            let start = thread_id * simulations_per_thread;
            let end = start + simulations_per_thread;

            let handle = thread::spawn(move || {
                for _ in start..end {
                    let mut forest = Forest::new(WIDTH, HEIGHT);
                    forest.grow(density_f64);
                    forest.start_fire();

                    while forest.has_burning_trees() {
                        forest.spread_fire();
                    }

                    let burned = forest.burned_percentage();
                    tx.send(burned).unwrap();
                }
            });

            handles.push(handle);
        }

        drop(tx);

        let mut total_burned = 0.0;
        let mut density_file = File::create(format!("results_{:.2}.csv", density_f64))?;
        writeln!(density_file, "trial,burned_percentage")?;

        for (i, burned) in rx.iter().enumerate() {
            writeln!(density_file, "{}, {:.2}", i + 1, burned)?;
            total_burned += burned;
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let average = total_burned / SIMULATIONS as f64;
        writeln!(summary_file, "{:.2},{:.2}", density_f64, average)?;
        println!(
            "Density: {:>5.2}% => avg burned: {:>6.2}%",
            density_f64, average
        );
    }

    Ok(())
}
