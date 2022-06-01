# ulam ftw 
takes a Coord( x, y )

```rust
    let mut ulam_points = HashMap::new();

    for x in -201..201 {
        if x % 10 == 0 {
            println!("{x}");
        }
        for y in -201..201 {
            let c = Coord::new(x, y);
            let a = ulam::get_ulam_deets(&c);

            ulam_points.insert(
                a.value,
                c
            );
        }
    }


    let c = Coord::new(20000, 20000);
    let a = ulam::get_ulam_deets(&c);

    println!("{}", a.value);
    // println!("{:?}", ulam_points[&55]);



    ulam::ulamspiral_img::generate(1_000, 1_000)
        .unwrap()
        .save("./result.png");
```