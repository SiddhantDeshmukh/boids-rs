use macroquad::prelude::*;
use ::rand::{rngs::ThreadRng, Rng};

// World size
pub const X_SIZE: f32 = 100.;
pub const Y_SIZE: f32 = 100.;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
}

impl Boid {
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }
    // Scale positions to window size for drawing, divide by domain size
    fn scale_position(&self, value: f32, window_size: f32) -> f32 {
        value * window_size as f32
    }

    pub fn win_pos(&self, window_width: f32, window_height: f32) -> Vec2 {
        vec2(
            self.scale_position(self.position.x, window_width) / X_SIZE,
            self.scale_position(self.position.y, window_height) / Y_SIZE,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub coherence: f32,
    pub separation: f32,
    pub avoid_factor: f32,
    pub alignment: f32,
    pub visual_range: f32,
    pub speed_limit: f32,
    pub margin: Vec2,
    pub turn_factor: f32,
}

impl Default for Params {
    fn default() -> Params {
        Params {
            coherence: 0.05,
            separation: 0.5,
            avoid_factor: 0.05,
            alignment: 0.5,
            visual_range: 10.,
            speed_limit: 1.,
            margin: vec2(0.01 * X_SIZE, 0.01 * Y_SIZE),
            turn_factor: 100.,
        }
    }
}
pub struct Bounds {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl Bounds {
    pub fn whole_domain() -> Self {
        // Entire window is valid
        Bounds {
            x_min: 0.,
            x_max: X_SIZE,
            y_min: 0.,
            y_max:Y_SIZE,
        }
    }
}


// Coordinate transforms
pub fn win_to_world(vec: Vec2, window_width: f32, window_height: f32) -> Vec2 {
    // Convert from window coordinates to world coords (between 0 and 1)
    Vec2::new(
        vec.x * X_SIZE / window_width,
        vec.y * Y_SIZE / window_height,
    )
}

pub fn world_to_win(vec: Vec2, window_width: f32, window_height: f32) -> Vec2 {
    Vec2::new(
        vec.x / X_SIZE * window_width,
        vec.y / Y_SIZE * window_height,
    )
}

// Numerics
pub fn range_scale(v: f32, old_low: f32, old_hi: f32, new_low: f32, new_hi: f32) -> f32 {
    // Scale 'v' from ['old_low', 'old_hi'] to ['new_low', 'new_hi']
    new_low + v * (new_hi - new_low) / (old_hi - old_low)
}

pub fn rvec2_range(rng: &mut ThreadRng, bounds: &Bounds) -> Vec2 {
    vec2(
        range_scale(rng.gen::<f32>(), 0., 1., bounds.x_min, bounds.x_max),
        range_scale(rng.gen::<f32>(), 0., 1., bounds.y_min, bounds.y_max),
    )
}

pub fn random_boid(rng: &mut ThreadRng, bounds: &Bounds) -> Boid {
    Boid {
        position: rvec2_range(rng, &bounds),
        velocity: rvec2_range(rng, &Bounds{
            x_min: -5., x_max: 5.,
            y_min: -5., y_max: 5.,
        }),
        color: WHITE  // later change
    }
}

// Helper functions

// Helper functions
fn distance(boid1: &Boid, boid2: &Boid) -> f32 {
    boid1.position.distance(boid2.position)
}

fn boids_in_range(boid1: &Boid, radius: f32, boids_pop: &Vec<Boid>) -> Vec<Boid> {
    let boids_in_range: Vec<Boid> = boids_pop
        .iter()
        .filter(|b2| **b2 != *boid1 && distance(&boid1, &b2) < radius)
        .cloned()
        .collect();

    boids_in_range
}

fn flock_center(boids_pop: &Vec<Boid>) -> Vec2 {
    // Find the center of a flock of boids
    if boids_pop.is_empty() {
        vec2(0., 0.)
    } else {
        let mut center: Vec2 = vec2(0., 0. );
        let num_boids = boids_pop.len();
        for boid in boids_pop {
            center += boid.position;
        }
        center /= num_boids as f32;
        center
    }
}

// Simulation functions
fn fly_towards_centre(boid: &mut Boid, params: &Params, boids_pop: &Vec<Boid>) {
    // Towards flock centre using coherence
    let center = flock_center(boids_pop);
    boid.velocity += (center - boid.position) * params.coherence;
}

fn avoid_others(boid: &mut Boid, params: &Params, boids_pop: &Vec<Boid>) {
    // No crashing using separation
    let avoidance_correction: Vec2 = boids_pop.iter()
        .filter(|b2| distance(boid, b2) < params.separation)
        .fold(vec2(0., 0.),
            |acc, b2| acc + boid.position - b2.position);
    boid.velocity += avoidance_correction * params.avoid_factor;
}

fn match_velocity(boid: &mut Boid, params: &Params, boids_pop: &Vec<Boid>) {
    // Fly like the other boids using alignment
    let avg_velocity: Vec2 = boids_pop.iter()
        .fold(vec2(0., 0.), |acc, b2| acc + b2.velocity);
    boid.velocity += (avg_velocity - boid.velocity) * params.alignment;
}

fn limit_speed(boid: &mut Boid, params:  &Params) {
    // Not too fast (CFL?)
    if boid.speed() > params.speed_limit {
        boid.velocity = boid.velocity / boid.speed() * params.speed_limit;
    }
}

fn keep_within_bounds(boid: &mut Boid, params: &Params) {
    // Don't leave the simulation bounds
    let mut velocity_change = Vec2::ZERO;

    if boid.position.x < params.margin.x {
        velocity_change.x += params.turn_factor;
    } else if boid.position.x > X_SIZE - params.margin.x {
        velocity_change.x -= params.turn_factor;
    }

    if boid.position.y < params.margin.y {
        velocity_change.y += params.turn_factor;
    } else if boid.position.y > Y_SIZE - params.margin.y {
        velocity_change.y -= params.turn_factor;
    }

    boid.velocity += velocity_change;
}

pub fn update_boids(params: &Params, boids_pop: &Vec<Boid>) -> Vec<Boid> {
    let new_boids: Vec<Boid> = boids_pop
        .iter()
        .map(|b1| {
            let nearby_boids = boids_in_range(b1, params.visual_range, boids_pop);
            let mut new_b = *b1;
            fly_towards_centre(&mut new_b, params, &nearby_boids);
            avoid_others(&mut new_b, params, &nearby_boids);
            match_velocity(&mut new_b, params, &nearby_boids);
            keep_within_bounds(&mut new_b, params);
            limit_speed(&mut new_b, params);
            new_b.position += new_b.velocity; // time step?
            new_b
        })
        .collect();
    new_boids
}