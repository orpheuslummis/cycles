use dioxus::prelude::*;
use rand::prelude::*;
use std::time::{Duration, Instant};

const HEXAGON_RADIUS: i32 = 2;

#[component]
pub fn Hexagon() -> Element {
    rsx! {
        FullScreenOfNumbers {}
    }
}

#[component]
fn HexagonGrid() -> Element {
    let mut selected_hex = use_signal(|| None::<usize>);
    let hexagon_positions = generate_hexagon_grid(HEXAGON_RADIUS);

    rsx! {
        div { class: "hexagon-grid",
            for (index , & (q , r)) in hexagon_positions.iter().enumerate() {
                Hexagon {
                    index,
                    q,
                    r,
                    is_selected: selected_hex() == Some(index),
                    on_click: move |_| selected_hex.set(Some(index))
                }
            }
        }
    }
}

#[component]
fn Hexagon(
    index: usize,
    q: i32,
    r: i32,
    is_selected: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let (x, y) = hex_to_pixel(q, r, 60.0); // 60.0 is the size of the hexagon

    rsx! {
        div {
            key: "{index}",
            class: if is_selected { "hexagon selected" } else { "hexagon" },
            style: "left: {x}px; top: {y}px;",
            onclick: move |e| on_click.call(e),
            div { class: "hexagon-content", "{index}" }
        }
    }
}

// Helper functions for hexagonal grid calculations
fn generate_hexagon_grid(radius: i32) -> Vec<(i32, i32)> {
    let mut positions = vec![];
    for q in -radius..=radius {
        for r in -radius..=radius {
            if q.abs() + r.abs() + (q + r).abs() <= 2 * radius {
                positions.push((q, r));
            }
        }
    }
    positions
}

fn hex_to_pixel(q: i32, r: i32, size: f32) -> (f32, f32) {
    let x = size * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    let y = size * (3.0 / 2.0 * r as f32);
    (x, y)
}
