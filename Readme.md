# Student Assignment

Problem: You have n students and offer k private lectures at different times.
Each student has to visit one lecture. So they can register themselves, which of the k private lectures they have time to attend.

This Program prints the distribution of the n students to the k lectures according to the student preferences.

It only works, when there is a solution.
When there is no solution, an optimizer an be used instead of an solver (the energy function is a fairness problem, please consult your local ethicist).

The solver uses [Z3](https://github.com/Z3Prover/z3) and the [Rust binding of Z3](https://docs.rs/z3/0.6.0/z3/index.html).

License is the same as the Z3 License: MIT

## How to Use

Give the program a JSON file.
Please have a look at the [example](./example.json) file.

## License

[MIT License](./LICENSE) © Matthias Möller. Made with ♥ in Germany.
