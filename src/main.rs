mod forest;
mod point;

use forest::{Cell, Forest};
use macroquad::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

const CELL_SIZE: f32 = 10.0;

#[macroquad::main("Symulacja pożaru lasu")]
async fn main() {
    let mut width: usize = 50;
    let mut height: usize = 50;
    let mut tree_density: f64 = 50.13;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-x" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        width = val;
                    } else {
                        eprintln!("Niepoprawna wartość dla -x");
                        return;
                    }
                    i += 2;
                } else {
                    eprintln!("Brak wartości po -x");
                    return;
                }
            }
            "-y" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        height = val;
                    } else {
                        eprintln!("Niepoprawna wartość dla -y");
                        return;
                    }
                    i += 2;
                } else {
                    eprintln!("Brak wartości po -y");
                    return;
                }
            }
            "-p" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<f64>() {
                        tree_density = val;
                    } else {
                        eprintln!("Niepoprawna wartość dla -p");
                        return;
                    }
                    i += 2;
                } else {
                    eprintln!("Brak wartości po -p");
                    return;
                }
            }
            _ => {
                eprintln!("Nieznany argument: {}", args[i]);
                i += 1;
            }
        }
    }

    println!("Ustawienia: szerokość: {}, wysokość: {}, gęstość drzew: {}%", width, height, tree_density);

    let mut forest = Forest::new(width, height);
    forest.grow(tree_density);
    // Nie uruchamiamy pożaru od razu

    let mut frame_counter = 0;
    let frames_between_spread = 10;
    let mut simulation_done = false;
    let mut burned_percent = 0.0;
    let mut waiting_for_start = true;

    let screen_width = screen_width();
    let screen_height = screen_height();
    let offset_x = (screen_width - (width as f32 * CELL_SIZE)) / 2.0;
    let offset_y = (screen_height - (height as f32 * CELL_SIZE)) / 2.0;

    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if simulation_done && is_key_pressed(KeyCode::R) {
            forest = Forest::new(width, height);
            forest.grow(tree_density);
            waiting_for_start = true;
            frame_counter = 0;
            simulation_done = false;
            burned_percent = 0.0;
        }

        for y in 0..height {
            for x in 0..width {
                let cell = forest.grid[y][x];
                let color = match cell {
                    Cell::Empty => BLACK,
                    Cell::Tree => GREEN,
                    Cell::Burning => ORANGE,
                    Cell::Burned => RED,
                };

                draw_rectangle(
                    offset_x + x as f32 * CELL_SIZE,
                    offset_y + y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }

        if waiting_for_start {
            let prompt = "Wcisnij ENTER, aby rozpoczac pozar...";
            let prompt_dim = measure_text(prompt, None, 40, 1.0);

            draw_text(
                prompt,
                (screen_width - prompt_dim.width) / 2.0,
                screen_height - 50.0,
                40.0,
                WHITE,
            );

            if is_key_pressed(KeyCode::Enter) {
                forest.start_fire();
                waiting_for_start = false;
            }

            next_frame().await;
            continue;
        }

        if !simulation_done && frame_counter % frames_between_spread == 0 {
            forest.spread_fire();

            if !forest.has_burning_trees() {
                simulation_done = true;
                burned_percent = forest.burned_percentage();

                if let Err(e) = save_result(tree_density, burned_percent) {
                    eprintln!("Błąd zapisu: {}", e);
                } else {
                    println!("Wynik zapisany do results.csv");
                }
            }
        }

        if simulation_done {
            let text1 = format!("Spalono {:.2}% lasu", burned_percent);
            let text2 = "Nacisnij R aby uruchomic ponownie, ESC aby zakonczyc...";

            let text1_dim = measure_text(&text1, None, 40, 1.0);
            let text2_dim = measure_text(text2, None, 24, 1.0);

            draw_text(
                &text1,
                (screen_width - text1_dim.width) / 2.0,
                screen_height - 570.0,
                40.0,
                WHITE,
            );

            draw_text(
                text2,
                (screen_width - text2_dim.width) / 2.0,
                screen_height - 20.0,
                24.0,
                WHITE,
            );
        }

        frame_counter += 1;
        next_frame().await;
    }
}

fn save_result(density: f64, burned: f64) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("results.csv")?;

    if file.metadata()?.len() == 0 {
        writeln!(file, "density,burned_percentage")?;
    }

    writeln!(file, "{:.2},{:.2}", density, burned)?;
    Ok(())
}
