# Euclidian rythm generators

Rust implementation of [“The Euclidean Algorithm Generates Traditional Musical
Rhythms”](http://cgm.cs.mcgill.ca/%7Egodfried/publications/banff.pdf).

Effectively a port of [a python
implementation](https://github.com/brianhouse/bjorklund), which is a port of
[the original paper's
implementation](https://ics-web.sns.ornl.gov/timing/Rep-Rate%20Tech%20Note.pdf).

# Example

Generating the bell pattern of [Adowa:
Mpre](https://www.youtube.com/watch?v=D8Fr1Bw-znM), traditional Ashanti music
from Ghana.

```rust
let mut pattern = [0 as u8; 12];
let pulses = 7;
euclidian_rythm(&mut pattern, pulses).unwrap();
println!("{:?}", pattern);
// [1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0]
```

# License

Either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
