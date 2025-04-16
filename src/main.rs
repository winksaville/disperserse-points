use std::f64;
use std::fmt::Debug;
use clap::Parser;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {{x: {:0.2?}, y: {:0.2?}}})", self.x, self.y)
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

// My problem, how to print N things in a rectangle so that they
// are as far apart as possible but still within the rectangle.
//
// The vscode take on the Farthest-First Traversal algorithm:
//   Farthest-First Traversal algorithm to find the best position of k points
//   such that the minimum distance between any two points is maximized.
//   This is a greedy algorithm that iteratively selects the point that is
//   farthest from the already selected points.
//   The algorithm starts with an initial point and then adds points one by one,
//   each time selecting the point that is farthest from the already selected points.
//   The process continues until k points are selected or no more candidates are available.
fn farthest_first_traversal(candidates: &[Point], k: usize) -> Vec<Point> {
    let mut centers = Vec::new();

    // Start with the center of the rectangle
    if let Some(&first) = candidates.first() {
        centers.push(first);
    }
    println!("Starting point: ({:0.2?}, {:0.2?})", centers[0].x, centers[0].y);

    fn my_min(a: f64, b: f64) -> f64 {
        let min = if a < b { a } else { b };
        println!("my_min of {:0.2?} and {:0.2?} is {:0.2?}", a, b, min);
        min
    }

    while centers.len() < k {
        println!("TOL CENTERS: centers.len {} < {k} center: {:?})", centers.len(), centers );
        let mut best_candidate = None;
        let mut max_distance = -1.0;

        for (idx, &candidate ) in candidates.iter().enumerate() {
            println!("TOL candidates: {idx}: ({:0.2?}, {:0.2?})", candidate.x, candidate.y);

            // Compare the distance from the current candidate point to each of
            // the centers points and find the minimum.
            let min_distance = centers
                .iter()
                .map(|center| {
                    // Calculate the distance between the candidate and the center
                    let distance = candidate.distance(center);
                    println!("Distance from ({:0.2?}, {:0.2?}) to ({:0.2?}, {:0.2?}) is {:0.2?}", candidate.x, candidate.y, center.x, center.y, distance);
                    distance
                })
                .fold(f64::INFINITY, my_min); //f64::min);

            // Now we check if this candidate Point is the farthest from the
            // current centers Points, if so, we add it to the list of centers.
            if min_distance > max_distance {
                max_distance = min_distance;
                best_candidate = Some(candidate);
                println!("New best candidate: ({:0.2?}, {:0.2?}), min distance: {min_distance:0.2?} > max_distance {max_distance:0.2?}", candidate.x, candidate.y);
            } else {
                println!("Reject   candidate: ({:0.2?}, {:0.2?}), min distance: {min_distance:0.2?} < max_distance {max_distance:0.2?}", candidate.x, candidate.y);
            }
        }

        if let Some(candidate) = best_candidate {
            println!("** Adding candidate: ({:0.2?}, {:0.2?}) with distance {:0.2?}", candidate.x, candidate.y, max_distance);
            centers.push(candidate);
        } else {
            println!("No more candidates available.");
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
        println!("Point {}: ({:0.2?}, {:0.2?})", i + 1, center.x, center.y);
    }
}
