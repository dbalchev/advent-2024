# About

My (@dbalchev) solutions to the problems in https://adventofcode.com/2024.

# How To Run

## Setup

1. Install [Rust](https://www.rust-lang.org/). I've used version 1.82.0.
2. `cargo run -- --day 0` should be able to finish successfully. (Note that it will download and build dependencies, so it will take some time).
3. (Optional) if you want to leverage the downloader, create a file `downloader.args` with the following format
    ```
    --url-prefix=https://adventofcode.com/2024/
    --user-session=${YOUR_AOC_USER_SESSION}
    ```

## Running a specific day

### Manual inputs

To run a specific day with a manually passed file, use

```
RUST_BACKTRACE=1 cargo run --release  -- --day ${DAY_NUMBER} --input-file ${PATH_TO_INPUT}
```

`RUST_BACKTRACE=1` Is not needed, but it helps when the solution panics.

### With automatic input passing

To run a specific day on the sample input after downloading it, use

```
RUST_BACKTRACE=1 cargo run -- --day ${DAY_NUMBER} --sample
```

To change which &lt;pre&gt; is used for the sample, `impl` the get `preferred_sample_input` method for the specific day's `Solution`.

To run on the real input, use

```
RUST_BACKTRACE=1 cargo run -- --day ${DAY_NUMBER} --real
```

## (Optional) Downloading inputs

```
cargo run --bin aoc_data_downloader -- --day ${DAY_NUMBER} @downloader.args
```

# Adding a new day

To add a day, you need to do a few things:

1. in `aoc/src/solutions/mod.rs` inside the `register_days!` macro add `day_##`.
2. create a file `day_##.rs` in `aoc/src/solutions/`
3. (optional) use the `new_day` VS Code code snippet to populate with a simple template or copy `day_00.rs` in the new file

# Project Structure

The project has 3 crates:

-   `aoc_data_downloader` - used to download the input data
-   `aoc` - contains the actual solutions for the current year and a minimal structure:
    -   `aoc/src/main.rs` - the entry point for the executable
    -   `aoc/src/solutions/mod.rs` - a registry for which days are included
    -   `aoc/src/solutions/day_##.rs` - the solution for a particular day
-   `aoc_utils` - contains all the infra code.

## Adapting to a new year

Copy everything but `aoc/solutions/day_##.rs`. You should be good to go. `day_00.rs` can be used as a template so you can leave it.

## Notable utils

-   `register_days!` - designed to be the single place where you have to register a new day. Declares each new module (one per file) and creates the `make_day_solutions` function, that returns a registry of the days, that can be ran.
    -   NOTE: it expects the days module to have a type `pub Solution` that implements the `DaySolution` trait.
-   `formatted_struct!` - designed to solve your parsing needs for (almost) every day. When you wrap your structs/enums in it will implement `Parsable`. It handles the following cases:
    -   Sequential structs:
        ```rust
        formatted_struct! {
            #[derive(Debug)]
            pub struct ButtonMoves {
                "X\\+",
                x_delta: i64,
                ", Y\\+",
                y_delta:i64,
                "\n",
            }
        }
        ```
        A normal struct definition, where the fields are seoarated by string literals. You can have optional string literals before the first and after the last field. If you have a literal before the first, it skips all the chars that match it. then it splits the input according to each regex between fields and it tries to parse each field from it's split. If one fails the full parsing fails.
    -   Alternative enums:
        ```rust
        formatted_struct! {
            #[derive(Debug)]
            pub enum Instruction {
                Set {
                    name:String,
                    "=",
                    value: i32,
                },
                Dash {
                    name: String,
                    "-",
                },
            }
        }
        ```
        Every enum variant must be a struct variant. They have the same logic as the sequential structs. It attempts to parse each variant in order, returning the first one that succeeds.
    -   Separated by:
        ```rust
        formatted_struct! {
            #[derive(Debug)]
            pub struct InputFormat {
                #[separated_by="\n\n"]
                schematics: Vec<Schematic>,
            }
        }
        ```
        Each field could be have the `separated_by` attribute and be a `Vec`. Then when it's parsed it's split by the passed regex and each item attempts to parse it. If any fails, the full parsing fails.
