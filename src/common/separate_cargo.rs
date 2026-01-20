use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> = LazyLock::new(|| unsafe {
    Regex::new(r"(?s)^(\s*Compiling\s+[a-zA-Z0-9_-]+\s+v\d+\.\d+\.\d+(?:-[a-zA-Z0-9.]+)?(?:\s+\([^)]+\))?\n(?:.*\n)*?\s*Running\s+`[^`]+`\n)(.*)").unwrap_unchecked()
});

pub fn separate_cargo_output(input: &str) -> (&str, &str) {
    let Some(cap) = REGEX.captures(input) else {
        println!("No captures");
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

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("Compiling playground v0.0.1"));
        assert!(cargo.contains("warning: unused variable"));
        assert!(cargo.contains("Running `target/debug/playground`"));
        assert_eq!(output, "Hello, world!\nThis is my actual output!");
    }

    #[test]
    fn test_clean_compile_no_warnings() {
        let input = r#"   Compiling my-app v1.2.3 (/home/user/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.5s
     Running `target/debug/my-app`
Program output here"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("Compiling my-app v1.2.3"));
        assert!(!cargo.contains("warning"));
        assert_eq!(output, "Program output here");
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

        println!("{cargo}\n\n{output}");

        assert_eq!(output, "Line 1\nLine 2\nLine 3\nFinal line");
        assert!(cargo.contains("Compiling"));
    }

    #[test]
    fn test_no_cargo_output() {
        let input = "Just program output\nNo cargo stuff here";
        let (cargo, output) = separate_cargo_output(input);
        println!("{cargo}\n\n{output}");
        assert_eq!(cargo, "");
        assert_eq!(output, "Just program output\nNo cargo stuff here");
    }

    #[test]
    fn test_empty_input() {
        let input = "";

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

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

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("Compiling silent v1.0.0"));
        assert_eq!(output, "");
    }

    #[test]
    fn test_release_build() {
        let input = r#"   Compiling myapp v2.0.0 (/workspace)
    Finished `release` profile [optimized] target(s) in 5.0s
     Running `target/release/myapp`
Release output"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("Compiling myapp v2.0.0"));
        assert!(cargo.contains("release"));
        assert_eq!(output, "Release output");
    }

    #[test]
    fn test_prerelease_version() {
        let input = r#"   Compiling beta-app v1.0.0-beta.1 (/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/beta-app`
Beta output"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("beta-app v1.0.0-beta.1"));
        assert_eq!(output, "Beta output");
    }

    #[test]
    fn test_crate_name_with_hyphens_and_underscores() {
        let input = r#"   Compiling my_crate-name v3.14.159 (/path)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.8s
     Running `target/debug/my_crate-name`
Output"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("my_crate-name v3.14.159"));
        assert_eq!(output, "Output");
    }

    #[test]
    fn test_with_compilation_errors() {
        let input = r#"   Compiling broken v0.1.0 (/project)
error[E0425]: cannot find value `foo` in this scope
  --> src/main.rs:2:5
   |
2  |     foo
   |     ^^^ not found in this scope

error: aborting due to previous error
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.2s
     Running `target/debug/broken`
"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("error[E0425]"));
        assert!(cargo.contains("aborting due to previous error"));
        assert!(output.is_empty())
    }

    #[test]
    fn test_output_with_special_characters() {
        let input = r#"   Compiling app v1.0.0 (/app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/app`
Special chars: 你好 🦀 "quotes" 'apostrophes' \backslash/ [brackets]"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert_eq!(
            output,
            r#"Special chars: 你好 🦀 "quotes" 'apostrophes' \backslash/ [brackets]"#
        );
        assert!(cargo.contains("Compiling"));
    }

    #[test]
    fn test_output_with_ansi_codes() {
        let input = "   Compiling app v1.0.0 (/app)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.0s\n     Running `target/debug/app`\n\x1b[31mRed text\x1b[0m\nNormal text";

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(output.contains("\x1b[31mRed text\x1b[0m"));
        assert!(cargo.contains("Compiling"));
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

        println!("{cargo}\n\n{output}");

        // Should capture from FIRST Compiling to Running
        assert!(cargo.starts_with("   Compiling dep1"));
        assert!(cargo.contains("Compiling myapp"));
        assert_eq!(output, "Output here");
    }

    #[test]
    fn test_very_long_output() {
        let program_output = "Line\n".repeat(1000);
        let input = format!(
            "   Compiling app v1.0.0 (/app)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.0s\n     Running `target/debug/app`\n{}",
            program_output
        );

        let (cargo, output) = separate_cargo_output(&input);

        println!("{cargo}\n\n{output}");

        assert_eq!(output.lines().count(), 1000);
        assert!(cargo.contains("Compiling"));
    }

    #[test]
    fn test_empty_program_output_with_trailing_newline() {
        let input = r#"   Compiling app v1.0.0 (/app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/app`
"#;

        let (cargo, output) = separate_cargo_output(input);

        println!("{cargo}\n\n{output}");

        assert!(cargo.contains("Running `target/debug/app`"));
        assert_eq!(output, "");
    }
}
