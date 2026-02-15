#[derive(Clone)]
pub enum TextCategory {
    Jargon,
    Acronym,
    Normal,
}
#[derive(Clone)]
pub struct LabelPkg {
    pub text: String,
    pub tooltip: Option<String>,
    pub category: TextCategory,
}

fn get_acronyms() -> std::collections::HashMap<String, String> {
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

fn get_jargon() -> std::collections::HashMap<String, String> {
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

// Segment description into phrases, words, punctuation, formatting, and whitespace
// "\b(?:CLA|ORGAN PROCUREMENT ORGANIZATION)\b|[a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+\.[a-zA-Z0-9_-]+|[\w’]+|[\s\S]+?"gmi
// https://regex101.com
pub fn segment_description(desc: &str) -> Vec<Vec<LabelPkg>> {
    // Assemble regex expression
    let newlines: String = String::from(r"\n+");
    let all_other_visible: String = String::from(r"\S");
    let words: String = String::from(r"[\w’'-]+");
    let email_addresses: String = String::from(r"[a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+\.[a-zA-Z0-9_-]+");
    let mut phrases: String = String::from(r"\b(?:");
    // Add all jargon 
    let jargon = get_jargon();
    for (key, value) in &jargon {
        phrases.push_str(format!("{key}|").as_str());
    }
    // Add all acronyms 
    let acronyms = get_acronyms();
    for (key, value) in &acronyms {
        phrases.push_str(format!("{key}|").as_str());
    }
    phrases.pop(); // Remove last |
    phrases.push_str(r")\b");
    // Put it all together
    let expr = String::from(format!("(?im){phrases}|{email_addresses}|{words}|{all_other_visible}|{newlines}"));
    // Compile regex
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
        captures.push(pkg);
    }
    return segments;
}


