pub mod citations;
pub mod formats;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::citations::{Citation, Contributor, ContributorType};

    #[test]
    fn test_apa() {
        let citation: Citation = Citation {
            title: Some("The Title".to_string()),
            contributors: Some(vec![
                Contributor {
                    organization: false,
                    last_name: Some("Last".to_string()),
                    name: Some("First".to_string()),
                    middle_name: None,
                    suffix: None,
                    position: ContributorType::Author,
                },
                Contributor {
                    organization: false,
                    last_name: Some("Last".to_string()),
                    name: Some("First".to_string()),
                    middle_name: None,
                    suffix: None,
                    position: ContributorType::Author,
                },
            ]),
            date: Some(NaiveDate::from_ymd(2019, 1, 1)),
            accessed: None,
            isbn: Some("123456789".to_string()),
            publisher: None,
            city_published: None,
            volume: None,
            issue: None,
            chapter: None,
            pages: Some((1, Some(2))),
            url: None,
            source: None,
            number: None,
            language: None,
            edition: None,
            doi: None,
        };

        assert_eq!(
            "Last, F., Last, F. (January 01, 2019). The Title. (pp. 1-2).",
            crate::formats::format_apa(&citation)
        );
    }
}
