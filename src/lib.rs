use macroquad::prelude::*;
use ::rand::{rngs::ThreadRng, Rng};


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
}

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub centering_factor: f32,
    pub min_distance: f32,
    pub avoid_factor: f32,
    pub matching_factor: f32,
    pub visual_range: f32,
    pub speed_limit: f32,
    pub margin: Vec2,
    pub turn_factor: f32,
    pub window_width: f32,
    pub window_height: f32
}

impl Default for Params {
    fn default() -> Params {
        Params {
            centering_factor: 0.005,
            min_distance: 20.,
            avoid_factor: 0.05,
            matching_factor: 0.05,
            visual_range: 30.,
            speed_limit: 5.,
            margin: vec2(50., 50.),
            turn_factor: 1.,
            window_width: 600.,
            window_height: 400.
        }
    }
}
pub struct Bounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
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
    let boid_colors = [
        WHITE,
        BLUE,
        BROWN,
        GOLD,
        RED,
        GREEN
    ];
    Boid {
        position: rvec2_range(rng, &bounds),
        velocity: rvec2_range(rng, &Bounds{
            x_min: -5., x_max: 5.,
            y_min: -5., y_max: 5.,
        }),
        color: boid_colors[rng.gen_range(0..boid_colors.len())] // random
    }
}

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
    boid.velocity += (center - boid.position) * params.centering_factor;
}

fn avoid_others(boid: &mut Boid, params: &Params, boids_pop: &Vec<Boid>) {
    // No crashing using separation
    let avoidance_correction: Vec2 = boids_pop.iter()
        .filter(|b2| distance(boid, b2) < params.min_distance)
        .fold(vec2(0., 0.),
            |acc, b2| acc + boid.position - b2.position);
    boid.velocity += avoidance_correction * params.avoid_factor;
}

fn match_velocity(boid: &mut Boid, params: &Params, boids_pop: &Vec<Boid>) {
    // Fly like the other boids using alignment
    let avg_velocity: Vec2 = boids_pop.iter()
        .fold(vec2(0., 0.), |acc, b2| acc + b2.velocity);
    boid.velocity += (avg_velocity - boid.velocity) * params.matching_factor;
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
    } else if boid.position.x > params.window_width - params.margin.x {
        velocity_change.x -= params.turn_factor;
    }

    if boid.position.y < params.margin.y {
        velocity_change.y += params.turn_factor;
    } else if boid.position.y > params.window_height - params.margin.y {
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
            avoid_others(&mut new_b, params, boids_pop);
            match_velocity(&mut new_b, params, &nearby_boids);
            keep_within_bounds(&mut new_b, params);
            limit_speed(&mut new_b, params);
            // new_b.velocity += rvec2_range(&mut rng, &Bounds {x_min: -1., x_max: 1., y_min: -1., y_max: 1.});
            new_b.position += new_b.velocity; // time step?
            new_b
        })
        .collect();
    new_boids
}