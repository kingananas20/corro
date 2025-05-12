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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunConfig {
    channel: Channel,
    mode: Mode,
    code: String,
}

impl RunConfig {
    pub fn new(channel: Channel, mode: Mode, code: String) -> RunConfig {
        RunConfig {
            channel,
            mode,
            code,
        }
    }
}

pub fn parse_run_command(command: &str, code: &str) -> RunConfig {
    let parts = command.split_whitespace();

    let mut channel = Channel::Stable;
    let mut mode = Mode::Release;

    for arg in parts {
        match arg.to_lowercase().as_str() {
            "release" => mode = Mode::Release,
            "debug" => mode = Mode::Debug,
            "stable" => channel = Channel::Stable,
            "beta" => channel = Channel::Beta,
            "nightly" => channel = Channel::Nightly,
            _ => {}
        }
    }

    RunConfig {
        channel,
        mode,
        code: code.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let examples = [
            "/run nightly release",
            "/run release",
            "/run debug beta",
            "/run",
            "/run unknown stable",
        ];

        let parsed = [
            RunConfig::new(Channel::Nightly, Mode::Release, "".into()),
            RunConfig::new(Channel::Stable, Mode::Release, "".into()),
            RunConfig::new(Channel::Beta, Mode::Debug, "".into()),
            RunConfig::new(Channel::Stable, Mode::Release, "".into()),
            RunConfig::new(Channel::Stable, Mode::Release, "".into()),
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
