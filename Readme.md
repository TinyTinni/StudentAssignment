# Student Assignment
[![Rust](https://github.com/TinyTinni/student_assignment/actions/workflows/rust.yml/badge.svg)](https://github.com/TinyTinni/student_assignment/actions/workflows/rust.yml)

Problem: You have n students and offer k private lectures at different times.
Each student has to visit one lecture. So they can register themselves, which of the k private lectures they have time to attend.

This Program prints the distribution of the n students to the k lectures according to the student preferences.

It only works, when there is a solution.
When there is no solution, an optimizer an be used instead of an solver (the energy function is a fairness problem, please consult your local ethicist).

The solver uses [Z3](https://github.com/Z3Prover/z3) and the [Rust binding of Z3](https://docs.rs/z3/0.6.0/z3/index.html).

## Example Problem

Imagine you have 3 Students $s_0, s_1$ and $s_2$ and you want to give them private lectures on time $t_0, t_1$ and $t_2$.

Since you are a good teacher, you give them the ability to express wishes, which timeslot they want to attend.
In this example, every student has to attend to exactly 1 timeslot and every timeslot has a capacity of 1 student.

- $s_0: [t_1]$
- $s_1: [t_0, t_1, t_2]$
- $s_2: [t_0, t_1]$

This program finds a solution for this problem and assigns the students to their timeslots. Solution:

- $s_0 -> t_1$
- $s_1 -> t_2$
- $s_2 -> t_0$

## How to Use

Give the program a JSON file.
Please have a look at the [example](./example.json) file.

You can change the student capacity per timeslot in the json.
The amount of timeslots, each student has to visit is the program option `--visits`.

```
USAGE:
    student_assignment-rs.exe <input> --visits <visits>
```

## Build

### On Windows

You either have to install Z3 and add the includes and library path into the global path,
or build Z3 locally and link it statically.

The Rust package supports this, 
but due to a missing feature in cargo [(issue 7914)](https://github.com/rust-lang/cargo/issues/7914),
you have to enable the option in the [Cargo.toml](./Cargo.toml) and disable the Unix specification of Z3 inclusion.


### On Linux

Install Z3 package and run `cargo build`.

## License

[MIT License](./LICENSE) © Matthias Möller. Made with ♥ in Germany.
