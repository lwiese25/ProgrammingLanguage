# Asa Language Interpreter
This project involves developing a complete interpreter for the Asa programming language, with a focus on handling conditional expressions and extending the language's capabilities. The project includes the design and implementation of a lexer, parser, and interpreter, as well as the creation of comprehensive tests to ensure correctness and reliability.

## Project Overview
The Asa language interpreter is designed to parse and execute programs written in a simple, expressive language. An example program in Asa might look like this:

### Asa

let x = 1 + 1;
let y = 5 - 2;
let z = x + y;

The interpreter supports various data types, including strings, numbers, and booleans, and is capable of executing arithmetic operations, variable bindings, and, importantly, conditional expressions.

## Key Components
### 1. Lexer and Grammar
The lexer has been updated to recognize the syntax for conditional operators, such as:

Greater-than (>)
Less-than (<)
Greater-than or Equal-to (>=)
Less-than or Equal-to (<=)
Equal-to (==)
Not Equal-to (!=)
These operators are critical for implementing conditional logic within the language. The grammar has been defined to incorporate these operators and ensure they are parsed correctly.

### 2. Parser
The parser has been modified to:

Support the new grammar rules, including the conditional operators.
Add new parser node variants to handle these operations.
Ensure consistency between the parser and the updated grammar.
The parser now correctly generates abstract syntax trees (ASTs) that reflect the structure of programs written in Asa, taking into account operator precedence and type compatibility.

### 3. Interpreter
The interpreter has been fully implemented, including:

Extending the existing Value enum to handle the results of conditional expressions.
Updating the Interpreter struct to maintain the state, including variable bindings and functions.
Implementing evaluation rules within the exec() function to correctly handle conditional operators.
The interpreter ensures that invalid expressions (e.g., comparing a number to a boolean) produce clear and informative error messages.

### 4. Testing
To ensure the correctness of the interpreter, a suite of at least 10 new tests has been written. These tests cover:

## Basic functionality of conditional expressions.

Edge cases, such as invalid type comparisons.

Operator precedence and proper execution order.

Existing tests have been updated to account for the new features, ensuring that the entire interpreter behaves as expected.


## Deliverables
The fully implemented interpreter located in /src/interpreter.rs, along with the updated lexer and parser.

A comprehensive set of tests in test/tests.rs that improve test coverage and ensure robust handling of all features.

A recording link that provides a detailed explanation and demonstration of the interpreter.

The project is organized and well-documented, with clear comments and explanations throughout the code. Assumptions made during implementation are noted, and any limitations are discussed.

This interpreter serves as a solid foundation for further development and exploration within the Asa programming language, providing a robust tool for executing complex programs with conditional logic.
