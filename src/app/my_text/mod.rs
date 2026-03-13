use chrono::offset::TimeZone;
use chrono::offset::Utc;

#[derive(Clone, PartialEq, Debug)]
pub enum TextCategory {
    Jargon,
    Acronym,
    Normal,
}
#[derive(Clone, PartialEq, Debug)]
pub struct LabelPkg {
    pub text: String,
    pub tooltip: Option<String>,
    pub category: TextCategory,
}
#[derive(Clone, PartialEq, Debug)]
pub struct PostalAddress {
    pub name: String,
    pub address1: String,
    pub address2: String,
    pub address3: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Company {

}

#[derive(Clone, PartialEq, Debug)]
pub struct Achievement {
    pub id: i32,
    pub description: String,
    pub in_resume: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Experience {
    pub id: i32,
    pub start: chrono::DateTime<chrono::offset::Utc>,
    pub end: chrono::DateTime<chrono::offset::Utc>,
    pub company: String,
    pub address: PostalAddress,
    pub achievements: Vec<Achievement>,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Application {
    pub id: i32,
    pub experiences: Vec<Experience>,

    pub category: TextCategory,
}

pub fn get_achievements() -> Vec<Achievement> {
    let mut achievements: Vec<Achievement> = Vec::<Achievement>::new();
    for i in 0..30 {
        let text: String = format!("Achievement {}", i);
        let achievement: Achievement = Achievement {
            id: i,
            description: text,
            in_resume: false,
        };
        achievements.push(achievement);
    }
    return achievements;
}

pub fn get_experiences() -> Vec<Experience> {
    let mut experiences: Vec<Experience> = Vec::<Experience>::new();
    let intel = Experience {
        id: 0,
        start: Utc.with_ymd_and_hms(2024, 6, 24, 19, 0, 0).unwrap(),
        end: Utc.with_ymd_and_hms(2024, 11, 15, 23, 0, 0).unwrap(),
        company: String::from("Intel"),
        address: PostalAddress {
            name: String::from("Jones Farm Campus"),
            address1: String::from("2111 NE 25th Avenue"),
            address2: String::from(""),
            address3: String::from(""),
            city: String::from("Hillsboro"),
            state: String::from("OR"),
            zip: String::from("97124"),
        },
        achievements: get_achievements(),
    };
    experiences.push(intel);
    return experiences;
}

pub fn get_acronyms() -> std::collections::HashMap<String, String> {
    // Create map
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::<String, String>::new();
    map.insert(
        "ITG".to_string(),
        "Information Technology Group".to_string(),
    );
    map.insert(
        "OHSU".to_string(),
        "Oregon Health and Science University".to_string(),
    );
    map.insert(
        "CLA".to_string(),
        "Cascade Life Alliance".to_string(),
    );
    return map;
}

pub fn get_jargon() -> std::collections::HashMap<String, String> {
    // Create map
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::<String, String>::new();
    map.insert(
        "Information Technology Group".to_string(),
        "ITG".to_string(),
    );
    map.insert(
        "Oregon Health and Science University".to_string(),
        "OHSU".to_string(),
    );
    map.insert(
        "Cascade Life Alliance".to_string(),
        "CLA".to_string(),
    );
    return map;
}

// concatenate strings into a single regex expression
// Website used to help create this expression: https://regex101.com
fn assemble_regex_expression (
    jargon: &std::collections::HashMap<String, String>, 
    acronyms: &std::collections::HashMap<String, String>
    ) -> String {
    // Assemble regex expression
    let newlines: String = String::from(r"\n+");
    let all_other_visible: String = String::from(r"\S");
    let words: String = String::from(r"[\w’'-]+");
    let email_addresses: String = String::from(r"[a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+\.[a-zA-Z0-9_-]+");
    let mut phrases: String = String::from(r"\b(?:");
    // Add all jargon 
    for (key, value) in jargon {
        phrases.push_str(format!("{key}|").as_str());
    }
    // Add all acronyms 
    for (key, value) in acronyms {
        phrases.push_str(format!("{key}|").as_str());
    }
    phrases.pop(); // Remove last |
    phrases.push_str(r")\b");
    // Put it all together
    let expr = String::from(format!("(?im){phrases}|{email_addresses}|{words}|{all_other_visible}|{newlines}"));
    return expr;
}

// For each capture, label it
pub fn label_capture(
    text: String,
    jargon: &std::collections::HashMap<String, String>, 
    acronyms: &std::collections::HashMap<String, String> 
    ) -> LabelPkg {
    let tooltip: Option<String> = match (jargon.get(&text), acronyms.get(&text)) {
        (Some(j), None) => Some(j.to_string()),
        (None, Some(a)) => Some(a.to_string()),
        _ => None
    };
    let category: TextCategory = match (jargon.contains_key(&text), acronyms.contains_key(&text)) {
        (true, false) => TextCategory::Jargon,
        (false, true) => TextCategory::Acronym,
        _ => TextCategory::Normal
    };
    let pkg: LabelPkg = LabelPkg {
        text: text,
        tooltip: tooltip,
        category: category,
    };
    return pkg;
}

// Segment description into phrases, words, punctuation, formatting, and whitespace
pub fn segment_description(desc: &str) -> Vec<Vec<LabelPkg>> {
    // Compile regex
    let jargon = get_jargon();
    let acronyms = get_acronyms();
    let expr = assemble_regex_expression(&jargon, &acronyms);
    let re = regex::Regex::new(expr.as_str()).unwrap();
    // Label all captures
    let it = re.captures_iter(desc);
    let mut captures: Vec<LabelPkg> = Vec::<LabelPkg>::new();
    let mut segments: Vec<Vec<LabelPkg>> = Vec::<Vec<LabelPkg>>::new();
    for cap in it {
        let text: String = cap[0].to_string();
        // Each segment is a line that has text
        if text.contains("\n") {
            segments.push(captures);
            captures = Vec::<LabelPkg>::new();
            continue;
        }
        let pkg: LabelPkg = label_capture(text, &jargon, &acronyms);
        captures.push(pkg);
    }
    return segments;
}

//fn get_application() -> Application {
//    return 
//}
