# PWALang Compiler Architecture

**Version:** Draft 0.1  
**Implementation language:** Rust  
**Status:** Early development

This document describes the architecture of the PWALang compiler, its implemented components, and its planned development.

PWALang is a domain-specific language designed to express plasma wakefield accelerator simulation configurations independently of a particular simulation engine.

## 1. Design Objective

PWALang aims to separate the scientific description of a plasma accelerator simulation from the configuration format of a particular simulation engine.

The proposed scientific compilation pipeline is:

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
 Abstract Syntax Tree
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

Currently implemented components include the lexer, parser, AST definitions, and a minimal console-output runtime.

The current runtime follows a separate development path:

```text
Embedded PWALang Source
        |
        v
      Lexer
        |
        v
      Parser
        |
        v
       AST
        |
        v
 Console Output Runtime
        |
        v
 Standard Output
```

The console-output runtime is not a plasma simulation solver and does not replace the planned scientific compilation pipeline.

Semantic validation, PWIR generation, backend adapters, and solver execution remain unimplemented.

## 2. Source Code Organization

The current Rust project contains:

```text
src/
├── main.rs
├── lexer.rs
├── parser.rs
├── ast.rs
└── runtime.rs
```

### `main.rs`

The executable entry point.

Currently responsible for:

- Providing an embedded PWALang source example
- Invoking lexical analysis
- Invoking syntactic analysis
- Reporting lexer and parser errors
- Executing supported console-output statements

The executable does not yet accept external `.pwa` files through command-line arguments.

### `lexer.rs`

Implements lexical analysis.

It transforms source characters into tokens representing identifiers, numbers, strings, punctuation, and arithmetic operators.

### `parser.rs`

Implements recursive-descent syntactic analysis.

It consumes tokens, recognizes grammatical structures, and constructs an AST.

Currently supported statements include properties, nested blocks, and console output.

### `ast.rs`

Defines the Rust data structures representing parsed PWALang programs.

It also contains preliminary expression types for future mathematical operations.

### `runtime.rs`

Implements a minimal console-output runtime.

It traverses the parsed AST, collects supported output values in source order, and prints them to standard output.

It does not evaluate arithmetic expressions, resolve variable references, or execute plasma simulations.

## 3. Lexical Analysis

Consider:

```pwa
density = 1e17;
```

The lexer produces tokens conceptually equivalent to:

```rust
[
    Token::Identifier("density".into()),
    Token::Equals,
    Token::Number(1e17),
    Token::Semicolon,
]
```

Tokens are represented using a Rust enum.

The implemented token variants are:

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    LeftBrace,
    RightBrace,
    Equals,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
}
```

The lexer recognizes scientific notation, including examples such as `1e17`, `1e-3`, and `2e+5`.

Arithmetic operators are tokenized but are not yet interpreted as complete mathematical expressions.

For example:

```pwa
10 + 20 * 3
```

Can be tokenized into numbers and operators, but cannot yet be parsed as a property value or evaluated.

## 4. Syntactic Analysis

PWALang uses recursive-descent parsing.

The parser includes functions such as:

```rust
parse_simulation()
parse_statements()
parse_statement()
parse_block()
parse_property()
parse_value()
parse_output()
```

The parser distinguishes statement types by examining the token following an identifier.

### Property assignment

```pwa
density = 1e17;
```

An equals sign indicates a property.

### Nested block

```pwa
plasma {
    density = 1e17;
}
```

A left brace indicates a nested block.

### Console output

```pwa
output("Hello, PWALang!");
```

An identifier named `output` followed by a left parenthesis indicates a built-in console-output statement.

Unknown function-style statements are rejected.

The parser requires exactly one literal argument for `output()`.

Nested blocks are supported through recursive statement parsing.

## 5. Abstract Syntax Tree

The AST represents parsed source code as structured data.

The current primary structures are:

```rust
#[derive(Debug, PartialEq)]
pub struct Simulation {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Property(Property),
    Block(Block),
    Output(Value),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}
```

A simulation contains statements.

A statement may be a property, a nested block, or a console-output statement.

For example:

```pwa
simulation "Test" {
    plasma {
        density = 1e17;
        output("Plasma loaded");
    }
}
```

Produces an AST conceptually equivalent to:

```text
Simulation
├── name: Test
└── statements
    └── Block
        ├── name: plasma
        └── statements
            ├── Property
            │   ├── name: density
            │   └── value: Number(1e17)
            └── Output
                └── String("Plasma loaded")
