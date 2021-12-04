# Advent of Code 2021

Check the [Advent of Code site](https://adventofcode.com/) for details

I don't know how many days I will be able to participate, but let's give it a shot.

Doing it in Rust. Run each day as `cargo run -- <<dayN>> <<params>>` where N is from 1 to 25.
For part 2 in a day it will typically be `dayN-2`.
If I didn't get to do one of the days, it will complain. Just `cargo run` to see the days
available (they show in the help as SUBCOMMANDS). E.g.

```
cargo run -- day1 data/day1.txt
```

Note: the event creators requested that data files for the problems are not posted in public.
See [this post](https://www.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/) for details.
So you need to get your own copy of these files from the AoC site if you want to run my solutions.

## Benchmarking

I have integrated the [Criterion](https://crates.io/crates/criterion) benchmarking crate to check it out.
It's not meant to be very useful to the read (or myself) except as a learning exercise in how to set up
this stuff. For example, it taught me how to split the originally binary crate into lib and bin crates side-by-side.

To run benchmarks, you should use `cargo bench -- <<options>>`. You can use option `--help` for details, `--list` to show which
benchmarks are available, or a portion of a benchmark name to run just the ones matching what you wrote (recommended).
For example, `cargo bench -- day4_run` would benchmark the computation part of the day4 solution, while
`cargo bench -- read` would run the reading part of all days. If you run the `_full` benchmarks, I recommend redirecting
the standard output to a file, to prevent console output performance from impacting the measurements.

Open the file `target/criterion/index.html` to see the results in your browser.

## License

The MIT License (MIT)
Copyright © 2021 Javier Arévalo

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
