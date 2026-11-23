# Native ECMAScript Parser Benchmark

This repository is a fork of [yuku-toolchain/ecmascript-parser-benchmark-native](https://github.com/yuku-toolchain/ecmascript-parser-benchmark-native), maintained to update parser benchmark data and add parsers.

Benchmarks for ECMAScript parsers compiled to native binaries (Zig, Rust), measuring raw parsing speed without any JavaScript runtime overhead.

## System

| Property | Value |
|----------|-------|
| OS | macOS 26.6.2 (arm64) |
| CPU | Apple M5 Max |
| Cores | 18 |
| Memory | 64 GB |

## Parsers

### [Yuku](https://github.com/yuku-toolchain/yuku)

**Language:** Zig

A high-performance & spec-compliant JavaScript/TypeScript compiler toolchain written in Zig.

### [Oxc](https://github.com/oxc-project/oxc)

**Language:** Rust

A high-performance JavaScript and TypeScript parser written in Rust.

### [SWC](https://github.com/swc-project/swc)

**Language:** Rust

An extensible Rust-based platform for compiling and bundling JavaScript and TypeScript.

### [SWC Next](https://github.com/swc-project/swc-next)

**Language:** Rust

The next-generation SWC JavaScript and TypeScript parser.

Measured on 2026-11-23.

| Parser | Published version |
|--------|-------------------|
| Yuku | 0.28.1 |
| Oxc | 0.159.0 |
| SWC | swc_ecma_parser 47.0.0 / swc_common 27.0.0 |
| SWC Next | 0.7.0 |

## Benchmarks

### [typescript.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/typescript.js)

**File size:** 7.83 MB

![Bar chart comparing native parser speeds for typescript.js](charts/typescript.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 11.78 ms | 11.57 ms | 12.32 ms | 1.00× |
| Yuku | 14.83 ms | 12.82 ms | 21.23 ms | 1.26× |
| Oxc | 15.31 ms | 14.13 ms | 19.27 ms | 1.30× |
| SWC | 31.71 ms | 29.71 ms | 44.88 ms | 2.69× |

### [checker.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/checker.ts)

**File size:** 2.95 MB

![Bar chart comparing native parser speeds for checker.ts](charts/checker.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 4.47 ms | 4.13 ms | 6.06 ms | 1.00× |
| Oxc | 4.74 ms | 4.39 ms | 6.40 ms | 1.06× |
| Yuku | 5.22 ms | 4.56 ms | 6.55 ms | 1.17× |
| SWC | 10.31 ms | 9.79 ms | 16.24 ms | 2.31× |

### [lib.dom.d.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/lib.dom.d.ts)

**File size:** 2.24 MB

![Bar chart comparing native parser speeds for lib.dom.d.ts](charts/lib_dom.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 1.21 ms | 1.13 ms | 1.61 ms | 1.00× |
| Oxc | 1.52 ms | 1.43 ms | 2.07 ms | 1.26× |
| Yuku | 1.79 ms | 1.60 ms | 2.54 ms | 1.48× |
| SWC | 4.06 ms | 3.58 ms | 5.77 ms | 3.37× |

### [react.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/react.js)

**File size:** 0.07 MB

![Bar chart comparing native parser speeds for react.js](charts/react.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 0.06 ms | 0.05 ms | 0.08 ms | 1.00× |
| Oxc | 0.07 ms | 0.07 ms | 0.15 ms | 1.24× |
| Yuku | 0.07 ms | 0.07 ms | 0.19 ms | 1.34× |
| SWC | 0.20 ms | 0.19 ms | 0.28 ms | 3.68× |

## Semantic

The ECMAScript specification defines a set of early errors that conformant implementations must report before execution. Some of these are detectable during parsing from local context alone, like `return` outside a function, `yield` outside a generator, invalid destructuring, etc. Others require knowledge of the program's scope structure and bindings, such as redeclarations, unresolved exports, private fields used outside their class, etc.

Parsers handle this differently: SWC checks some scope-dependent errors during parsing itself, while Yuku, Oxc, and SWC Next provide a separate semantic analysis pass. This keeps parsing fast and lets each consumer opt in only to the work it actually needs. A formatter, for example, only needs the AST and should not pay the cost of scope resolution.

The benchmarks below measure parsing followed by this additional pass, which builds a scope tree and symbol table, resolves identifier references to their declarations, and reports the remaining early errors. SWC Next enables `SyntaxChecks::all()`; Yuku enables `check = true`. These timings do not establish equivalent conformance coverage.

### [typescript.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/typescript.js)

![Bar chart comparing parser speeds with semantic analysis for typescript.js](charts/typescript_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 25.90 ms | 24.43 ms | 34.46 ms | 1.00× |
| Yuku + Semantic | 30.90 ms | 28.09 ms | 39.30 ms | 1.19× |
| Oxc + Semantic | 31.71 ms | 30.07 ms | 42.02 ms | 1.22× |

### [checker.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/checker.ts)

![Bar chart comparing parser speeds with semantic analysis for checker.ts](charts/checker_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 9.68 ms | 9.33 ms | 13.10 ms | 1.00× |
| Oxc + Semantic | 10.54 ms | 10.18 ms | 11.91 ms | 1.09× |
| Yuku + Semantic | 10.99 ms | 10.18 ms | 12.64 ms | 1.14× |

### [lib.dom.d.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/lib.dom.d.ts)

![Bar chart comparing parser speeds with semantic analysis for lib.dom.d.ts](charts/lib_dom_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 2.56 ms | 2.31 ms | 3.30 ms | 1.00× |
| Yuku + Semantic | 2.85 ms | 2.67 ms | 3.71 ms | 1.11× |
| Oxc + Semantic | 2.90 ms | 2.68 ms | 4.12 ms | 1.13× |

### [react.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/react.js)

![Bar chart comparing parser speeds with semantic analysis for react.js](charts/react_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 0.11 ms | 0.11 ms | 0.14 ms | 1.00× |
| Yuku + Semantic | 0.16 ms | 0.15 ms | 0.27 ms | 1.40× |
| Oxc + Semantic | 0.17 ms | 0.16 ms | 0.33 ms | 1.49× |

## Run Benchmarks

### Prerequisites

- [Node.js 22.18 or later](https://nodejs.org/) - Script runtime
- [pnpm](https://pnpm.io/) - Package manager
- [Rust](https://www.rust-lang.org/tools/install) - For building Rust-based parsers
- [Zig](https://ziglang.org/download/) 0.17.0 - Required by Yuku 0.28.1

### Steps

1. Clone the repository:

```bash
git clone https://github.com/yuku-toolchain/ecmascript-parser-benchmark-native.git
cd ecmascript-parser-benchmark-native
```

2. Install dependencies:

```bash
pnpm install
```

3. Download the benchmark files:

```bash
pnpm run load-files
```

4. Build the parsers:

```bash
pnpm run build
```

5. Validate parser and semantic outputs:

```bash
pnpm validate
```

6. Run benchmarks:

```bash
pnpm bench
```

This will run benchmarks on all test files. Results are saved to the `result/` directory.

## Methodology

Parsing is timed in-process to isolate it from process startup, dynamic linking, file I/O, and memory teardown, which would otherwise dominate the measurement on smaller files.

The source is read once, then each parser runs 50 warmup iterations followed by 300 timed iterations. A monotonic clock wraps only the parse call (plus the semantic pass for the semantic variants); allocation and teardown happen outside the timed region, and the result passes through an optimization barrier so the work cannot be elided. Reported figures are the median, minimum, and 99th percentile of the timed runs.

Binaries are built with release optimizations: Rust with `cargo build --release` (LTO, single codegen unit, symbol stripping) and Zig with `zig build --release=fast`. Each uses a fast general-purpose allocator (Rust `mimalloc`, Zig `smp_allocator`).