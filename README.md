# Disperserse points

I'm working with ChatGPT 4.0 to disperse points
in a 2D space. The goal is to take a set of points and disperse them
in a way that they are evenly distributed across the space with maximal
distance between them.

See this [conversation](https://chatgpt.com/share/67fdd976-d4b4-800c-8b51-ba50899e95a3)

This is simplified with the space defined as rectangle and
the points must be inside the rectangle. Later the points will
be a polygon initially a square although more complex polygons
could be used later.

This has some limitations:
- The arithmetic intorduces some floating point errors.
- The points and it would seem there could be small errors rounding up
  which might cause points to be outside the rectangle. My goal is the
  can be "on the line" but not over.
- If the steps are too large they are not evenly distributed and "wrong"
  see the 3x3 9 points and step 1 example below.

I think we should have the algorithm to pick the step. For use for
my 3d printer the point with actaully be a polygon initiall a square
and I the squares size will be a multiple of the printer reslolution
so a rounding algorithm is going to be needed.

Anyway on first blush the bot did produce a working algorithm.


## Usage

```bash
$ cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/disperserse-points --help`
Maximize minimum distance between points in a rectangle

Usage: disperserse-points [OPTIONS]

Options:
  -x, --width <WIDTH>            Width of the rectangle [default: 10]
  -y, --height <HEIGHT>          Height of the rectangle [default: 10]
  -n, --num-points <NUM_POINTS>  Number of points to place [default: 5]
  -s, --step <STEP>              Grid resolution (step size) [default: 1]
  -h, --help                     Print help
  -V, --version                  Print version
```

### Examples

Defaults to 10x10 rectangle with 5 points
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/disperserse-points`
Point 1: (0, 0)
Point 2: (10, 10)
Point 3: (10, 0)
Point 4: (0, 10)
Point 5: (5, 5)
```

Custom rectangle with 3x3 points and 9 points
Note: This is wrong because the step is too large
```bash
$ cargo run -- -x 3 -y 3 -n 9 -s 1 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/disperserse-points -x 3 -y 3 -n 9 -s 1`
Point 1: (0, 0)
Point 2: (3, 3)
Point 3: (3, 0)
Point 4: (0, 3)
Point 5: (1, 1)
Point 6: (2, 2)
Point 7: (1, 0)
Point 8: (2, 0)
Point 9: (0, 1)
```

Custom rectangle with 3x3 points and 9 points step 0.5.
Now it works and we get three rows and three columns
evenly distributed.
```bash
$ cargo run -- -x 3 -y 3 -n 9 -s 0.5
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/disperserse-points -x 3 -y 3 -n 9 -s 0.5`
Point 1: (0, 0)
Point 2: (3, 3)
Point 3: (3, 0)
Point 4: (0, 3)
Point 5: (1.5, 1.5)
Point 6: (1.5, 0)
Point 7: (0, 1.5)
Point 8: (3, 1.5)
Point 9: (1.5, 3)
```

But if we make the step small it's not perfect because
probably because of the floating point precision.
```bash
$ cargo run -- -x 3 -y 3 -n 9 -s 0.1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/disperserse-points -x 3 -y 3 -n 9 -s 0.1`
Point 1: (0, 0)
Point 2: (2.9000000000000012, 2.9000000000000012)
Point 3: (2.9000000000000012, 0)
Point 4: (0, 2.9000000000000012)
Point 5: (1.5000000000000002, 1.5000000000000002)
Point 6: (1.4000000000000001, 0.1)
Point 7: (0.1, 1.4000000000000001)
Point 8: (2.9000000000000012, 1.5000000000000002)
Point 9: (1.5000000000000002, 2.9000000000000012)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
