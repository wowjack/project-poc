use macroquad::prelude::*;
use macroquad::math::Vec2;

/// Simulation bodies are defined as follows
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    mass: f32,
    color: Color
}



// TODO functions for participants to implement


const G: f32 = 6.6743e-11;
// Simple math operation
// Given two bodies, return gravitational force each experiences
// F = G *(m1 * m2) / r^2
fn calculate_gravitational_force(body1: &Body, body2: &Body) -> f32 {
    /*
    let distance_sq = body1.position.distance_squared(body2.position);
    return G * body1.mass * body2.mass / distance_sq;
    */

    todo!()
}



// Iterable operation
// Iterate over the vec of bodies and update position based on velocity
// p2 = p1 + v1*time_step
fn update_body_positions(bodies: &mut Vec<Body>) {
    /*
    for body in bodies {
        body.position += body.velocity;
    }
    */

    todo!()
}



// Nested iteration
// Update the velocity of each body based on the gravitational acceleration exerted by all other bodies
// v2 = v1 + a*time_step
fn update_body_velocities(bodies: &mut Vec<Body>) {
    /*
    for i in 0..bodies.len() {
        let (first, second) = bodies.split_at_mut(i); // split into [0, i) and [i, len)
        let Some((body, second)) = second.split_first_mut() else { continue }; // split [i, len) into i and [i+1, len)

        // Fold bodies from first and second into sum of accelerations exerted on body
        let accel = first.iter()
            .chain(second.iter())
            .fold(Vec2::ZERO, |accel, other| accel + get_acceleration_components(body, other) );

        body.velocity += accel;
    }
    */

    todo!()
}










// Boilerplate for drawing

#[macroquad::main("NBody sim poc")]
async fn main() {
    let centerx = screen_width()/2.;
    let centery = screen_height()/2.;

    let mut bodies = vec![
        Body { position: Vec2::new(centerx-100., centery), radius: 15., mass: 1e12, ..Body::default() },
        Body { position: Vec2::new(centerx+100., centery), velocity: Vec2::new(0., -0.4), ..Body::default() }
    ];

    loop {
        clear_background(DARKGRAY);

        for body in &bodies {
            draw_circle(body.position.x, body.position.y, body.radius, body.color);
        }

        update_body_positions(&mut bodies);
        update_body_velocities(&mut bodies);
        collide_bodies(&mut bodies);

        next_frame().await
    }
}



/// Default body implementation
impl Default for Body {
    fn default() -> Self {
        Self { position: Default::default(), velocity: Default::default(), radius: 10., mass: 1., color: WHITE }
    }
}


/// Get the x and y acceleration other exerts on body \
/// Use the gravitational force function implemented by the participant
fn get_acceleration_components(body: &Body, other: &Body) -> Vec2 {
    let acceleration = calculate_gravitational_force(body, other) / body.mass;
    let angle = (other.position - body.position).to_angle(); //potentially fails in the unlikely situation body bodies have the same position
    return Vec2::new(
        acceleration * angle.cos(),
        acceleration * angle.sin()
    )
}


fn collide_bodies(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        let (_, second) = bodies.split_at_mut(i);
        let Some((body1, second)) = second.split_first_mut() else { continue };

        for body2 in second {
            let diff = body2.position - body1.position;
            let distance = diff.length();

            if distance > body1.radius + body2.radius { continue }

            let norm = diff / distance;
            let rel_vel = body1.velocity - body2.velocity;
            let v_norm = rel_vel.dot(norm);

            let scale_vec = v_norm * norm / (body1.mass + body2.mass) * 0.85;

            body1.velocity -= (2. * body2.mass) * scale_vec;
            body2.velocity += (2. * body1.mass) * scale_vec;
        }
    }
}

