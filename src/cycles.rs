use dioxus::prelude::*;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use tokio::time::{interval, Duration};

const GRID_SIZE: usize = 21;
const TOTAL_CHARS: usize = GRID_SIZE * GRID_SIZE;
const TARGET_FRAME_TIME: Duration = Duration::from_micros(150000);

const CHARACTERS: [char; 20] = [
    '凧', '矢', '日', '星', '空', '風', '雨', '雪', '山', '川', '火', '水', '木', '金', '土', '月',
    '花', '鳥', '魚', '龍',
];

#[derive(Clone)]
struct ColoredChar {
    character: char,
    color: String,
    energy: f64,
    phase: f64,
    quasicrystal_value: f64,
}

#[component]
pub fn Cycles() -> Element {
    rsx! {
        FullScreenOfChars {}
    }
}

#[component]
fn FullScreenOfChars() -> Element {
    let chars = use_signal(|| initialize_grid());

    use_future(move || {
        to_owned![chars];
        async move {
            let mut rng = rand::thread_rng();
            let mut interval = interval(TARGET_FRAME_TIME);

            loop {
                interval.tick().await;
                chars.with_mut(|cs| apply_exotic_automata(cs, &mut rng));
            }
        }
    });

    rsx! {
        div { class: "full-screen-chars",
            for (index , colored_char) in chars.iter().enumerate() {
                div { key: "{index}", class: "char", style: "color: {colored_char.color}", "{colored_char.character}" }
            }
        }
    }
}

fn initialize_grid() -> Vec<ColoredChar> {
    let mut rng = rand::thread_rng();
    (0..TOTAL_CHARS)
        .map(|_| {
            let char = *CHARACTERS.choose(&mut rng).unwrap();
            ColoredChar {
                character: char,
                color: get_color_for_char(char, rng.gen()),
                energy: rng.gen(),
                phase: rng.gen(),
                quasicrystal_value: rng.gen(),
            }
        })
        .collect()
}

fn apply_exotic_automata(grid: &mut Vec<ColoredChar>, rng: &mut ThreadRng) {
    let mut new_grid = grid.clone();
    let global_energy = grid.iter().map(|c| c.energy).sum::<f64>() / TOTAL_CHARS as f64;
    let global_phase = grid.iter().map(|c| c.phase).sum::<f64>() / TOTAL_CHARS as f64;
    let global_quasicrystal =
        grid.iter().map(|c| c.quasicrystal_value).sum::<f64>() / TOTAL_CHARS as f64;

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let idx = i * GRID_SIZE + j;
            let neighbors = get_neighbors(i, j);
            let neighbor_counts = count_neighbors(&neighbors, grid);

            let energy_sum: f64 = neighbors.iter().map(|&n| grid[n].energy).sum();
            let phase_sum: f64 = neighbors.iter().map(|&n| grid[n].phase).sum();
            let quasicrystal_sum: f64 = neighbors.iter().map(|&n| grid[n].quasicrystal_value).sum();
            let avg_energy = energy_sum / neighbors.len() as f64;
            let avg_phase = phase_sum / neighbors.len() as f64;
            let avg_quasicrystal = quasicrystal_sum / neighbors.len() as f64;

            new_grid[idx] = determine_new_state(
                &grid[idx],
                &neighbor_counts,
                avg_energy,
                avg_phase,
                avg_quasicrystal,
                global_energy,
                global_phase,
                global_quasicrystal,
                rng,
            );
        }
    }
    *grid = new_grid;
}

fn determine_new_state(
    current: &ColoredChar,
    neighbor_counts: &HashMap<char, usize>,
    avg_energy: f64,
    avg_phase: f64,
    avg_quasicrystal: f64,
    global_energy: f64,
    global_phase: f64,
    global_quasicrystal: f64,
    rng: &mut ThreadRng,
) -> ColoredChar {
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let e = std::f64::consts::E;
    let pi = std::f64::consts::PI;

    let energy_factor = (current.energy * phi).sin().abs();
    let phase_factor = (current.phase * e).cos().abs();
    let quasicrystal_factor = (current.quasicrystal_value * pi).tan().abs();

    // Ensure the probability is within the valid range
    let change_prob = (0.02 + quasicrystal_factor * 0.08).clamp(0.0, 1.0);
    let new_char = if rng.gen_bool(change_prob) {
        *CHARACTERS.choose(rng).unwrap()
    } else {
        current.character
    };

    let new_energy = (current.energy + avg_energy + global_energy * phase_factor).fract();
    let new_phase = (current.phase + avg_phase + global_phase * quasicrystal_factor).fract();
    let new_quasicrystal =
        (current.quasicrystal_value + avg_quasicrystal + global_quasicrystal * energy_factor)
            .fract();

    let combined_factor = (new_energy * new_phase * new_quasicrystal).powf(1.0 / 3.0);

    ColoredChar {
        character: new_char,
        color: get_color_for_char(new_char, combined_factor),
        energy: new_energy,
        phase: new_phase,
        quasicrystal_value: new_quasicrystal,
    }
}

fn get_color_for_char(c: char, combined_factor: f64) -> String {
    let hue = match c {
        '凧' => 0.0,
        '矢' => 18.0,
        '日' => 36.0,
        '星' => 54.0,
        '空' => 72.0,
        '風' => 90.0,
        '雨' => 108.0,
        '雪' => 126.0,
        '山' => 144.0,
        '川' => 162.0,
        '火' => 180.0,
        '水' => 198.0,
        '木' => 216.0,
        '金' => 234.0,
        '土' => 252.0,
        '月' => 270.0,
        '花' => 288.0,
        '鳥' => 306.0,
        '魚' => 324.0,
        '龍' => 342.0,
        _ => 0.0,
    };
    let saturation = 70.0 + combined_factor * 30.0;
    let lightness = 20.0 + combined_factor * 60.0;
    format!("hsl({}, {}%, {}%)", hue, saturation, lightness)
}

fn get_neighbors(i: usize, j: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            let ni = (i as isize + di + GRID_SIZE as isize) % GRID_SIZE as isize;
            let nj = (j as isize + dj + GRID_SIZE as isize) % GRID_SIZE as isize;
            neighbors.push(ni as usize * GRID_SIZE + nj as usize);
        }
    }
    neighbors
}

fn count_neighbors(neighbors: &[usize], grid: &[ColoredChar]) -> HashMap<char, usize> {
    neighbors
        .iter()
        .map(|&n| grid[n].character)
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}
