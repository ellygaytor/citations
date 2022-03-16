use crate::citations::{Citation, ContributorType};

/// Format a citation in the APA style.
/// Heavily reccomended but not required are the following fields in the citation:
/// - title
/// - authors
/// - date
/// - pages
/// Returns a string containing the formatted citation.
/// APA style citations are formatted as follows:
/// Last, M. F. (Year, Month Date). Title. Source. DOI/URL.
/// If the citation does not contain a DOI or URL, the DOI/URL field is left blank.
/// If the citation does not contain a source, the source field is left blank.
/// If there are multiple authors, the authors are listed with a comma separating them.
/// IE: Last, M. F., Last, M. F., & Last, M. F. (Year, Month Date). Title. Source. DOI/URL.
pub fn format_apa(citation: &Citation) -> String {
    let mut citation_string = String::new();
    let mut authors = String::new();
    let contributors = citation.contributors.clone().unwrap();
    for contributor in contributors {
        if contributor.position == ContributorType::Author {
            if !authors.is_empty() {
                authors.push_str(", ");
            }
            if let Some(last_name) = &contributor.last_name {
                authors.push_str(last_name);
            }
            if let Some(name) = &contributor.name {
                if !name.is_empty() {
                    authors.push_str(", ");
                }
                authors.push_str(&name[0..1]);
                authors.push('.');
            }
            if let Some(middle_name) = &contributor.middle_name {
                if !middle_name.is_empty() {
                    authors.push(' ');
                }
                authors.push_str(&middle_name[0..1]);
                authors.push('.');
            }

            if let Some(suffix) = &contributor.suffix {
                if !suffix.is_empty() {
                    authors.push(' ');
                }
                authors.push_str(suffix);
            }
        }
    }
    if !authors.is_empty() {
        citation_string.push_str(&authors);
        citation_string.push(' ');
    }
    if let Some(date) = &citation.date {
        citation_string.push('(');
        citation_string.push_str(&date.format("%B %d, %Y").to_string());
        citation_string.push_str("");
        citation_string.push_str("). ");
    }
    if let Some(title) = &citation.title {
        citation_string.push_str(title);
        citation_string.push_str(". ");
    }

    if let Some(source) = &citation.source {
        citation_string.push_str(source);
        citation_string.push_str(". ");
    }
    if let Some(url) = &citation.url {
        citation_string.push_str(url);
        citation_string.push_str(". ");
    }
    if let Some(doi) = &citation.doi {
        citation_string.push_str(doi);
        citation_string.push_str(". ");
    }
    // Add pages if they exist. If only one page is specified, prefix it with "p. ". If multiple pages are specified, prefix them with "pp. ". Surround the pages with parentheses.
    if let Some((start_page, end_page)) = &citation.pages {
        citation_string.push('(');
        if end_page.is_some() {
            citation_string.push_str("pp. ");
        } else {
            citation_string.push_str("p. ");
        }

        citation_string.push_str(&start_page.to_string());

        if let Some(end_page) = end_page {
            citation_string.push('-');
            citation_string.push_str(&end_page.to_string());
        }
        citation_string.push(')');
        citation_string.push('.');
    }
    citation_string
}
