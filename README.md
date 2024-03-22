# AST-Analyzer

A simple rust based tool to document methods in a given go file.

## Requirements

 - rust
 - cargo
 - Ollama 
 - Codellama installed via ollama

## Building 

To build this project, run the following command inside the project directory.

```bash
cargo build
```

## Running this

First run ollama and have it serve codellama. 

```bash
ollama serve codellama
```

Then run the following command to generate the documentation.

```bash
target/debug/docthis <path to go file>
```

## Examples

The examples folder gives some weird examples that can be tested with this tool.

These are deliberately poorly/confusing written go programs that we want to see if the 
tool can document.

mystery-1 : Calculates the finbonacci series of the nth number.
mystery-2 : Determines if a given number is a prime number.
mystery-3 : Reverses a string
mystery-4 : Sums up the digits inside a number.
