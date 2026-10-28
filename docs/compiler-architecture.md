# PWALang Compiler Architecture

**Version:** Draft 0.1  
**Implementation language:** Rust  
**Status:** Early development

This document explains how the PWALang compiler is structured, why its components exist, and how they interact.

It is intended for contributors, students, and developers interested in compiler engineering and scientific computing.

## 1. Design Objective

PWALang aims to separate the scientific description of a plasma accelerator simulation from the configuration format of a particular simulation engine.

Its proposed pipeline is:

```text
PWALang Source (.pwa)
        |
        v
      Lexer
        |
        v
      Parser
        |
        v
Abstract Syntax Tree (AST)
        |
        v
 Semantic Validation
        |
        v
       PWIR
        |
        v
  Backend Adapter
        |
        v
 Simulation Engine
```

Only the lexer, parser, and AST stages are currently implemented.

## 2. Source Code Organization

The initial Rust project contains:

```text
src/
├── main.rs
├── lexer.rs
├── parser.rs
└── ast.rs
```

### `main.rs`

The executable entry point.

Currently responsible for providing example source text, invoking the lexer and parser, and displaying the resulting AST.

### `lexer.rs`

Implements lexical analysis.

It transforms source characters into tokens representing identifiers, numbers, strings, and punctuation.

### `parser.rs`

Implements syntactic analysis.

It consumes tokens, recognizes grammatical structures, and constructs an AST.

### `ast.rs`

Defines the data structures used to represent parsed programs.

## 3. Lexical Analysis

Consider this source statement:

```pwa
density = 1e17;
```

The lexer produces a sequence conceptually equivalent to:

```rust
[
    Token::Identifier("density".into()),
    Token::Equals,
    Token::Number(1e17),
    Token::Semicolon,
]
```

Tokens are represented using a Rust enum.

```rust
pub enum Token {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    LeftBrace,
    RightBrace,
    Equals,
    Semicolon,
}
```

An `enum` represents a value that can take one of several defined forms.

Here, some token variants carry data, while punctuation variants do not.

## 4. Syntactic Analysis

The parser uses recursive-descent parsing.

Each major grammatical construct is handled by a dedicated function.

Examples include:

```rust
parse_simulation()
parse_statements()
parse_statement()
parse_block()
parse_property()
```

The parser distinguishes properties from blocks by examining the token following an identifier.

An equals sign indicates a property:

```pwa
density = 1e17;
```

A left brace indicates a block:

```pwa
plasma {
    density = 1e17;
}
```

Nested blocks are supported because `parse_block()` calls the statement parser recursively.

## 5. Abstract Syntax Tree

The AST represents source code as structured data rather than raw text.

The current model includes:

```rust
pub struct Simulation {
    pub name: String,
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Property(Property),
    Block(Block),
}

pub struct Block {
    pub name: String,
    pub statements: Vec<Statement>,
}

pub struct Property {
    pub name: String,
    pub value: Value,
}

pub enum Value {
    Number(f64),
    String(String),
}
```

A `Simulation` contains statements.

A statement can be a property or a nested block.

A block can contain further statements, producing a recursive tree structure.

For example:

```pwa
simulation "Test" {
    plasma {
        density = 1e17;
    }
}
```

Corresponds conceptually to:

```text
Simulation
├── name: Test
└── statements
    └── Block
        ├── name: plasma
        └── statements
            └── Property
                ├── name: density
                └── value: Number(1e17)
```

## 6. Rust Concepts Used

| Concept | Purpose |
|---|---|
| `struct` | Groups related data |
| `enum` | Defines alternative value variants |
| `Vec<T>` | Stores a growable sequence of values |
| `String` | Stores owned text |
| `f64` | Represents floating-point values |
| `Option<T>` | Represents an optional value |
| `Result<T, E>` | Represents success or failure |
| `match` | Performs pattern matching |
| `&self` | Borrows an instance immutably |
| `&mut self` | Borrows an instance mutably |
| `?` | Propagates recoverable errors |
| `#[test]` | Declares an automated test |

The parser borrows the token sequence rather than taking ownership of it.

Its position changes as tokens are consumed.

## 7. Error Handling

The parser returns a `Result`.

```rust
Result<Simulation, String>
```

A successful parse returns `Ok(Simulation)`.

An unsuccessful parse returns `Err(String)`.

Current errors include missing delimiters, invalid statement structures, missing values, and unexpected trailing tokens.

Future improvements should include source spans, line and column information, structured diagnostic codes, and more informative error messages.

## 8. Automated Testing

The compiler uses Rust's built-in testing framework.

```bash
cargo test
```

Tests cover successful parsing and rejection of malformed input.

New grammar features should include positive and negative tests.

A successful test suite demonstrates correctness for tested cases, not complete correctness for all possible inputs.

## 9. Planned Semantic Analysis

The semantic analyzer will operate on the AST.

Its responsibilities are expected to include:

- Recognizing supported scientific declarations
- Validating parameter names and types
- Checking physical dimensions and units
- Enforcing numerical and physical constraints
- Rejecting unsupported or ambiguous configurations
- Preparing validated data for intermediate representation generation

For example, a numeric value representing plasma density should not be interpreted without a defined unit convention.

## 10. Planned PWIR

PWIR stands for PWALang Intermediate Representation.

It is intended to represent validated simulation configurations independently of a specific solver's input syntax.

The AST captures how the user wrote the configuration.

PWIR will instead capture a normalized, validated representation suitable for backend processing.

Its final schema has not yet been defined.

## 11. Planned Backend Architecture

The initial target is WarpX, potentially through its Python PICMI interface.

Other possible backends include HiPACE++ and FBPIC.

Backend support will require explicit capability checks.

Simulation engines differ in their numerical models, geometries, physical approximations, and supported features.

PWALang must not silently treat scientifically incompatible configurations as equivalent.

## 12. Development Principles

1. Prioritize scientific correctness over convenience.
2. Keep parsing separate from physical validation.
3. Prefer explicit assumptions and units.
4. Reject unsupported backend features clearly.
5. Maintain automated tests for language behavior.
6. Document implemented features separately from proposals.
7. Preserve reproducibility and traceability of generated configurations.

## 13. Contributor Workflow

From the Rust project directory:

```bash
cargo fmt
cargo test
cargo run
```

Before submitting changes, contributors should verify that the test suite passes and update documentation when language behavior changes.

The compiler architecture is expected to evolve as the project matures.
