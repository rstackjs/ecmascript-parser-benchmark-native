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
| SWC Next | 11.72 ms | 11.47 ms | 15.81 ms | 1.00× |
| Yuku | 12.81 ms | 12.61 ms | 13.35 ms | 1.09× |
| Oxc | 14.23 ms | 13.80 ms | 14.95 ms | 1.21× |
| SWC | 29.56 ms | 28.93 ms | 49.58 ms | 2.52× |

### [checker.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/checker.ts)

**File size:** 2.95 MB

![Bar chart comparing native parser speeds for checker.ts](charts/checker.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 4.07 ms | 3.85 ms | 4.33 ms | 1.00× |
| Oxc | 4.42 ms | 4.28 ms | 5.01 ms | 1.09× |
| Yuku | 4.49 ms | 4.35 ms | 4.70 ms | 1.10× |
| SWC | 9.77 ms | 9.52 ms | 10.36 ms | 2.40× |

### [lib.dom.d.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/lib.dom.d.ts)

**File size:** 2.24 MB

![Bar chart comparing native parser speeds for lib.dom.d.ts](charts/lib_dom.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 1.15 ms | 1.09 ms | 2.18 ms | 1.00× |
| Oxc | 1.43 ms | 1.39 ms | 1.51 ms | 1.24× |
| Yuku | 1.60 ms | 1.56 ms | 1.68 ms | 1.39× |
| SWC | 3.60 ms | 3.48 ms | 3.88 ms | 3.14× |

### [react.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/react.js)

**File size:** 0.07 MB

![Bar chart comparing native parser speeds for react.js](charts/react.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next | 0.05 ms | 0.05 ms | 0.08 ms | 1.00× |
| Oxc | 0.07 ms | 0.06 ms | 0.12 ms | 1.24× |
| Yuku | 0.07 ms | 0.07 ms | 0.11 ms | 1.28× |
| SWC | 0.18 ms | 0.17 ms | 0.26 ms | 3.37× |

## Semantic

The ECMAScript specification defines a set of early errors that conformant implementations must report before execution. Some of these are detectable during parsing from local context alone, like `return` outside a function, `yield` outside a generator, invalid destructuring, etc. Others require knowledge of the program's scope structure and bindings, such as redeclarations, unresolved exports, private fields used outside their class, etc.

Parsers handle this differently: SWC checks some scope-dependent errors during parsing itself, while Yuku, Oxc, and SWC Next provide a separate semantic analysis pass. This keeps parsing fast and lets each consumer opt in only to the work it actually needs. A formatter, for example, only needs the AST and should not pay the cost of scope resolution.

The benchmarks below measure parsing followed by this additional pass, which builds a scope tree and symbol table, resolves identifier references to their declarations, and reports the remaining early errors. SWC Next enables `SyntaxChecks::all()`; Yuku enables `check = true`. These timings do not establish equivalent conformance coverage.

### [typescript.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/typescript.js)

![Bar chart comparing parser speeds with semantic analysis for typescript.js](charts/typescript_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 24.52 ms | 24.10 ms | 29.39 ms | 1.00× |
| Yuku + Semantic | 27.34 ms | 27.02 ms | 28.51 ms | 1.12× |
| Oxc + Semantic | 29.82 ms | 29.38 ms | 31.74 ms | 1.22× |

### [checker.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/checker.ts)

![Bar chart comparing parser speeds with semantic analysis for checker.ts](charts/checker_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 9.31 ms | 9.08 ms | 10.27 ms | 1.00× |
| Oxc + Semantic | 10.13 ms | 10.01 ms | 11.44 ms | 1.09× |
| Yuku + Semantic | 10.14 ms | 10.01 ms | 10.39 ms | 1.09× |

### [lib.dom.d.ts](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/lib.dom.d.ts)

![Bar chart comparing parser speeds with semantic analysis for lib.dom.d.ts](charts/lib_dom_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 2.34 ms | 2.25 ms | 3.05 ms | 1.00× |
| Oxc + Semantic | 2.60 ms | 2.55 ms | 2.70 ms | 1.11× |
| Yuku + Semantic | 2.67 ms | 2.59 ms | 2.78 ms | 1.14× |

### [react.js](https://raw.githubusercontent.com/yuku-toolchain/parser-benchmark-files/refs/heads/main/react.js)

![Bar chart comparing parser speeds with semantic analysis for react.js](charts/react_semantic.svg)

| Parser | Median | Min | p99 | Relative |
|--------|--------|-----|-----|----------|
| SWC Next + Semantic | 0.11 ms | 0.11 ms | 0.21 ms | 1.00× |
| Yuku + Semantic | 0.15 ms | 0.14 ms | 0.24 ms | 1.30× |
| Oxc + Semantic | 0.16 ms | 0.15 ms | 0.27 ms | 1.36× |

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