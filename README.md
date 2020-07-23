# Unit: Universal Test Generator

[![Build status](https://github.com/rogeriochaves/unit/workflows/ci/badge.svg)](https://github.com/rogeriochaves/unit/actions)

The most boring part of writing tests is having to setup the test runner for a new project, or having to learn how to write tests on a new language. People end up postponing it and end up never testing their code.

With unit, a simple `unit path/to/file` command will set up your test runner for many languages and test frameworks, and create a test file ready to run.

## Instalation

For Mac, you can install using [brew](https://brew.sh/)

```bash
brew tap rogeriochaves/tap
brew install unit
```

For Linux and Windows, download the binary on [releases page](https://github.com/rogeriochaves/unit/releases)

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
| Clojure    | std              |
| Elm        | elm-test         |
| Java       | junit            |
| JavaScript | jest             |
| Perl       | std, test-spec   |
| Python     | std              |
| Ruby       | std, rspec       |
| Rust       | std, integration |

The "std" test runner stands for the default test runner that comes with the language and requires no dependencies to be installed.

## Contributing

If you'd like to support a new language or test framework, first create a new example project for it under `tests/examples/mylanguage/mytestrunner`. Then create or edit the test file for it on `tests/mylanguage_test.rs`.

You can follow the examples of other tests like `tests/javascript_test.rs` or `tests/python_test.rs`, and then run it with:

`cargo test mylanguage`

You will need to have [rust installed](https://www.rust-lang.org/tools/install) to run cargo.

Then you can implement the generator, on `src/generators/mylanguage.rs`. Again, follow the examples of other modules, then add it to `src/generators/mod.rs` and `src/lib.rs` on the `available_generators` function.
