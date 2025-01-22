# Rust CLI Step-by-Step Calculator
A command-line calculator written in Rust that evaluates mathematical expressions step by step. This tool is designed to parse, validate, and compute expressions while displaying every intermediate calculation stage, making it easier to understand how results are derived.

# Features
- Expression Parsing:
  - Supports basic arithmetic: +, -, *, /.
  - Handles parentheses and respects operator precedence.
- Step-by-Step Evaluation:
  - Breaks down the expression into smaller parts.
  - Shows each calculation step before providing the final result.
- Error Handling:
  - Validates input for syntax issues like unmatched parentheses or invalid characters.
  - Provides user-friendly error messages.
- Interactive CLI Mode:
  - Accepts single expressions directly from the terminal.

# Technology Stack
- Language: Rust
- Dependencies:
  - regex for input validation and parsing.
  - custom parsing logic for evaluating expressions.
