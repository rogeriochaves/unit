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

| Language | Test Runners |
| -------- | ------------ |
| Ruby     | std, rspec   |
| Rust     | std          |
