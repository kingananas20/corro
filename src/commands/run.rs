use playground_api::endpoints::{Channel, CrateType, Edition, ExecuteRequest, Mode};

pub fn parse_run_command(command: &str, code: String) -> ExecuteRequest {
    let parts = command.split_whitespace();

    let mut channel = Channel::Stable;
    let mut mode = Mode::Debug;
    let mut edition = Edition::Edition2024;
    let mut crate_type = CrateType::Binary;
    let mut tests = false;
    let mut backtrace = false;

    for arg in parts {
        match arg.to_lowercase().as_str() {
            "-r" => mode = Mode::Release,
            "beta" => channel = Channel::Beta,
            "nightly" => channel = Channel::Nightly,
            "2015" | "e2015" => edition = Edition::Edition2015,
            "2018" | "e2018" => edition = Edition::Edition2018,
            "2021" | "e2021" => edition = Edition::Edition2021,
            "binary" | "bin" => crate_type = CrateType::Binary,
            "library" | "lib" => crate_type = CrateType::Library,
            "tests" => tests = true,
            "backtrace" => backtrace = true,
            _ => {}
        }
    }

    ExecuteRequest {
        channel,
        mode,
        edition,
        crate_type,
        tests,
        backtrace,
        code: code.to_owned(),
    }
}

#[cfg(test)]
mod tests {}
