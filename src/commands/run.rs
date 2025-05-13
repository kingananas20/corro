// Create rust playground api crate

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Release,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edition {
    E2018,
    E2021,
    E2024,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteRequest {
    channel: Channel,
    mode: Mode,
    edition: Edition,
    code: String,
}

impl ExecuteRequest {
    pub fn new(channel: Channel, mode: Mode, edition: Edition, code: String) -> ExecuteRequest {
        ExecuteRequest {
            channel,
            mode,
            edition,
            code,
        }
    }
}

pub fn parse_run_command(command: &str, code: &str) -> ExecuteRequest {
    let parts = command.split_whitespace();

    let mut channel = Channel::Stable;
    let mut mode = Mode::Release;
    let mut edition = Edition::E2024;

    for arg in parts {
        match arg.to_lowercase().as_str() {
            "release" => mode = Mode::Release,
            "debug" => mode = Mode::Debug,
            "stable" => channel = Channel::Stable,
            "beta" => channel = Channel::Beta,
            "nightly" => channel = Channel::Nightly,
            "2018" | "e2018" => edition = Edition::E2018,
            "2021" | "e2021" => edition = Edition::E2021,
            "2024" | "e2024" => edition = Edition::E2024,
            _ => {}
        }
    }

    ExecuteRequest {
        channel,
        mode,
        edition,
        code: code.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let examples = [
            "/run nightly release 2018",
            "/run 2021 release",
            "/run debug beta",
            "/run",
            "/run unknown e2021 stable",
        ];

        let parsed = [
            ExecuteRequest::new(Channel::Nightly, Mode::Release, Edition::E2018, "".into()),
            ExecuteRequest::new(Channel::Stable, Mode::Release, Edition::E2021, "".into()),
            ExecuteRequest::new(Channel::Beta, Mode::Debug, Edition::E2024, "".into()),
            ExecuteRequest::new(Channel::Stable, Mode::Release, Edition::E2024, "".into()),
            ExecuteRequest::new(Channel::Stable, Mode::Release, Edition::E2021, "".into()),
        ];

        for i in 0..examples.len() {
            assert_eq!(
                parse_run_command(examples[i], ""),
                parsed[i],
                "should be the same output: {}",
                i
            );
        }
    }
}
