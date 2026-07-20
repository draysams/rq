# Build `rq`: A Rust CLI Data Query Tool

---

## What you're building

`rq` ("Rust Query") is a command-line tool that grows from a one-line `println!` into a
`grep` + `jq` + `csvkit` hybrid: it reads text, CSV, or JSON, filters it, and prints
formatted output. Every phase adds one real capability and one or two new Rust concepts,
in the order the borrow checker will actually make you learn them.

**Approach:** Every phase starts with **failing tests**. You write (or paste in) the
test first, watch it fail (won't even compile at first — that's normal and expected),
then write the minimum code to make it pass, then refactor. This is TDD, Rust-flavored.

**How to use this doc:**

- Work top to bottom. Don't skip phases — each one deliberately sets up the pain that
  the next phase's concept resolves.
- Each phase has: **Concepts**, **Read First**, **Task**, **Tests**, **Hints**, **Done when**.
- When the compiler yells at you, that error is part of the curriculum. Read it fully
  before searching for help.
- Keep these open in tabs the whole time:
  - [The Rust Book](https://doc.rust-lang.org/book/) (referred to below as "the Book")
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [std docs](https://doc.rust-lang.org/std/)
  - [Rust Cheat Sheet for JS/TS devs](https://cheats.rs/) — genuinely excellent for your background

---

## Phase 0 — Environment & First Compile

**Concepts:** `cargo`, crates vs. modules vs. packages, the compile/run loop.

**Read first:**

- Book ch. 1: [Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)

**Task:**

1. Install via [rustup](https://rustup.rs/).
2. `cargo new rq --bin && cd rq`
3. Look at the generated `Cargo.toml` and `src/main.rs`. Compare mentally to
   `package.json` — `Cargo.toml` is your manifest, `Cargo.lock` is your `package-lock.json`.
4. Run `cargo run`. Then `cargo build --release` and find the binary in `target/release`.

**Tests:** None yet — but from this point forward, run `cargo test` after every phase
even on phases that don't ask you to. Getting into that habit now matters more than
the tests themselves at this stage.

**Hints:**

- `cargo check` is your fast feedback loop (type-checks without producing a binary) —
  use it constantly instead of `cargo run` while iterating.

**Done when:** `cargo run` prints `Hello, world!` and you understand what `cargo check`,
`cargo build`, and `cargo run` each do differently.

---

## Phase 1 — Reading Args (No Crates Yet)

**Concepts:** `std::env::args`, `Vec<String>`, ownership basics, `String` vs `&str`.

**Read first:**

- Book ch. 4.1–4.3: [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) — read this properly, it's the chapter everything else depends on.
- Book ch. 12.1: [Accepting Command Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)

**Task:** Make `rq` echo back whatever argument you pass it.

```
cargo run -- hello
# prints: You said: hello
```

**Tests (TDD — write this first, in `src/main.rs` or a new `tests/cli.rs`):**

```rust
// tests/cli.rs
use std::process::Command;

#[test]
fn echoes_the_argument() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("hello")
        .output()
        .expect("failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("You said: hello"));
}
```

### DONE

Run with `cargo test`. It will fail to compile or fail the assertion — good, that's step one of TDD.

**Hints:**

- `std::env::args()` returns an iterator of `String`, where `args[0]` is the binary path — same gotcha as `process.argv[0]`/`[1]` in Node.
- `.collect::<Vec<String>>()` turns the iterator into a vector you can index.
- If you get a "cannot move out of index" error, that's your first real ownership lesson — try `.get(1)` and `.clone()` and notice the difference. Don't over-think it yet; you'll revisit this in Phase 3.

**Done when:** the CLI test passes and you can explain, in your own words, why
`args[1]` alone might not compile the way you expect.

---

## Phase 2 — Read a File (Errors, `Result`, `?`)

**Concepts:** `Result<T, E>`, the `?` operator, `std::fs::read_to_string`, panics vs. errors.

**Read first:**

- Book ch. 9: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) — all of it, this is a load-bearing chapter.
- Book ch. 12.2–12.3: [Reading a File](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)

**Task:** `rq <path>` reads a file and prints its contents. If the file doesn't exist,
print a clean error to stderr and exit non-zero — no panic, no ugly backtrace.

**Tests:**

```rust
// tests/cli.rs (add to existing file)
use std::process::Command;

#[test]
fn prints_file_contents() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("tests/fixtures/sample.txt")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello from fixture"));
}

#[test]
fn missing_file_exits_nonzero_without_panicking() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .arg("does/not/exist.txt")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("panicked"));
}
```

Create the fixture: `mkdir -p tests/fixtures && echo "hello from fixture" > tests/fixtures/sample.txt`

**Hints:**

- This is the point where you split `main()` into a `run()` function returning
  `Result<(), Box<dyn std::error::Error>>` so you can use `?` — exactly the refactor
  the Book does in ch. 12.3. Do it, don't skip it; it's the pattern you'll use forever.
- Think of `Result<T, E>` as a discriminated union you're _forced_ to handle —
  TS's `{ ok: true, value } | { ok: false, error }` pattern, except the compiler
  enforces it instead of your linter suggesting it.
- `.unwrap()` is your escape hatch during exploration but should be gone from
  `main.rs` by the end of this phase — it's the equivalent of an uncaught throw.

**Done when:** both tests pass, and there is no `.unwrap()` left in your file-reading path.

---

## Phase 3 — `wc`-style Line/Word/Char Counts (Iterators)

**Concepts:** Iterators (`.lines()`, `.split_whitespace()`, `.chars()`), `map`/`filter`/`count`,
why iterators are lazy.

**Read first:**

- Book ch. 13.2–13.3: [Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust by Example: Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)

**Task:** Add a `count` subcommand-ish flag (still hand-rolled, no `clap` yet):

```
rq --count tests/fixtures/sample.txt
# lines: 1  words: 3  chars: 19
```

**Tests (TDD: write these against a small pure function, not the CLI, so you
practice unit-testing logic separately from I/O — an important Rust habit):**

```rust
// src/lib.rs (new file — see hint below)
pub fn count_stats(content: &str) -> (usize, usize, usize) {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    (lines, words, chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_a_simple_string() {
        assert_eq!(count_stats("hello world\nsecond line"), (2, 3, 23));
    }

    #[test]
    fn empty_string_is_all_zero() {
        assert_eq!(count_stats(""), (0, 0, 0));
    }
}
```

Run just these with `cargo test count_stats` (or `cargo test --lib`).

**Hints:**

- This is your cue to convert `rq` from a plain binary into a **binary + library**:
  create `src/lib.rs` for testable logic, keep `src/main.rs` thin (just arg-parsing
  and calling into the library). This mirrors keeping business logic out of your
  Express route handlers — same instinct, new syntax.
- `Cargo.toml` doesn't need edits for this — Cargo auto-detects `src/lib.rs` alongside
  `src/main.rs` as long as the package name matches.
- `.chars().count()` counts Unicode scalar values, not bytes — worth reading why
  `String` indexing by `[i]` doesn't compile in Rust at all (Book ch. 8.2, UTF-8 section).

**Done when:** unit tests in `lib.rs` pass, `main.rs` calls into `rq::count_stats`,
and you can explain why putting logic in the lib crate makes it independently testable.

---

## Phase 4 — Grep: Pattern Matching Over Lines (Structs, Lifetimes-lite)

**Concepts:** structs, `impl` blocks, borrowing across function boundaries, basic lifetimes.

**Read first:**

- Book ch. 5: [Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- Book ch. 12.4–12.5: [Refactoring / Test-Driven Development in Rust](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html) — this chapter is _literally_ TDD in Rust, read it closely, it's the best-fit reference in this whole roadmap.
- Book ch. 10.3: [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) (skim — just enough to recognize `'a`)

**Task:** `rq grep <pattern> <path>` prints matching lines, like a tiny `grep`.

**Tests (TDD — again, test the pure function before wiring up the CLI):**

```rust
// src/lib.rs
pub fn grep_lines<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(pattern)).collect()
}

#[cfg(test)]
mod grep_tests {
    use super::*;

    #[test]
    fn finds_matching_lines() {
        let content = "apple\nbanana\ngrapefruit";
        assert_eq!(grep_lines("apple", content), vec!["apple", "grapefruit"]);
    }

    #[test]
    fn no_match_returns_empty() {
        let content = "apple\nbanana";
        assert_eq!(grep_lines("zzz", content), Vec::<&str>::new());
    }

    #[test]
    fn case_sensitive_by_default() {
        let content = "Apple\napple";
        assert_eq!(grep_lines("apple", content), vec!["apple"]);
    }
}
```

**Hints:**

- The `'a` in `grep_lines<'a>` says "the strings I return borrow from `content`,
  don't outlive it." The compiler will refuse to compile this wrong — try deleting
  the lifetime annotation and read what it tells you before putting it back.
- Once this works, add a `Config` struct (pattern, path, case-insensitive flag) with
  an associated `fn build(args: &[String]) -> Result<Config, &str>` — this is the
  exact refactor the Book does in ch. 12.3, and it's the natural next step for your
  hand-rolled arg parsing before Phase 5 replaces it with `clap`.

**Done when:** all grep tests pass and `rq grep apple tests/fixtures/sample.txt` works from the shell.

---

## Phase 5 — Real Arg Parsing with `clap` (First External Crate)

**Concepts:** adding dependencies, derive macros, subcommands.

**Read first:**

- [clap derive tutorial](https://docs.rs/clap/latest/clap/_derive/index.html) — this is reference docs (exhaustive attribute list); start with the [cookbook](https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html) instead for task-by-task examples. For any `docs.rs` crate, look in the left sidebar under the crate name for sections like `_derive`, `_cookbook`, `_tutorial` — these are the guided docs hidden inside the reference.
- Book ch. 7: [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — you'll want proper modules (`src/commands/`) around now.

**Task:** Replace hand-rolled parsing with `clap`, using subcommands:

```
rq count <path>
rq grep <pattern> <path> [--ignore-case]
```

**Tests:** Re-run your Phase 1–4 integration tests (`tests/cli.rs`) unmodified except
for the new command shape — if they still pass after the refactor, you've proven the
refactor didn't change behavior. This is the "refactor" step of red-green-refactor,
just delayed a phase so you feel the value of having tests before a big rewrite.

Add one new test for the flag:

```rust
#[test]
fn grep_ignore_case_flag_matches_regardless_of_case() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .args(["grep", "APPLE", "tests/fixtures/sample.txt", "--ignore-case"])
        .output()
        .unwrap();
    assert!(!output.stdout.is_empty());
}
```

**Hints:**

- `cargo add clap --features derive`
- Model each subcommand as an enum variant with `#[derive(Parser)]` /
  `#[derive(Subcommand)]` — this is your first real encounter with Rust enums
  carrying data, which is a bigger deal than TS enums (closer to a tagged union/ADT).
- Expect this phase to break your existing tests briefly — that's expected; fix them
  to match the new CLI shape and move on.

**Done when:** `rq --help` shows generated subcommand help, and all integration tests
pass against the new interface.

---

## Phase 6 — CSV Support (`serde`, Vec<Struct>)

**Concepts:** `serde`, deriving `Deserialize`, `Vec<T>` of structs, the `csv` crate.

**Read first:**

- [serde.rs overview](https://serde.rs/) — read "Data model" and "Derive" sections
- [csv crate docs](https://docs.rs/csv/latest/csv/) — the "Reading" section

**Task:** `rq csv <path> --column <name>` prints just one column's values from a CSV file.

**Tests:**

```rust
// tests/fixtures/people.csv
// name,age,city
// Selene,30,St. Petersburg
// Ada,29,Concourse

#[test]
fn extracts_a_column() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .args(["csv", "tests/fixtures/people.csv", "--column", "name"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Selene"));
    assert!(stdout.contains("Ada"));
}

#[test]
fn unknown_column_errors_cleanly() {
    let output = Command::new(env!("CARGO_BIN_EXE_rq"))
        .args(["csv", "tests/fixtures/people.csv", "--column", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
```

Also write a unit test in `lib.rs` for the extraction logic itself, independent of the CLI,
same pattern as Phases 3–4.

**Hints:**

- `cargo add csv serde --features serde/derive`
- Start with `csv::Reader::from_path` and `StringRecord` (untyped) before reaching for
  `#[derive(Deserialize)]` structs — get the dumb version working first, then upgrade.
- This is where you'll feel the difference from TS most: there's no `any`. Every row
  is either successfully parsed into your shape or it's an error you must handle —
  no silent `undefined` sneaking through.

**Done when:** both tests pass and you have a unit test for the extraction function
that doesn't touch the filesystem.

---

## Phase 7 — JSON Support (Enums, Pattern Matching)

**Concepts:** `serde_json::Value`, `match`, recursive enums.

**Read first:**

- Book ch. 6: [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums-and-pattern-matching.html) — this is the chapter that makes `serde_json::Value` click.
- [serde_json docs](https://docs.rs/serde_json/latest/serde_json/enum.Value.html)

**Task:** `rq json <path> --path <dot.notation>` prints the value at a JSON path, e.g.
`rq json data.json --path user.address.city`.

**Tests:**

```rust
// src/lib.rs
use serde_json::Value;

pub fn get_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(value, |acc, key| acc.get(key))
}

#[cfg(test)]
mod json_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gets_nested_value() {
        let data = json!({"user": {"address": {"city": "Longford"}}});
        assert_eq!(get_path(&data, "user.address.city").unwrap(), "Longford");
    }

    #[test]
    fn missing_path_returns_none() {
        let data = json!({"user": {}});
        assert!(get_path(&data, "user.missing.path").is_none());
    }
}
```

**Hints:**

- `cargo add serde_json`
- `Value` is an enum: `Null, Bool, Number, String, Array, Object`. Write a `match`
  over it once by hand to print any value type nicely — this is the pattern-matching
  equivalent of a TS `switch` over a discriminated union, but exhaustive by
  compiler-enforcement (miss a variant, it won't compile).
- `try_fold` here is doing what `path.split('.').reduce(...)` with early-exit-on-undefined
  would do in JS — a good "aha" moment for how iterator combinators replace loops.

**Done when:** both tests pass and you can pretty-print an arbitrary JSON value's type
and content via a `match`.

---

## Phase 8 — Unify Behind a Trait (Polymorphism Rust-Style)

**Concepts:** traits, `dyn Trait` vs generics, trait objects.

**Read first:**

- Book ch. 10.2: [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- Book ch. 18.2: [Using Trait Objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)

**Task:** Define a `DataSource` trait with a `fn rows(&self) -> Vec<Vec<String>>` (or similar),
implement it for both your CSV and JSON readers, and have a single `filter`/`select`
code path work over either — this is the "unify formats" moment that makes `rq`
feel like a real tool instead of three separate scripts glued together.

**Tests:**

```rust
#[test]
fn csv_and_json_sources_produce_comparable_rows() {
    let csv_rows = CsvSource::from_path("tests/fixtures/people.csv").unwrap().rows();
    let json_rows = JsonSource::from_path("tests/fixtures/people.json").unwrap().rows();
    assert_eq!(csv_rows.len(), json_rows.len());
}
```

(Adjust to your actual trait shape — the point of this test is "two different
implementers of the same trait are interchangeable to calling code," write it to
prove that.)

**Hints:**

- Ask yourself: do I need dynamic dispatch (`Box<dyn DataSource>`, decided at
  runtime which file format) or static dispatch (`impl DataSource` generics,
  decided at compile time)? You genuinely need the runtime kind here since the file
  format is a CLI argument — good real-world case for `dyn`.
- This is the closest Rust concept to a TS `interface` — but note methods are opt-in
  per type via `impl Trait for Type`, not structural like TS's duck typing.

**Done when:** filtering/column-selection logic is written once, against the trait,
and works identically against both CSV and JSON fixtures.

---

## Phase 9 — Output Formatting & Error Polish (`thiserror`, `anyhow`)

**Concepts:** custom error types, `From` conversions, exit codes, `Display`.

**Read first:**

- [thiserror docs](https://docs.rs/thiserror/latest/thiserror/)
- [anyhow docs](https://docs.rs/anyhow/latest/anyhow/)
- Book ch. 9.1 revisited, specifically the section on custom error types.

**Task:**

- Define an `RqError` enum (`FileNotFound`, `InvalidColumn`, `ParseError`, etc.) with `thiserror`.
- Add `--format table|json|csv` output flag using a small formatting trait.
- Every failure path returns a specific, testable error variant — no more generic
  string errors, no panics anywhere in the codebase.

**Tests:**

```rust
#[test]
fn invalid_column_returns_specific_error_variant() {
    let err = extract_column(&fixture_records(), "nope").unwrap_err();
    assert!(matches!(err, RqError::InvalidColumn(_)));
}
```

**Hints:**

- `thiserror` for your library's error enum (typed, matchable); `anyhow` at the
  `main.rs` boundary if you want a catch-all `Result<(), anyhow::Error>` there —
  common, idiomatic split in real Rust CLIs.
- Run `cargo clippy` now if you haven't been — it will likely flag a handful of
  non-idiomatic patterns from earlier phases. Fixing them is worth doing here, once,
  in bulk.

**Done when:** `cargo clippy` is clean, every error path has a dedicated test, and
`main.rs` contains no `.unwrap()`/`.expect()` outside of tests.

---

## Phase 10 — Ship It

**Concepts:** integration test suites, `README`, `cargo publish` (optional), CI.

**Read first:**

- Book ch. 11.3: [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- Book ch. 14: [More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)

**Task:**

1. Write a full `tests/` suite covering every subcommand end-to-end (you've been
   accumulating these — now review for gaps).
2. Add a `README.md` documenting usage.
3. Optional: add a GitHub Actions workflow running `cargo test` + `cargo clippy` on push.
4. Optional stretch goals if you want to keep going past this roadmap:
   - a `--query` flag with simple filter expressions (`age > 25`) — parser-writing practice
   - streaming large files instead of reading fully into memory — `BufReader` practice
   - `async` file/network fetching for remote URLs — your first taste of `tokio`

**Done when:** `cargo test` runs a full suite covering count/grep/csv/json/format/errors
end-to-end, `cargo clippy` is silent, and you have a working binary you'd hand to
someone else without embarrassment.

---

## Quick Reference: TS → Rust Concept Map

| TypeScript                                        | Rust                              | Where you meet it |
| ------------------------------------------------- | --------------------------------- | ----------------- |
| `undefined` / optional chaining `?.`              | `Option<T>`                       | Phase 2, 7        |
| `try/catch`, thrown errors                        | `Result<T, E>`, `?` operator      | Phase 2           |
| `interface`                                       | `trait`                           | Phase 8           |
| discriminated union                               | `enum` with data                  | Phase 6, 7        |
| `readonly`, structural immutability-by-convention | ownership & borrowing, enforced   | Phase 1, 3, 4     |
| `zod`/runtime validation                          | `serde` derive + `Result`         | Phase 6, 7        |
| `npm install`                                     | `cargo add`                       | Phase 5 onward    |
| ESLint                                            | `clippy`                          | Phase 9           |
| Jest                                              | built-in `#[test]` + `cargo test` | every phase       |

---

## General debugging workflow for every phase

1. `cargo check` — fast, catches type errors.
2. `cargo test <name>` — run one test at a time while red.
3. Read the **whole** compiler error, including the "help:" lines — Rust's compiler
   frequently tells you the exact fix.
4. `cargo clippy` — idiomatic-style pass, run at the end of each phase, not just Phase 9.
5. Stuck longer than ~20 minutes on a borrow-checker fight? Simplify: clone the value
   (`.clone()`) to get something compiling, get the test green, then optimize away the
   clone once you understand _why_ the borrow checker objected. Don't fight the
   borrow checker and learn the concept in the same sitting — sequence those.
