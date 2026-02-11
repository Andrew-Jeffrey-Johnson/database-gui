
pub enum TokenType {
    Jargon,
    Acronym,
    Normal,
}

struct TokenContainer {
    token: String,
    raw_tooltip: Option<String>,
    tt: TokenType
}

pub fn segment_description(desc: &str) -> Vec<String> {
    // Capture all lines that have content and trim leading/trailing whitespace
    let mut expr = String::from(r"(?:[\S]+[ \t]*)+");
    let re = regex::Regex::new(expr.as_str()).unwrap();
    let it = re.captures_iter(desc);
    let mut segments: Vec<String> = Vec::<String>::new();
    for cap in it {
        let seg: String = cap[0].to_string();
        segments.push(seg);
    }
    return segments;
}


//https://regex101.com/r/lf8r4y/1
// The regex expression to group every word separate from punctuation and newlines 
// except spaces. Ignores case for the first group
//(?i:Cascade Life Alliance|organ procurement organization)|(\n)|(\b\w+\b)|([•])|([^ ])/gm
// without groups:
// /Cascade Life Alliance|organ procurement organization|\b\w+\b|[\n\r\v\f]|\S|[\t ]*/gmi
//fn annotate_description(original_description: &str) -> Vec<TokenContainer> {
//    // Create dictionary
//    let mut dictionary = std::collections::HashMap::new();
//    dictionary.insert(
//        "ITG".to_string(),
//        "Information Technology Group".to_string(),
//    );
//    dictionary.insert(
//        "Information Technology Group".to_string(),
//        "ITG".to_string(),
//    );
//    dictionary.insert(
//        "OHSU".to_string(),
//        "Oregon Health and Science University".to_string(),
//    );
//    dictionary.insert(
//        "Oregon Health and Science University".to_string(),
//        "OHSU".to_string(),
//    );
//    dictionary.insert(
//        "CLA".to_string(),
//        "Cascade Life Alliance".to_string(),
//    );
//    dictionary.insert(
//        "Cascade Life Alliance".to_string(),
//        "CLA".to_string(),
//    );
//    // Create regex expression
//    let mut expr = String::from(r"/((?i)");
//    // Add all keys 
//    let itr = dictionary.clone().into_keys();
//    for key in itr {
//        expr.push_str(format!("{key}|").as_str());
//    }
//    expr.push_str(r")?|(?:[\S]+[[:blank:]]*)+|\n");
//    // Compile regex
//    let re = regex::Regex::new(expr.as_str()).unwrap();
//    // Get all matches
//    let it = re.captures_iter(original_description);
//    // Make a vector containing labels for all matches 
//    let mut segments: Vec<Vec<TokenContainer>> = Vec::<Vec::<TokenContainer>>::new();
//    for cap in it {
//        let token: String = cap[0].to_string();
//        let raw_tooltip: Option<String> = dictionary.get(&token).cloned();
//        if raw_tooltip.is_some() {
//            labels.push(TokenContainer{token: token, raw_tooltip: raw_tooltip, tt: TokenType::Jargon})
//        }
//        else if token == "\n".to_string() {
//            labels.push(TokenContainer{token: token, raw_tooltip: raw_tooltip, tt: TokenType::Newline})
//        }
//        else {
//            labels.push(TokenContainer{token: token, raw_tooltip: raw_tooltip, tt: TokenType::Other})
//        }
//    }
//    return labels;
//}
