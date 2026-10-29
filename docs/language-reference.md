# PWALang Language Reference

**Version:** Draft 0.1  
**Status:** Experimental

This document describes the syntax currently supported by the PWALang compiler.

PWALang is a domain-specific language intended for defining plasma wakefield accelerator simulation configurations.

> The current compiler supports lexical analysis, parsing, and abstract syntax tree generation. Physical units, scientific validation, and solver execution are not yet implemented.

## 1. Basic Program Structure

A PWALang program begins with a `simulation` declaration.

```pwa
simulation "PWFA_Test" {
    density = 1e17;
}
```

The declaration contains a simulation name followed by a body enclosed in braces.

Currently, one top-level simulation declaration is supported per parsed input.

## 2. Tokens and Identifiers

The compiler recognizes:

| Token | Example | Purpose |
|---|---|---|
| Identifier | `plasma` | Names blocks and properties |
| Number | `1e17` | Numeric value |
| String literal | `"PWFA_Test"` | Text value |
| Left brace | `{` | Opens a block |
| Right brace | `}` | Closes a block |
| Equals | `=` | Assigns a property value |
| Semicolon | `;` | Terminates a property |

`simulation` is recognized as a special word in the top-level grammar.

Other names, including `plasma`, `beam`, and `density`, are currently ordinary identifiers rather than reserved keywords.

## 3. Properties

Properties follow this structure:

```pwa
name = value;
```

Examples:

```pwa
density = 1e17;
energy = 1.0;
description = "Baseline plasma";
```

A semicolon is required after every property.

Currently supported values are numeric literals and quoted strings.

## 4. Nested Blocks

Blocks group related statements.

```pwa
simulation "PWFA_Test" {
    plasma {
        density = 1e17;
    }

    beam {
        energy = 1.0;
    }
}
```

Blocks can contain properties and other blocks.

```pwa
simulation "Nested_Test" {
    plasma {
        profile {
            density = 1e17;
        }
    }
}
```

There is no fixed syntactic nesting depth, although practical limits depend on available resources.

Block names are not yet checked against a scientific schema.

## 5. Arithmetic Operators (Experimental)

PWALang Math v0.1 introduces lexical support for basic arithmetic operators. This is the first stage of implementing mathematical expressions for plasma wakefield accelerator simulation configurations.

**Implementation status:** Arithmetic tokenization is implemented and tested. Arithmetic expression parsing, operator precedence, and numerical evaluation are not yet implemented.

### 5.1 Supported Arithmetic Tokens

The lexer currently recognizes the following operators:

| Operator | Name | Example |
|---|---|---|
| `+` | Addition | `10 + 5` |
| `-` | Subtraction | `10 - 5` |
| `*` | Multiplication | `10 * 5` |
| `/` | Division | `10 / 5` |
| `^` | Exponentiation | `2 ^ 3` |
| `(` | Left parenthesis | `(10 + 5)` |
| `)` | Right parenthesis | `(10 + 5)` |

These symbols are recognized as individual tokens. Their mathematical meaning will be implemented in subsequent compiler stages.

### 5.2 Arithmetic Expressions (Planned)

The following syntax illustrates the intended arithmetic expression system:

```pwa
simulation "Arithmetic_Test" {
    addition = 10 + 5;
    subtraction = 10 - 5;
    multiplication = 10 * 5;
    division = 10 / 5;
    exponentiation = 2 ^ 3;
    combined = (10 + 5) * 2;
}
```

**Note:** These examples are not yet accepted by the current parser. They demonstrate the planned syntax rather than implemented functionality.

### 5.3 Operator Precedence (Planned)

Mathematical expressions will follow conventional operator precedence.

| Precedence | Operators | Description |
|---|---|---|
| Highest | `()` | Parenthesized expressions |
| 2 | `^` | Exponentiation |
| 3 | Unary `+`, `-` | Positive and negative signs |
| 4 | `*`, `/` | Multiplication and division |
| Lowest | `+`, `-` | Addition and subtraction |

Exponentiation is planned to be right-associative. For example, `2 ^ 3 ^ 2` will represent `2 ^ (3 ^ 2)`.

The expression `-2 ^ 2` will represent `-(2 ^ 2)`.

### 5.4 Scientific Notation

PWALang already supports floating-point numeric literals using scientific notation.

Examples:

```pwa
simulation "Scientific_Notation" {
    density = 1e17;
    small_value = 1e-3;
    large_value = 2e+5;
}
```

The lexer distinguishes exponent signs from arithmetic operators.

For example:

- `1e-3` is recognized as one numeric token.
- `10 - 3` is recognized as a number, subtraction operator, and another number.

Scientific notation is supported independently of arithmetic expression parsing.

### 5.5 Expression Representation

The compiler defines preliminary abstract syntax tree types for mathematical expressions:

- `Expression::Number` represents a numeric literal.
- `Expression::Binary` represents an operation between two expressions.
- `Expression::Unary` represents an operation applied to one expression.
- `BinaryOperator` defines addition, subtraction, multiplication, division, and exponentiation.
- `UnaryOperator` defines unary plus and minus.

These structures are currently preparatory and are not yet integrated into property parsing or evaluation.

### 5.6 Planned Mathematical Extensions

Future PWALang mathematics development may include:

- Named variables and constants
- Mathematical functions such as `sqrt()`, `sin()`, `cos()`, `exp()`, and `log()`
- Calculus I: derivatives, limits, and integration
- Calculus II: sequences, series, and numerical integration
- Calculus III: vectors, gradients, divergence, curl, and multivariable operations
- Physical unit and dimensional validation
- Mathematical expressions for plasma density profiles and particle-beam configurations

These capabilities remain proposals. Their exact syntax, numerical methods, and integration with simulation backends have not yet been specified.

## 6. Formal Grammar

The following EBNF summarizes the implemented parser structure:

```ebnf
program     = simulation ;
simulation  = "simulation", string, "{", { statement }, "}" ;
statement   = property | block ;
property    = identifier, "=", value, ";" ;
block       = identifier, "{", { statement }, "}" ;
value       = number | string ;
```

Lexical details such as identifier characters, numeric formats, and string escaping are defined by the lexer implementation and will be specified more precisely as the language evolves.

## 7. Syntax Errors

The parser rejects malformed structures.

Missing semicolon:

```pwa
simulation "Test" {
    density = 1e17
}
```

Missing closing brace:

```pwa
simulation "Test" {
    plasma {
        density = 1e17;
}
```

Unexpected tokens after the simulation:

```pwa
simulation "Test" {}
extra
```

## 8. Syntax Versus Scientific Validity

The following example is syntactically acceptable:

```pwa
simulation "Test" {
    banana {
        temperature = "hello";
    }
}
```

However, it is not a defined plasma simulation configuration.

This distinction exists because the current parser validates grammar, not scientific meaning.

Future semantic validation will check supported blocks, property names, value types, physical dimensions, parameter constraints, and backend compatibility.

## 9. Planned Language Features

The following features are proposals, not implemented syntax:

- Physical quantities with explicit units
- Validated plasma and particle-beam schemas
- Simulation domains and numerical settings
- Diagnostics and output configuration
- Solver capability validation
- Solver-independent intermediate representation
- External `.pwa` file loading through a command-line interface

Language changes will be documented here as implementations and tests are added.

## 10. Running the Current Compiler

From the nested Rust project directory:

```bash
cargo run
cargo test
```

The executable currently parses a source example embedded in `src/main.rs`.

See [Compiler Architecture](compiler-architecture.md) for implementation details.
