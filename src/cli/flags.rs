use std::str::FromStr;
use std::env;

/// Represents supported CLI flags like --timing
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CliFlags {
    Timing,
}

impl FromStr for CliFlags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--timing" | "-t" => Ok(CliFlags::Timing),
            _ => Err(()),
        }
    }
}

/// Parse les flags et renvoie une version propre des arguments restants
pub fn parse_and_clean_args() -> (Vec<CliFlags>, Vec<String>) {
    let mut flags = Vec::new();
    let mut args_clean = Vec::new();
    for arg in std::env::args().skip(1) {
        match arg.parse::<CliFlags>() {
            Ok(flag) => flags.push(flag),
            Err(_) => args_clean.push(arg),
        }
    }

    (flags, args_clean)
}
