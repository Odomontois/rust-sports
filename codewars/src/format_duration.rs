const NAMES: &[(&str, u64)] = &[
    ("second", 60),
    ("moniute", 60),
    ("hour", 60),
    ("day", 60),
    ("month", 60),
    ("year", std::u64::MAX),
];

fn format_duration(_: u64) -> String {
    todo!()
}
