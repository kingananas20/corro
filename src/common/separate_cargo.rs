use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> = LazyLock::new(|| unsafe {
    Regex::new(r"(?sm)^(.*?^\s*(?:Running|error: could not compile|Finished)[^\n]*\n?)(.*)")
        .unwrap_unchecked()
});

pub fn separate_cargo_output(input: &str) -> (&str, &str) {
    let Some(cap) = REGEX.captures(input) else {
        return (Default::default(), input);
    };

    let cargo = cap.get(1).map(|m| m.as_str()).unwrap_or_default();

    let output = cap.get(2).map(|m| m.as_str()).unwrap_or_default();

    (cargo, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_warnings_and_output() {
        let input = r#"   Compiling playground v0.0.1 (/playground)
warning: unused variable: `hm`
  --> src/main.rs:14:9
   |
14 |     let hm: HashMap<i32, i32> = collection!{1:1,2:2,3:3};
   |         ^^ help: if this is intentional, prefix it with an underscore: `_hm`
   |
   = note: `#[warn(unused_variables)]` on by default
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.65s
     Running `target/debug/playground`
Hello, world!
This is my actual output!"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("Compiling playground v0.0.1"));
        assert!(cargo.contains("warning: unused variable"));
        assert!(cargo.contains("Running"));
        assert!(!cargo.contains("Hello, world!"));
        assert_eq!(output, "Hello, world!\nThis is my actual output!");
    }

    #[test]
    fn test_clean_compile_no_warnings() {
        let input = r#"   Compiling my-app v1.2.3 (/home/user/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.5s
     Running `target/debug/my-app`
Program output here"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("Compiling my-app v1.2.3"));
        assert!(cargo.contains("Running"));
        assert!(!cargo.contains("Program output"));
        assert_eq!(output, "Program output here");
    }

    #[test]
    fn test_compilation_error_could_not_compile() {
        let input = r#"   Compiling playground v0.0.1 (/playground)
error: expected `;`, found `println`
 --> src/main.rs:2:15
  |
2 |     let x = 10
  |               ^ help: add `;` here
3 |     println!("Hello, world!");
  |     ------- unexpected token
warning: unused variable: `x`
 --> src/main.rs:2:9
  |
2 |     let x = 10
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
warning: `playground` (bin "playground") generated 1 warning
error: could not compile `playground` (bin "playground") due to 1 previous error; 1 warning emitted
"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("error: expected `;`"));
        assert!(cargo.contains("error: could not compile"));
        assert!(output.is_empty());
    }

    #[test]
    fn test_multiline_program_output() {
        let input = r#"   Compiling test v0.1.0 (/test)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/test`
Line 1
Line 2
Line 3
Final line"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("Running"));
        assert_eq!(output, "Line 1\nLine 2\nLine 3\nFinal line");
    }

    #[test]
    fn test_no_cargo_output() {
        let input = "Just program output\nNo cargo stuff here";
        let (cargo, output) = separate_cargo_output(input);

        assert_eq!(cargo, "");
        assert_eq!(output, "Just program output\nNo cargo stuff here");
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let (cargo, output) = separate_cargo_output(input);

        assert_eq!(cargo, "");
        assert_eq!(output, "");
    }

    #[test]
    fn test_no_program_output() {
        let input = r#"   Compiling silent v1.0.0 (/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.3s
     Running `target/debug/silent`
"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("Running"));
        assert_eq!(output, "");
    }

    #[test]
    fn test_release_build() {
        let input = r#"   Compiling myapp v2.0.0 (/workspace)
    Finished `release` profile [optimized] target(s) in 5.0s
     Running `target/release/myapp`
Release output"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("Compiling myapp v2.0.0"));
        assert!(cargo.contains("Running"));
        assert_eq!(output, "Release output");
    }

    #[test]
    fn test_multiple_compiling_lines() {
        let input = r#"   Compiling dep1 v1.0.0
   Compiling dep2 v2.0.0
   Compiling myapp v0.1.0 (/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.0s
     Running `target/debug/myapp`
Output here"#;

        let (cargo, output) = separate_cargo_output(input);

        assert!(cargo.contains("Compiling dep1"));
        assert!(cargo.contains("Compiling myapp"));
        assert!(cargo.contains("Running"));
        assert_eq!(output, "Output here");
    }
}
