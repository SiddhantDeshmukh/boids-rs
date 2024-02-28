use std::f32::consts::PI;

use egui_macroquad::egui::Widget;
use boids::*;
use macroquad::prelude::*;
use egui_macroquad::egui;
use ::rand::rngs::ThreadRng;

fn reset(rng: &mut ThreadRng) -> (Params, Vec<Boid>) {
    // Initialise parameters and boid positions
    let params = Params::default();
    let bounds = Bounds {x_min: 0., x_max: screen_width(), y_min: 0., y_max: screen_height()};
    // Create boids
    let num_boids = 200;
    let boids_pop: Vec<Boid> = (0..num_boids).map(|_| random_boid(rng, &bounds)).collect::<Vec<Boid>>();
    return (params, boids_pop);
}

fn draw_fps(x: f32, y: f32, font_size: f32) {
    let fps = get_fps();
    let c = match fps {
        0..=10 => RED,
        11..=30 => ORANGE,
        _ => GREEN
    };
    draw_text(format!("FPS: {}", fps).as_str(), x, y, font_size, c)
}

macro_rules! slider {
    ($value:expr, $min:expr, $max:expr, $ui:expr) => {
        egui::Slider::new($value, std::ops::RangeInclusive::new($min, $max)).ui($ui);
    };
}

#[macroquad::main("Boids")]
async fn main() {
    // For rendering the window
    // Set up initialisation
    let mut rng: ThreadRng = ::rand::thread_rng();
    let (mut params, mut boids_pop) = reset(&mut rng);   
    let window_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 255);

    loop {
        // Dynamic screen sizing
        params.window_width = screen_width();
        params.window_height = screen_height();
        clear_background(BLACK);

        // Update boids
        boids_pop = update_boids(&params, &boids_pop);

        // Render boids
        for boid in boids_pop.iter() {
            let velocity = Vec2::new(boid.velocity.x, boid.velocity.y).normalize();
            let angle = velocity.y.atan2(velocity.x) * 180. / PI;
            draw_poly(boid.position.x, boid.position.y, 3, 6., angle, boid.color);
        }

        // EGUI Window
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .frame(egui::Frame{fill: window_color, ..Default::default()})
                .fixed_pos(egui::pos2(12., 24.))
                .show(egui_ctx, |ui| {
                    // Reset button
                    if ui.button("Reset").clicked() {
                        (params, boids_pop) = reset(&mut rng);
                    }
                    // Coherence
                    ui.horizontal(|ui| {
                        ui.label("Coherence");
                        slider!(&mut params.centering_factor, 0., 0.01, ui);
                    });
                    // Separation
                    ui.horizontal(|ui| {
                        ui.label("Separation");
                        slider!(&mut params.avoid_factor, 0., 0.1, ui);
                    });
                    // Alignment
                    ui.horizontal(|ui| {
                        ui.label("Alignment");
                        slider!(&mut params.matching_factor, 0., 0.1, ui);
                    });
                    // Visual Range
                    ui.horizontal(|ui| {
                        ui.label("Visual Range");
                        slider!(&mut params.visual_range, 3., 60., ui);
                    });
                });
        });

        // Draw EGUI
        egui_macroquad::draw();

        // Draw last macroquad
        draw_fps(params.window_width - 120., 20., 32.);
        next_frame().await
    }
}
