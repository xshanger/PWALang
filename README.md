# PWALang

### A Domain-Specific Language for Plasma Wakefield Accelerator Simulations

**PWALang** is an experimental, open-source domain-specific language (DSL) designed to simplify the definition, configuration, and portability of plasma wakefield accelerator (PWFA) simulations.

The long-term goal is to provide a unified interface through which researchers and students can describe accelerator simulation experiments without having to rewrite their configurations for every supported simulation engine.

PWALang is being developed in **Rust**, with an emphasis on compiler architecture, scientific correctness, extensibility, and reproducibility.

> **Project Status:** Early development. PWALang currently provides an experimental lexer, parser, and abstract syntax tree (AST). It is not yet capable of running plasma simulations.

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

The proposed architecture is:

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
        +----> WarpX
        |
        +----> HiPACE++ (planned)
        |
        +----> FBPIC (planned)
```

The intermediate representation is intended to describe simulation parameters independently of any particular backend.

However, different simulation engines use different physical approximations, numerical methods, and supported features. PWALang will need to explicitly validate backend compatibility rather than assume that all simulation configurations are interchangeable.

## Example Syntax

An experimental PWALang configuration:

```text
simulation "PWFA_Test" {
    plasma {
        density = 1e17;
    }

    beam {
        energy = 1.0;
    }
}
```

The current compiler can parse this nested structure into an abstract syntax tree.

The example is not yet a physically validated simulation configuration. Physical units, parameter constraints, and solver-specific compatibility checks are planned features.

## Development Roadmap

| Stage | Description | Status |
|---|---|---|
| 1 | Rust project foundation and lexer | Complete |
| 2 | Initial parser and AST | Complete |
| 3 | Nested simulation blocks | In progress |
| 4 | Physical units and semantic validation | Planned |
| 5 | Solver-independent intermediate representation (PWIR) | Planned |
| 6 | WarpX backend integration | Planned |
| 7 | Scientific validation and reproducibility testing | Planned |
| 8 | Additional simulation backends | Future |

## Technology

- **Language:** Rust
- **Compiler architecture:** Lexer, parser, AST, semantic analysis, intermediate representation
- **Simulation domain:** Plasma wakefield acceleration and particle-in-cell simulation
- **Initial target backend:** WarpX through its Python PICMI interface
- **Development environment:** Visual Studio Code and GitHub Codespaces

## Getting Started

### Prerequisites

Install the Rust toolchain using [rustup](https://rustup.rs/).

### Clone the repository

```bash
git clone https://github.com/xshanger/PWALang.git
cd PWALang/PWALang
```

### Run the compiler

```bash
cargo run
```

### Run automated tests

```bash
cargo test
```

### Format the source code

```bash
cargo fmt
```

At this stage, the executable parses an example embedded in the Rust source code. Loading external `.pwa` files through a command-line interface is planned.

## Long-Term Goals

The project is intended to explore several possibilities:

1. A readable and expressive language for defining plasma accelerator experiments.
2. Unit-aware configuration validation to reduce common scientific input errors.
3. A solver-independent intermediate representation for compatible simulation models.
4. Backend adapters for established plasma simulation engines.
5. Reproducible simulation configurations suitable for research and education.
6. A foundation for future accelerator simulation tooling and scientific workflows.

These are development goals, not currently implemented capabilities.

## Contributing

PWALang is currently an independent experimental project in its early stages.

Suggestions, bug reports, technical discussions, and contributions are welcome as the language design evolves.

If you are interested in compiler development, plasma physics, accelerator science, or scientific computing, feel free to open a GitHub issue.

## Author

Developed by **xshanger**.

Created out of a personal interest in particle accelerator physics, scientific computing, and building tools that may make computational research more accessible.

## License

A license has not yet been selected. Until one is added, the repository should not be assumed to grant open-source reuse or redistribution rights.
