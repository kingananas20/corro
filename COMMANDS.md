# Rust Discord Bot — Command Reference

---

## ✅ Supported Commands

  prefix = `!`

- `<prefix>cargo run` — Run Rust code.
- `<prefix>cargo publish` — Upload code to GitHub Gist.
- `/version` — Show toolchain version used (rustc, cargo).
- `/explain <E####>` — Explain a Rust compiler error code.
- `/crates` — Show the available crates to use when running code.
- `/crate info <name>` — Get informations about a crate.

---

# 🧪 Planned Commands

## 🔧 Code Utilities

- `<prefix>fmt` — Format Rust code using `rustfmt`.
- `<prefix>clippy` — Run clippy lints on code and return warnings/suggestions.
- `<prefix>miri` — Run rust code using the miri interpreter
- `<prefix>check` — Type-check code without running.
- `<prefix>build` — Try building the code and return success/errors.
- `<prefix>bench` — Benchmark simple functions. // probably not
- `<prefix>test` — Run unit tests in provided code. // supported in run command

---

## 📚 Learning & Assistance

- `<prefix>doc <item>` — Fetch docs from [docs.rs](https://docs.rs) or Rust stdlib. // if possible
- `<prefix>book <chapter/topic>` — Link to a section in *The Rust Book*.  // if possible
- `<prefix>edition` — Convert code to a specific Rust edition (2015/2018/2021). // probably not

---

## 🧠 Linting & Help

- `<prefix>help <topic>` — Explain common Rust idioms.
- `<prefix>lint` — Style or logic suggestions (simplified clippy).  // no clippy is good
- `<prefix>tips` — Random Rust tip or idiom with example.
- `<prefix>why <concept>` — Explain *why* a concept exists (e.g., "why lifetimes?").

---

## 🔍 Metadata & Tooling

- `<prefix>crate <name>` — Fetch crate info from crates.io.
- `<prefix>features <crate>` — Show optional features of a crate. // maybe

---

## 🎯 Community & Fun (maybe)

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
