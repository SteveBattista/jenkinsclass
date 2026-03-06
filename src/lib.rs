use std::{env, sync::LazyLock};

/// In-memory fortunes loaded from `data/fortunes.txt`.
pub static FORTUNES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    include_str!("../data/fortunes.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
});

/// Parses `--name <value>` from any iterator of CLI-like arguments.
#[must_use]
pub fn parse_name_from_args<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        if arg.as_ref() == "--name" {
            return args
                .next()
                .map_or_else(|| String::from("you"), |value| value.as_ref().to_string());
        }
    }

    String::from("you")
}

/// Assembles the fortune message for the provided recipient name.
#[must_use]
pub fn print_fortune(fortune: &str, name: &str) -> String {
    format!(
        "🥠 Fortune Cookie 🥠\nThis fortune is for {name}.\n\n\"{fortune}\"\n\n✨ May your fortune come true! ✨"
    )
}

/// Parses `--name <value>` from CLI args and falls back to `you` when absent.
#[must_use]
pub fn get_name_from_args() -> String {
    parse_name_from_args(env::args().skip(1))
}
