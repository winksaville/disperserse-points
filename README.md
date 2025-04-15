# Disperserse points

I'm working with ChatGPT 4.0 to disperse points
in a 2D space. The goal is to take a set of points and disperse them
in a way that they are evenly distributed across the space with maximal
distance between them.

See this [conversation](https://chatgpt.com/share/67fdd976-d4b4-800c-8b51-ba50899e95a3)

This is simplified with the space defined a s rectangle and
the points must be inside the rectangle. Later the pionts will
be a polygon initially as square although more complex polygons
could be used later.

Currently this hardcode a for 5 points in a 10 x 10 square and solves
it correct, AFAICT, with a point in each corner and one in the middle.

## Usage

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/disperserse-points`
Point 1: (0, 0)
Point 2: (10, 10)
Point 3: (10, 0)
Point 4: (0, 10)
Point 5: (5, 5)
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
