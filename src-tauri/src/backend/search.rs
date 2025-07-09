use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

pub fn search<'a>(app_name: &str, applications_name: &'a Vec<String>) -> Vec<&'a str> {
    let mut matcher = Matcher::new(Config::DEFAULT);
    let matches = Pattern::new(
        app_name,
        CaseMatching::Ignore,
        Normalization::Smart,
        AtomKind::Fuzzy,
    )
    .match_list(applications_name, &mut matcher);

    matches.iter().map(|(s, _)| s.as_str()).collect()
}
