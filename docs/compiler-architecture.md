PWALang Compiler Architecture
Version: Draft 0.1  

Implementation language: Rust  

Status: Early development

This document describes the architecture of the PWALang compiler, its implemented components, and its planned development.

PWALang is a domain-specific language designed to express plasma wakefield accelerator simulation configurations independently of a particular simulation engine.
1. Design Objective
PWALang aims to separate the scientific description of a plasma accelerator simulation from the configuration format of a particular simulation engine.

The proposed scientific compilation pipeline is:

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

Currently implemented components include the lexer, parser, AST definitions, and a minimal console-output runtime.

The current runtime follows a separate development path:

External PWALang Source (.pwa)
        |
        v
 Command-Line File Loading

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

The console-output runtime is not a plasma simulation solver and does not replace the planned scientific compilation pipeline.

Semantic validation, PWIR generation, backend adapters, and solver execution remain unimplemented.
2. Source Code Organization
The current Rust project contains the following source modules and an example program:

src/

├── main.rs

├── lexer.rs

├── parser.rs

├── ast.rs

└── runtime.rs

examples/
└── hello.pwa

main.rs
The executable entry point.

Currently responsible for:
- Reading a .pwa file path from command-line arguments
- Validating the file extension and loading source text
- Invoking lexical analysis
- Invoking syntactic analysis
- Reporting lexer and parser errors
- Executing supported console-output statements
The executable accepts exactly one command-line argument identifying an external .pwa source file. It verifies the .pwa suffix, reads the file as UTF-8 text, and reports file-reading failures. It currently does not support multiple input files or additional command-line options.
lexer.rs
Implements lexical analysis.

It transforms source characters into tokens representing identifiers, numbers, strings, punctuation, and arithmetic operators.
parser.rs
Implements recursive-descent syntactic analysis.

It consumes tokens, recognizes grammatical structures, and constructs an AST.

Currently supported statements include properties, nested blocks, and console output.
ast.rs
Defines the Rust data structures representing parsed PWALang programs.

It also contains preliminary expression types for future mathematical operations.
runtime.rs
Implements a minimal console-output runtime.

It traverses the parsed AST, collects supported output values in source order, and prints them to standard output.

It does not evaluate arithmetic expressions, resolve variable references, or execute plasma simulations.
3. External Source Files and Command-Line Interface
The executable reads a .pwa file from disk instead of using an embedded source string. The current command-line entry point is implemented using Rust standard-library facilities: std::env for arguments, std::fs for file reading, and std::process for error exit codes.

From the nested Rust project directory (PWALang/PWALang), run:
cargo run -- examples/hello.pwa
The -- separator passes the file path to the PWALang executable rather than to Cargo. Alternatively, after building the binary:
cargo build
./target/debug/pwalang examples/hello.pwa
A minimal example file is:
simulation "My_First_Program" {
    output("Hello from a real .pwa file!");
    output(42);

    plasma {
        density = 1e17;
        output("Plasma configured!");
    }
}
Expected program output:
Hello from a real .pwa file!
42
Plasma configured!
The program reports an error and exits unsuccessfully when the required file argument is missing, the filename does not end in .pwa, the file cannot be read, or lexical or parsing errors occur. The .pwa suffix check does not by itself guarantee syntactic or scientific validity; parsing provides syntactic validation, while scientific validation is still planned.
VS Code development workflow
VS Code can open .pwa files as text. An optional workspace setting may associate *.pwa with JavaScript highlighting as a temporary visual convenience; this does not provide a PWALang grammar, language server, completion, or diagnostics. A dedicated extension is a future enhancement.

An optional VS Code task can invoke Cargo with the active file path, allowing users to run the currently open .pwa file. This is editor configuration rather than a feature of the compiler itself, and should not be assumed to exist in every checkout.
4. Lexical Analysis
Consider:

density = 1e17;

The lexer produces tokens conceptually equivalent to:

[

    Token::Identifier("density".into()),

    Token::Equals,

    Token::Number(1e17),

    Token::Semicolon,

]

Tokens are represented using a Rust enum.

The implemented token variants are:

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

The lexer recognizes scientific notation, including examples such as 1e17, 1e-3, and 2e+5.

Arithmetic operators are tokenized but are not yet interpreted as complete mathematical expressions.

For example:

10 + 20 * 3

Can be tokenized into numbers and operators, but cannot yet be parsed as a property value or evaluated.
5. Syntactic Analysis
PWALang uses recursive-descent parsing.

The parser includes functions such as:

parse_simulation()

parse_statements()

parse_statement()

parse_block()

parse_property()

parse_value()

parse_output()

The parser distinguishes statement types by examining the token following an identifier.
Property assignment

density = 1e17;

An equals sign indicates a property.
Nested block

plasma {

    density = 1e17;

}

A left brace indicates a nested block.
Console output

output("Hello, PWALang!");

An identifier named output followed by a left parenthesis indicates a built-in console-output statement.

Unknown function-style statements are rejected.

The parser requires exactly one literal argument for output().

Nested blocks are supported through recursive statement parsing.
6. Abstract Syntax Tree
The AST represents parsed source code as structured data.

The current primary structures are:

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

A simulation contains statements.

A statement may be a property, a nested block, or a console-output statement.

For example:

simulation "Test" {

    plasma {

        density = 1e17;

        output("Plasma loaded");

    }

}

Produces an AST conceptually equivalent to:

Simulation

├── name: Test