```

The AST preserves the hierarchical structure of the source program.

## 6. Preliminary Mathematical Expression AST

PWALang Math v0.1 introduces arithmetic tokens and preliminary AST definitions.

The following types are currently defined:

```rust
pub enum Expression {
    Number(f64),

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

pub enum UnaryOperator {
    Plus,
    Minus,
}
```

These structures are intended to represent mathematical expressions as recursive trees.

For example, the future expression:

```pwa
10 + 20 * 3
```

Could be represented conceptually as:

```text
Binary(Add)
├── Number(10)
└── Binary(Multiply)
    ├── Number(20)
    └── Number(3)
```

This representation preserves operator precedence.

`Box<Expression>` allows recursive expression structures to be stored using indirection.

**Implementation status:** The expression types exist, but the current parser does not yet construct them. Mathematical evaluation is not implemented.

## 7. Console Output Runtime

The current runtime is implemented in `runtime.rs`.

Its primary functions are:

```rust
execute()
collect_output()
collect_statements()
format_value()
```

### Execution process

The runtime receives a parsed `Simulation` and traverses its statements.

When it encounters an output statement, it formats the literal value.

For nested blocks, it recursively traverses their statements.

Property declarations are ignored by the current runtime.

### Example

```pwa
simulation "Output_Test" {
    output("Hello, PWALang!");
    output(42);

    plasma {
        output("Nested output");
    }
}
```

Expected output:

```text
Hello, PWALang!
42
Nested output
```

### Testable output collection

The runtime separates output collection from terminal printing.

`collect_output()` returns a `Vec<String>` that can be checked through automated tests.

`execute()` prints the collected values to standard output.

This design makes console-output behavior easier to verify without capturing terminal output during tests.

### Current limitations

- Only numeric and string literals are supported.
- Variable references are not supported.
- Arithmetic expressions cannot be evaluated.
- No mutable runtime state or symbol table exists.
- Output is not connected to simulation diagnostics or scientific data export.

The runtime is intended as an early development utility, not a complete execution environment.

## 8. Rust Concepts Used

| Concept | Purpose |
|---|---|
| `struct` | Groups related data |
| `enum` | Defines alternative value variants |
| `Vec<T>` | Stores growable sequences |
| `String` | Stores owned text |
| `f64` | Represents floating-point values |
| `Option<T>` | Represents optional values |
| `Result<T, E>` | Represents success or failure |
| `match` | Performs pattern matching |
| `&self` | Borrows an instance immutably |
| `&mut self` | Borrows an instance mutably |
| `Box<T>` | Provides indirection for recursive structures |
| `?` | Propagates recoverable errors |
| `#[test]` | Declares an automated test |

The parser borrows the token sequence rather than taking ownership of it.

Its position changes as tokens are consumed.

The runtime borrows the parsed simulation and traverses its statements without modifying the AST.

## 9. Error Handling

The parser returns:

```rust
Result<Simulation, String>
```

A successful parse returns `Ok(Simulation)`.

An unsuccessful parse returns `Err(String)`.

Current errors include:

- Missing delimiters
- Invalid statement structures
- Missing property values
- Invalid output arguments
- Unknown function-style statements
- Unexpected trailing tokens

The lexer also rejects unsupported characters and malformed literals.

Future improvements should include source spans, line and column information, structured diagnostic codes, and clearer error messages.

## 10. Automated Testing

PWALang uses Rust's built-in testing framework.

```bash
cargo test
```

At the time of this documentation update, the reported test suite contains **19 passing tests**.

Tests cover:

- Lexical analysis
- Arithmetic token recognition
- Simulation declarations
- Properties and nested blocks
- Console-output parsing
- Rejection of malformed syntax
- Runtime output ordering

New language features should include both positive and negative tests.

Passing tests demonstrate correctness for the tested cases, not complete correctness for all possible inputs.

## 11. Planned Semantic Analysis

The semantic analyzer will operate on the AST.

Its responsibilities are expected to include:

- Recognizing supported scientific declarations
- Validating parameter names and types
- Checking physical dimensions and units
- Enforcing numerical and physical constraints
- Rejecting unsupported or ambiguous configurations
- Preparing validated data for intermediate representation generation

For example, a numeric value representing plasma density should not be interpreted without a defined unit convention.

## 12. Planned PWIR

PWIR stands for PWALang Intermediate Representation.

It is intended to represent validated simulation configurations independently of a specific solver's input syntax.

The AST captures how the user wrote the configuration.

PWIR will instead capture a normalized, validated representation suitable for backend processing.

Its final schema has not yet been defined.

## 13. Planned Backend Architecture

The initial target is WarpX, potentially through its Python PICMI interface.

Other possible backends include HiPACE++ and FBPIC.

Backend support will require explicit capability checks.

Simulation engines differ in their numerical models, geometries, physical approximations, and supported features.

PWALang must not silently treat scientifically incompatible configurations as equivalent.

## 14. Development Principles

1. Prioritize scientific correctness over convenience.
2. Keep parsing separate from physical validation.
3. Prefer explicit assumptions and units.
4. Reject unsupported backend features clearly.
5. Maintain automated tests for language behavior.
6. Document implemented features separately from proposals.
7. Preserve reproducibility and traceability of generated configurations.

## 15. Contributor Workflow

From the nested Rust project directory:

```bash
cargo fmt
cargo test
cargo run
```

Before submitting changes, contributors should verify that the test suite passes and update documentation when language behavior changes.

The compiler architecture will evolve as PWALang matures.