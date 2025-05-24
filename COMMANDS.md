# Rust Discord Bot — Command Reference

---

## ✅ Supported Commands

- `<prefix>run` — Run Rust code.
- `<prefix>share` — Upload code to GitHub Gist.

---

## 🧪 Planned Commands

- Run code with the Miri interpreter (`miri`)
- Explain error codes (`explain`)

---

## 🔧 Code Utilities

- `<prefix>fmt` — Format Rust code using `rustfmt`.
- `<prefix>clippy` — Run clippy lints on code and return warnings/suggestions.
- `<prefix>check` — Type-check code without running.
- `<prefix>build` — Try building the code and return success/errors.
- `<prefix>bench` — Benchmark simple functions (if sandboxed).
- `<prefix>test` — Run unit tests in provided code.

---

## 📚 Learning & Assistance

- `<prefix>doc <item>` — Fetch docs from [docs.rs](https://docs.rs) or Rust stdlib.
- `<prefix>book <chapter/topic>` — Link to a section in *The Rust Book*.
- `<prefix>play <code>` — Run code on the Rust Playground (or generate link).
- `<prefix>edition` — Convert code to a specific Rust edition (2015/2018/2021).
- `<prefix>explain <E####>` — Explain a Rust compiler error code.

---

## 🧠 Linting & Help

- `<prefix>help <topic>` — Explain common Rust idioms or errors.
- `<prefix>lint` — Style or logic suggestions (simplified clippy).
- `<prefix>tips` — Random Rust tip or idiom with example.
- `<prefix>why <concept>` — Explain *why* a concept exists (e.g., "why lifetimes?").

---

## 🔍 Metadata & Tooling

- `<prefix>crate <name>` — Fetch crate info from crates.io.
- `<prefix>depgraph` — Generate a dependency graph.
- `<prefix>version` — Show toolchain version used (rustc, cargo).
- `<prefix>features <crate>` — Show optional features of a crate.

---

## 🎯 Community & Fun

- `<prefix>quote` — Show a random quote from the Rust community.
- `<prefix>meme` — Post a Rust programming meme.
- `<prefix>vote <poll>` — Start a reaction-based poll.

---

## 🔒 Security & Best Practices

- `<prefix>audit` — Run `cargo audit` on shared dependencies.
- `<prefix>unsafe` — Count `unsafe` blocks in code and warn.

---

## 📝 Notes

- `<prefix>` should be replaced with your bot's prefix (e.g. `!`, `?`, `rust!`).
- Some commands may require integration with external tools or APIs like:
  - [Rust Playground](https://play.rust-lang.org/)
  - [docs.rs](https://docs.rs)
  - [crates.io](https://crates.io)
  - GitHub Gist
