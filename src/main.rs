use std::f64;
use clap::Parser;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Generate a grid of candidate points within the rectangle
fn generate_grid(width: f64, height: f64, step: f64) -> Vec<Point> {
    let mut points = Vec::new();
    let mut y = 0.0;
    while y <= height {
        let mut x = 0.0;
        while x <= width {
            points.push(Point { x, y });
            x += step;
        }
        y += step;
    }
    points
}

// Farthest-First Traversal algorithm
fn farthest_first_traversal(candidates: &[Point], k: usize) -> Vec<Point> {
    let mut centers = Vec::new();

    // Start with the center of the rectangle
    if let Some(&first) = candidates.first() {
        centers.push(first);
    }

    while centers.len() < k {
        let mut best_candidate = None;
        let mut max_distance = -1.0;

        for &candidate in candidates {
            let min_distance = centers
                .iter()
                .map(|center| candidate.distance(center))
                .fold(f64::INFINITY, f64::min);

            if min_distance > max_distance {
                max_distance = min_distance;
                best_candidate = Some(candidate);
            }
        }

        if let Some(candidate) = best_candidate {
            centers.push(candidate);
        } else {
            break;
        }
    }

    centers
}

/// FFTrav: Maximize minimum distance between points in a rectangle
#[derive(Parser, Debug)]
#[command(name = "fftrav", version = "1.0", about = "Maximize minimum distance between points in a rectangle")]
struct Args {
    /// Width of the rectangle
    #[arg(short = 'x', long, default_value_t = 10.0)]
    width: f64,

    /// Height of the rectangle
    #[arg(short = 'y', long, default_value_t = 10.0)]
    height: f64,

    /// Number of points to place
    #[arg(short = 'n', long, default_value_t = 5)]
    num_points: usize,

    /// Grid resolution (step size)
    #[arg(short = 's', long, default_value_t = 1.0)]
    step: f64,
}

fn main() {
    let args = Args::parse();

    let candidates = generate_grid(args.width, args.height, args.step);
    let centers = farthest_first_traversal(&candidates, args.num_points);

    for (i, center) in centers.iter().enumerate() {
        println!("Point {}: ({}, {})", i + 1, center.x, center.y);
    }
}
