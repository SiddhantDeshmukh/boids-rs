use boids::*;
use macroquad::prelude::*;
use ::rand::{rngs::ThreadRng, Rng};

fn init(rng: &mut ThreadRng) -> (Params, Vec<Boid>) {
    // Initialise parameters and boid positions
    let params = Params::default();
    let bounds = Bounds::whole_domain();
    // Create boids
    let num_boids = 50;
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

#[macroquad::main("Boids")]
async fn main() {
    // For rendering the window
    let mut window_width;
    let mut window_height;
    // Set up initialisation
    let mut rng: ThreadRng = ::rand::thread_rng();
    let (params, mut boids_pop) = init(&mut rng);

    loop {
        // Dynamic screen sizing
        window_width = screen_width();
        window_height = screen_height();
        clear_background(BLACK);

        // Update boids
        boids_pop = update_boids(&params, &boids_pop);

        // Render boids
        for boid in boids_pop.iter() {
            let position = Vec2::from(boid.win_pos(window_width, window_height));
            let velocity = Vec2::new(boid.velocity.x, boid.velocity.y).normalize();
            let angle = velocity.angle_between(Vec2::new(1.0, 0.0)).acos(); // Angle between velocity and x-axis

            let cone_color = Color::from_rgba(50, 128, 230, 255);
            draw_circle(position.x, position.y, 4., cone_color);
            // draw_poly_lines(
            //     position.x, position.y,
            //     3, 10., angle, 1., cone_color)
        }

        draw_fps(8., 8., 12.);
        next_frame().await
    }
}
