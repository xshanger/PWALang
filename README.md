# PWALang

### A Domain-Specific Language for Plasma Wakefield Accelerator Simulations

**PWALang** is an experimental, publicly developed domain-specific language (DSL) designed to simplify the definition, configuration, and portability of plasma wakefield accelerator (PWFA) simulations.

The long-term goal is to provide a unified interface through which researchers and students can describe accelerator simulation experiments without having to rewrite configurations for every compatible simulation engine.

PWALang is developed in **Rust**, emphasizing compiler architecture, scientific correctness, extensibility, and reproducibility.

> **Project Status — Early Development (v0.1):** PWALang currently supports lexical analysis, recursive-descent parsing, abstract syntax tree (AST) generation, nested simulation blocks, literal console output, and loading external `.pwa` files. It cannot yet execute plasma simulations.

## Why I Created PWALang

My interest in particle accelerator physics began long before I started this project.

I've always been fascinated by how charged particles behave under extreme electromagnetic conditions, how plasma can be used to accelerate particles, and how accelerator technology contributes to our understanding of fundamental physics.

While exploring particle-in-cell (PIC) simulations and working with tools such as OSIRIS, I became increasingly interested in the computational side of accelerator research.

These simulation engines are powerful, but they often have different configuration formats, input conventions, numerical methods, and workflows.

That led me to a question:

**What if we could describe a plasma accelerator experiment once and translate that description into configurations for different compatible simulation engines?**

PWALang is my attempt to explore that idea.

This project is also a personal learning journey. I want to deepen my understanding of compiler design, scientific computing, numerical simulation, and plasma accelerator physics by building something that connects these disciplines.

I am developing PWALang independently, driven by curiosity and a long-term ambition to contribute to experimental particle accelerator research.

## Project Vision

PWALang aims to separate the scientific description of a simulation from the implementation details of individual simulation engines.

The proposed scientific compilation architecture is:

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
Semantic & Dimensional Validation
        |
        v
PWALang Intermediate Representation (PWIR)
        |
        v
 Backend Adapters
        |
        +----> WarpX (initial target)
        |
        +----> HiPACE++ (planned)
        |
        +----> FBPIC (planned)
```

The intermediate representation is intended to describe validated simulation parameters independently of any particular backend.

Different simulation engines use different physical approximations, numerical methods, and supported features. PWALang must explicitly validate backend compatibility rather than assume that all simulation configurations are interchangeable.

**The semantic validation, PWIR, backend adapters, and simulation execution stages are not yet implemented.**

### Current Execution Pipeline

The currently implemented development pipeline is:

```text
External .pwa File
        |
        v
Command-Line Interface
        |
        v
   File Loading
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
 Terminal Output
