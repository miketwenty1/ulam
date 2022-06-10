# ulam crate
### ulam spiral lib for rust.

- Takes a Coord( x, y ) and returns a value in the spiral.
- Takes a value and returns a Coord(x,y).
- Can also be used to produce pictures of ulam spirals with primes colored.
## Important Notes
- this current starts with 0 in the middle and starts to the right.
```rust
Coord {
    x: 1,
    y: 0,
}
```
This Coord is equal the value of 1 if passed to the 

## Example usage:
```rust
    let mut ulam_points = HashMap::new();

    for x in -201..201 {
        if x % 10 == 0 {
            println!("{x}");
        }
        for y in -201..201 {
            let c = Coord::new(x, y);
            let a = ulam::get_ulam_point(&c);

            ulam_points.insert(
                a.value,
                c
            );
        }
    }


    let c = Coord::new(20000, 20000);
    let a = ulam::get_ulam_point(&c);

    println!("{}", a.value);
    println!("{:?}", ulam_points[&55]);



    ulam::ulamspiral_img::generate(1_000, 1_000)
        .unwrap()
        .save("./result.png");
```