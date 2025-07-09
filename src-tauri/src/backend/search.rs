use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

use crate::backend::model::ApplicationInformation;

// TODO refactor
pub fn search(
    app_name: &str,
    applications: Vec<ApplicationInformation>,
) -> Vec<ApplicationInformation> {
    let applications_name = applications
        .iter()
        .map(|app| app.name.clone())
        .collect::<Vec<String>>();

    let mut matcher = Matcher::new(Config::DEFAULT);
    let matches = Pattern::new(
        app_name,
        CaseMatching::Ignore,
        Normalization::Smart,
        AtomKind::Fuzzy,
    )
    .match_list(applications_name, &mut matcher)
    .iter()
    .map(|result| result.0.clone())
    .collect::<Vec<String>>();

    matches
        .iter()
        .map(|app_name| applications.iter().find(|app| app.name == app_name.clone()))
        .map(|app_filter| app_filter.unwrap().clone())
        .collect::<Vec<ApplicationInformation>>()
}