```

The console runtime provides basic output functionality for testing the language. It is not a plasma simulation engine.

## Current Language Features

| Feature | Status |
|---|---|
| Lexical analysis | Implemented |
| Numeric and string literals | Implemented |
| Scientific notation | Implemented |
| Simulation declarations | Implemented |
| Property assignments | Implemented |
| Nested configuration blocks | Implemented |
| Arithmetic operator tokenization | Implemented |
| Preliminary mathematical expression AST types | Defined, not yet integrated |
| Console `output()` statement | Implemented for literals |
| External `.pwa` file loading | Implemented |
| Command-line execution | Implemented |
| Arithmetic expression parsing | Planned |
| Arithmetic evaluation | Planned |
| Variable references | Planned |
| Physical units and dimensional validation | Planned |
| Scientific simulation execution | Planned |

## Example Syntax

PWALang currently supports the following program:

```pwa
simulation "PWFA_Test" {
    output("Starting PWALang!");

    plasma {
        density = 1e17;
        output("Plasma configuration loaded");
    }

    beam {
        energy = 1.0;
        output("Beam configuration loaded");
    }
}
```

The program can be parsed into an AST and its `output()` statements executed.

Expected terminal output:

```text
Starting PWALang!
Plasma configuration loaded
Beam configuration loaded
```

The `plasma` and `beam` blocks are currently syntactic structures. Their properties are not yet scientifically validated or passed to a simulation solver.

### Arithmetic Support

PWALang recognizes these arithmetic operators:

| Operator | Operation |
|---|---|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `^` | Exponentiation |
| `()` | Parentheses |

These operators are currently recognized by the lexer.

The following is **planned syntax**, not yet executable:

```pwa
simulation "Math_Test" {
    output(10 + 20 * 3);
}
```

Future expression parsing and evaluation will enable mathematical operations with appropriate precedence and error handling.

## Getting Started

### Prerequisites

Install the Rust toolchain using [rustup](https://rustup.rs/).

You can develop PWALang using Visual Studio Code, GitHub Codespaces, or another Rust-compatible development environment.

### Clone the Repository

```bash
git clone https://github.com/xshanger/PWALang.git
cd PWALang/PWALang
```

The Rust project is currently located inside the repository's nested `PWALang/` directory.

### Create a PWALang Program

Create a file named `examples/hello.pwa` inside the Rust project directory:

```pwa
simulation "My_First_Program" {
    output("Hello from PWALang!");
    output(42);

    plasma {
        density = 1e17;
        output("Plasma configured!");
    }
}
```

### Run a `.pwa` File

From the Rust project directory:

```bash
cargo run -- examples/hello.pwa
```

Expected output:

```text
Hello from PWALang!
42
Plasma configured!
```

The `--` separates Cargo's arguments from the file path passed to PWALang.

Alternatively, build the executable:

```bash
cargo build
```

Then execute the program directly:

```bash
./target/debug/pwalang examples/hello.pwa
```

The current command-line interface expects exactly one `.pwa` file path.

### Run Automated Tests

```bash
cargo test
```

The latest reported test run passed **19 tests**, covering lexical analysis, parsing, nested blocks, output statements, malformed syntax, and runtime output ordering.

External file-loading integration tests have not yet been added.

### Format Rust Source Code

```bash
cargo fmt
```

### Visual Studio Code Support

PWALang source files use the `.pwa` extension.

VS Code can open and edit these files. Basic syntax highlighting can be configured through file associations, although a dedicated PWALang language extension has not yet been developed.

The compiler can be executed through VS Code's integrated terminal or a configured build task.

Future editor integration may include dedicated syntax highlighting, code completion, diagnostics, and language-server support.

## Development Roadmap

| Stage | Description | Status |
|---|---|---|
| 1 | Rust project foundation and lexer | Complete |
| 2 | Initial parser and AST | Complete |
| 3 | Nested simulation blocks | Complete |
| 4 | Basic console output runtime | Complete |
| 5 | External `.pwa` file loading and CLI | Implemented; integration testing pending |
| 6 | Arithmetic expression parsing and evaluation | In progress — lexer and preliminary AST |
| 7 | Physical units and semantic validation | Planned |
| 8 | Solver-independent intermediate representation (PWIR) | Planned |
| 9 | WarpX backend integration | Planned |
| 10 | Scientific validation and reproducibility testing | Planned |
| 11 | Additional simulation backends | Future |

## Technology

- **Implementation language:** Rust
- **Compiler architecture:** Lexer, recursive-descent parser, AST, planned semantic analysis and intermediate representation
- **Current runtime:** Literal console output
- **Simulation domain:** Plasma wakefield acceleration and particle-in-cell simulation
- **Initial target backend:** WarpX, potentially through its Python PICMI interface
- **Development environment:** Visual Studio Code and GitHub Codespaces
- **Version control:** Git and GitHub

## Long-Term Goals

PWALang is intended to explore several possibilities:

1. A readable and expressive language for defining plasma accelerator experiments.
2. Unit-aware configuration validation to reduce common scientific input errors.
3. A solver-independent intermediate representation for compatible simulation models.
4. Backend adapters for established plasma simulation engines.
5. Reproducible simulation configurations suitable for research and education.
6. Mathematical expression support for scientific parameters and plasma profiles.
7. Developer tooling for authoring, validating, and executing `.pwa` configurations.

These are development goals, not currently implemented scientific capabilities.

## Contributing

PWALang is currently an independent experimental project in its early stages.

Suggestions, bug reports, technical discussions, and contributions are welcome as the language design evolves.

If you are interested in compiler development, plasma physics, accelerator science, or scientific computing, feel free to open a GitHub issue.

Before submitting code changes, contributors should run:

```bash
cargo fmt
cargo test
```

Language changes should include appropriate tests and documentation updates.

## Documentation

- [Language Reference](docs/language-reference.md)
- [Compiler Architecture](docs/compiler-architecture.md)

PWALang documentation is maintained alongside the compiler.

Implemented features and planned features are documented separately.

## Author

Developed by **xshanger**.

Created out of a personal interest in particle accelerator physics, scientific computing, and building tools that may make computational research more accessible.

## License

A license has not yet been selected.

Although the source code is publicly accessible, the repository should not be assumed to grant permission for reuse, modification, or redistribution until an appropriate license is added.
