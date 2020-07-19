# Unit: Universal Test Generator

The most boring part of writing tests is having to setup the test runner for a new project, or having to learn how to write tests on a new language. People end up postponing it and end up never testing their code.

With unit, a simple `unit path/to/file` command will set up your test runner for many languages and test frameworks, and create a test file ready to run.

## Instalation

Download the binary on packages tab

## Usage example

```
$ unit src/user.rb
Test file created! Run it with `ruby test/test_user.rb`
```

You can also use other test runners

```
$ unit src/user.rb --rspec
```

## Suported Languages

| Language   | Test Runners     |
| ---------- | ---------------- |
| Elm        | elm-test         |
| JavaScript | jest             |
| Perl       | std, test-spec   |
| Python     | std              |
| Ruby       | std, rspec       |
| Rust       | std, integration |

The "std" test runner stands for the default test runner that comes with the language and requires no dependencies to be installed. It is the default selection if no option is specified.

## Contributing

If you'd like to support a new language or test framework, first create a new example project for it under `tests/examples/mylanguage/mytestrunner`. Then create or edit the test file for it on `tests/mylanguage_test.rs`.

You can follow the examples of other tests like `tests/javascript_test.rs` or `tests/python_test.rs`, and then run it with:

`cargo test mylanguage`

You will need to have [rust installed](https://www.rust-lang.org/tools/install) to run cargo.

Then you can implement the generator, on `src/generators/mylanguage.rs`. Again, follow the examples of other modules, then add it to `src/generators/mod.rs` and `src/lib.rs` on the `available_generators` function.
