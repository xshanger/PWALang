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

## 5. Formal Grammar

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

## 6. Syntax Errors

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

## 7. Syntax Versus Scientific Validity

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

## 8. Planned Language Features

The following features are proposals, not implemented syntax:

- Physical quantities with explicit units
- Validated plasma and particle-beam schemas
- Simulation domains and numerical settings
- Diagnostics and output configuration
- Solver capability validation
- Solver-independent intermediate representation
- External `.pwa` file loading through a command-line interface

Language changes will be documented here as implementations and tests are added.

## 9. Running the Current Compiler

From the nested Rust project directory:

```bash
cargo run
cargo test
```

The executable currently parses a source example embedded in `src/main.rs`.

See [Compiler Architecture](compiler-architecture.md) for implementation details.
