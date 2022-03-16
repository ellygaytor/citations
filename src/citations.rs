use chrono::naive::NaiveDate;

/// Struct to store all of the information about a citation.
#[derive(Clone)]
pub struct Citation {
    /// The title of the publication.
    pub title: Option<String>,
    /// The list of contributors to the publication. 
    pub contributors: Option<Vec<Contributor>>,
    /// The date of publication.
    pub date: Option<NaiveDate>,
    /// The date the publication was accessed on.
    pub accessed: Option<NaiveDate>,
    /// The ISBN of the publication.
    pub isbn: Option<String>,
    /// The publisher of the publication.
    pub publisher: Option<String>,
    /// The city where the publication was published.
    pub city_published: Option<String>,
    /// The volume of the publication.
    pub volume: Option<String>,
    /// The issue of the publication.
    pub issue: Option<String>,
    /// The chapter cited.
    pub chapter: Option<String>,
    /// The pages cited.
    pub pages: Option<(u32, Option<u32>)>,
    /// The URL of the publication.
    pub url: Option<String>,
    /// The source of the publication.
    pub source: Option<String>,
    /// The number of the publication.
    pub number: Option<String>,
    /// The language of the publication.
    pub language: Option<String>,
    /// The edition of the publication.
    pub edition: Option<String>,
    /// The DOI of the publication.
    pub doi: Option<String>,
}

/// Struct to store information about a contributor to a publication.
#[derive(Clone)]
pub struct Contributor {
    /// Whether the contributor is an organization or a person. 
    /// True if the contributor is an organization, false if the contributor is a person.
    pub organization: bool,
    /// The last name of the contributor.
    pub last_name: Option<String>,
    /// The first name of the contributor.
    /// If the contributor is an organization, this field is used for the name of the organization.
    pub name: Option<String>,
    /// The middle name of the contributor.
    pub middle_name: Option<String>,
    /// The suffix of the contributor.
    pub suffix: Option<String>,
    /// The position of the contributor. 
    pub position: ContributorType,
}

/// Enum to store the type of contributor.
#[derive(Clone, Debug, PartialEq)]
pub enum ContributorType {
    /// The contributor is an author.
    Author,
    /// The contributor is an authority.
    Authority,
    /// The contributor is an editor.
    Editor,
    /// The contributor is a translator.
    Translator,
    /// The contributor is a reviewer.
    Reviewer,
    /// The contributor is a director.
    Director,
    /// The contributor is a composer.
    Composer,
}