└── statements

    └── Block

        ├── name: plasma

        └── statements

            ├── Property

            │   ├── name: density

            │   └── value: Number(1e17)

            └── Output

                └── String("Plasma loaded")

The AST preserves the hierarchical structure of the source program.
7. Preliminary Mathematical Expression AST
PWALang Math v0.1 introduces arithmetic tokens and preliminary AST definitions.

The following types are currently defined:

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

These structures are intended to represent mathematical expressions as recursive trees.

For example, the future expression:

10 + 20 * 3

Could be represented conceptually as:

Binary(Add)

├── Number(10)

└── Binary(Multiply)

    ├── Number(20)

    └── Number(3)

This representation preserves operator precedence.

Box<Expression> allows recursive expression structures to be stored using indirection.

Implementation status: The expression types exist, but the current parser does not yet construct them. Mathematical evaluation is not implemented.
8. Console Output Runtime
The current runtime is implemented in runtime.rs.

Its primary functions are:

execute()

collect_output()

collect_statements()

format_value()

Execution process
The runtime receives a parsed Simulation and traverses its statements.

When it encounters an output statement, it formats the literal value.

For nested blocks, it recursively traverses their statements.

Property declarations are ignored by the current runtime.
Example

simulation "Output_Test" {

    output("Hello, PWALang!");

    output(42);

    plasma {

        output("Nested output");

    }

}

Expected output:

Hello, PWALang!

42

Nested output

Testable output collection
The runtime separates output collection from terminal printing.

collect_output() returns a Vec<String> that can be checked through automated tests.

execute() prints the collected values to standard output.

This design makes console-output behavior easier to verify without capturing terminal output during tests.
Current limitations
- Only numeric and string literals are supported.
- Variable references are not supported.
- Arithmetic expressions cannot be evaluated.
- No mutable runtime state or symbol table exists.
- Output is not connected to simulation diagnostics or scientific data export.
The runtime is intended as an early development utility, not a complete execution environment.
9. Rust Concepts Used
| Concept | Purpose |

|---|---|

| struct | Groups related data |

| enum | Defines alternative value variants |

| Vec<T> | Stores growable sequences |

| String | Stores owned text |

| f64 | Represents floating-point values |

| Option<T> | Represents optional values |

| Result<T, E> | Represents success or failure |

| match | Performs pattern matching |

| &self | Borrows an instance immutably |

| &mut self | Borrows an instance mutably |

| Box<T> | Provides indirection for recursive structures |

| ? | Propagates recoverable errors |

| #[test] | Declares an automated test |

The parser borrows the token sequence rather than taking ownership of it.

Its position changes as tokens are consumed.

The runtime borrows the parsed simulation and traverses its statements without modifying the AST.
10. Error Handling
The parser returns:

Result<Simulation, String>

A successful parse returns Ok(Simulation).

An unsuccessful parse returns Err(String).

Current errors include:
- Missing delimiters
- Invalid statement structures
- Missing property values
- Invalid output arguments
- Unknown function-style statements
- Unexpected trailing tokens
The lexer also rejects unsupported characters and malformed literals.

Future improvements should include source spans, line and column information, structured diagnostic codes, and clearer error messages.
11. Automated Testing
PWALang uses Rust's built-in testing framework.

cargo test

At the time of this documentation update, the reported test suite contains 19 passing tests.

Tests cover:
- Lexical analysis
- Arithmetic token recognition
- Simulation declarations
- Properties and nested blocks
- Console-output parsing
- Rejection of malformed syntax
- Runtime output ordering
New language features should include both positive and negative tests. Planned CLI integration tests should cover valid external files, missing arguments, unreadable files, incorrect extensions, and malformed source.

Passing tests demonstrate correctness for the tested cases, not complete correctness for all possible inputs.
12. Planned Semantic Analysis
The semantic analyzer will operate on the AST.

Its responsibilities are expected to include:
- Recognizing supported scientific declarations
- Validating parameter names and types
- Checking physical dimensions and units
- Enforcing numerical and physical constraints
- Rejecting unsupported or ambiguous configurations
- Preparing validated data for intermediate representation generation
For example, a numeric value representing plasma density should not be interpreted without a defined unit convention.
13. Planned PWIR
PWIR stands for PWALang Intermediate Representation.

It is intended to represent validated simulation configurations independently of a specific solver's input syntax.

The AST captures how the user wrote the configuration.

PWIR will instead capture a normalized, validated representation suitable for backend processing.

Its final schema has not yet been defined.
14. Planned Backend Architecture
The initial target is WarpX, potentially through its Python PICMI interface.

Other possible backends include HiPACE++ and FBPIC.

Backend support will require explicit capability checks.

Simulation engines differ in their numerical models, geometries, physical approximations, and supported features.

PWALang must not silently treat scientifically incompatible configurations as equivalent.
15. Development Principles
1. Prioritize scientific correctness over convenience.
2. Keep parsing separate from physical validation.
3. Prefer explicit assumptions and units.
4. Reject unsupported backend features clearly.
5. Maintain automated tests for language behavior.
6. Document implemented features separately from proposals.
7. Preserve reproducibility and traceability of generated configurations.
16. Contributor Workflow
From the nested Rust project directory:

cargo fmt

cargo test

cargo run

Before submitting changes, contributors should verify that the test suite passes, run an example .pwa file, and update documentation when language behavior changes.

The compiler architecture will evolve as PWALang matures.