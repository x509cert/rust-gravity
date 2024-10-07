use macroquad::prelude::*;

const N:u32 = 75;

fn random_color() -> Color {
    
    // Generate random values for red, green, blue, and alpha (RGBA)
    let r: f32 = rand::gen_range(0.0, 1.0); 
    let g: f32 = rand::gen_range(0.0, 1.0);
    let b: f32 = rand::gen_range(0.0, 1.0);
    let a: f32 = rand::gen_range(0.0, 1.0);  
    
    Color { r, g: g, b: b, a: a } 
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Circles with Gravity".to_owned(),
        fullscreen: true, 
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    struct Circle {
        position: Vec2,
        velocity: Vec2,
        radius: f32,
        mass: f32,
        color: Color,
    }

    // Gravitational constant to control the strength of attraction
    let mut gravitational_constant: f32 = 5000.0;

    // screen center
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    let mut circles = Vec::new();

    for _ in 1..N {

        let x = rand::gen_range(cx - cx/10.0, cx + cx/10.0);
        let y = rand::gen_range(cy - cy/10.0, cy + cy/10.0);

        let vx = rand::gen_range(-0.1, 0.1);
        let vy = rand::gen_range(-0.1, 0.1);

        let m = rand::gen_range(8.0,75.0);
        
        let c = Circle {
            position: vec2(x,y),
            velocity: vec2(vx,vy),
            radius: m / 2.0,
            mass: m,
            color: random_color(),
        };

        circles.push(c);
    }

    loop {

        // Check if the UP or DOWN arrow key is pressed to adjust gravity
        if is_key_down(KeyCode::Up) {
            gravitational_constant += 50.0; // Increase gravity
        }

        if is_key_down(KeyCode::Down) {
            gravitational_constant -= 50.0; // Decrease gravity, but keep it positive
            if gravitational_constant < 0.0 {
                gravitational_constant = 0.0;
            }
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        let dt = get_frame_time();

        // Calculate gravitational forces
        let forces = {
            let mut forces = vec![Vec2::ZERO; circles.len()];

            for i in 0..circles.len() {
                for j in (i + 1)..circles.len() {
                    let direction = circles[j].position - circles[i].position;
                    let distance_sq = direction.length_squared();

                    // Avoid division by zero or extremely large forces
                    if distance_sq > 1.0 {
                        let force_magnitude = (gravitational_constant * circles[i].mass * circles[j].mass) / distance_sq;
                        let force = direction.normalize() * force_magnitude;

                        forces[i] += force;
                        forces[j] -= force; // Newton's third law
                    }
                }
            }
            forces
        };

        // Apply a damping factor to slow down the circles over time
        for i in 0..circles.len() {
            let velocity = circles[i].velocity; // Copy velocity to avoid mutable and immutable borrow conflict
            let acceleration = forces[i] / circles[i].mass;
            circles[i].velocity += acceleration * dt;
            circles[i].velocity *= 0.99; // Apply damping factor to reduce velocity over time
            circles[i].position += velocity * dt;

            // Bounce off the window edges
            if circles[i].position.x - circles[i].radius < 0.0 {
                circles[i].position.x = circles[i].radius;
                circles[i].velocity.x *= -1.0;
            }

            if circles[i].position.x + circles[i].radius > screen_width() {
                circles[i].position.x = screen_width() - circles[i].radius;
                circles[i].velocity.x *= -1.0;
            }

            if circles[i].position.y - circles[i].radius < 0.0 {
                circles[i].position.y = circles[i].radius;
                circles[i].velocity.y *= -1.0;
            }

            if circles[i].position.y + circles[i].radius > screen_height() {
                circles[i].position.y = screen_height() - circles[i].radius;
                circles[i].velocity.y *= -1.0;
            }
        }

        // Clear the screen
        clear_background(WHITE);

        // Draw circles
        for circle in &circles {
            draw_circle(circle.position.x, circle.position.y, circle.radius, circle.color);
        }

        // Draw FPS counter
        draw_text(&format!("FPS: {:.0}", get_fps()), 10.0, 20.0, 30.0, BLACK);

        // Gravity
        draw_text(&format!("Gravity: {:.0}", gravitational_constant), 10.0, 50.0, 30.0, BLACK);

        // Draw number of circles
        draw_text(&format!("Circles: {}", circles.len()), 10.0, 80.0, 30.0, BLACK);

        // Calculate and draw average speed
        let total_speed: f32 = circles.iter().map(|c| c.velocity.length()).sum();
        let average_speed = total_speed / circles.len() as f32;
        draw_text(&format!("Avg Speed: {:.2}", average_speed), 10.0, 110.0, 30.0, BLACK);

        // Calculate and draw total kinetic energy
        let total_kinetic_energy: f32 = circles.iter().map(|c| 0.5 * c.mass * c.velocity.length_squared()).sum();
        draw_text(&format!("Total Energy: {:.2}", total_kinetic_energy), 10.0, 140.0, 30.0, BLACK);

        // Draw frame time
        let frame_time_ms = dt * 1000.0;
        draw_text(&format!("Frame Time: {:.2} ms", frame_time_ms), 10.0, 170.0, 30.0, BLACK);

        next_frame().await;
    }
}