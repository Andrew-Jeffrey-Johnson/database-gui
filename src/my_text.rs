
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
    for (key, _value) in jargon {
        phrases.push_str(format!("{key}|").as_str());
    }
    // Add all acronyms 
    for (key, _value) in acronyms {
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

pub fn latex_gen(
    summary: &String,
    //experiences: &Vec<Experience>,
    ) {
    let document_heading = String::from(r"
        \documentclass[12pt]{article}
        \usepackage[margin=0.5in]{geometry}
        \usepackage[document]{ragged2e}
        \pagenumbering{gobble}
        \usepackage{hyperref}
        \renewcommand{\familydefault}{\sfdefault}
        \usepackage[inline]{enumitem}
        \newlist{commalist}{description*}{4}
        \setlist[commalist]{itemjoin={{,}},itemjoin*={{, and}},afterlabel=\unskip{{~}}}
        \newlist{commalistnoand}{description*}{4}
        \setlist[commalistnoand]{itemjoin={{,}},afterlabel=\unskip{{~}}}
        \setlist{topsep=0pt,itemsep=0pt,partopsep=0pt, parsep=0pt}
        \setlength\parindent{0pt}
        \newcommand\sectionspacing{12pt}
        \begin{document}
            \raggedright
        ");
    let contact_section = String::from(r"
        \begin{center}
            {\fontsize{17.28pt}{0pt}\textbf{Andrew Johnson}}\\
            {Hillsboro, OR \(|\) (503)-360-8854 \(|\) andrew.jeffrey.johnson@gmail.com}\\
            {LinkedIn: \href{https://www.linkedin.com/in/andrew-jeffrey-johnson/}{andrew-jeffrey-johnson} \(|\) GitHub: \href{https://github.com/Andrew-Jeffrey-Johnson}{Andrew-Jeffrey-Johnson} \(|\) Website: \href{https://www.luminlapid.com}{www.luminlapid.com}}
        \end{center}
        ");
    let summary_section = format!(r"
        %\vspace{{-8pt}}
        {{\fontsize{{14pt}}{{0pt}}\textbf{{About}}}}\\
        \vspace{{3pt}}
        \hrule
        \vspace{{3pt}}
        \noindent{{{}}}
        ", summary);
    let professional_experience_heading = String::from(r"
        \vspace{\sectionspacing}
        {\fontsize{14pt}{0pt}\textbf{Professional Experience}}\\
        \vspace{3pt}
        \hrule
        \vspace{3pt}
        ");
    //let mut professional_experience_body = Vec::<String>::new();
   // for e in experiences {
   //     // Only add the experience if there is at least one selected achievement
   //     let mut any_selected = false;
   //     for a in &e.achievements {
   //         if a.in_resume {
   //             any_selected = true;
   //             break;
   //         }
   //     }
   //     if !any_selected {
   //         continue;
   //     }
   //     professional_experience_body.push(format!(r"
   //         \noindent{{\textbf{{{}}} – \textit{{{}}}}}\\
   //         {{{}}} 
   //         \hspace*{{\fill}}
   //         \textit{{{} - {}}}
   //         \begin{{itemize}}
   //         ", e.company, e.get_address(), e.title, e.get_start(), e.get_end()));
   //     for a in &e.achievements {
   //         if a.in_resume {
   //             professional_experience_body.push(format!(r"
   //                 \item {}
   //             ", a.variants[a.selected_variant].description));
   //         }
   //     }
   //     professional_experience_body.push(String::from(r"\end{itemize}"));
   // }
    let document_ending = String::from(r"
          \vspace{\sectionspacing}
          {\fontsize{14pt}{0pt}\textbf{Notable Projects}}\\
          \vspace{3pt}
          \hrule
          \vspace{3pt}
          \textbf{Personal Linux Server}
          \hspace*{\fill}
          \textit{December 2016 – Present}\\
          \begin{itemize}
            \item Host my website at {\href{https://www.luminlapid.com}{www.luminlapid.com}} on my own hardware at home using Linux and Django. 
            \item Enabled TCP port forwarding on Frontier, Nighthawk and eero routers for server.
            \item Installed Debian Linux server on a Dell Poweredge R630 server as upgrade to the HP Envy.
            \item Added an additional SSD, extending the drive partition and configuring it for use with the OS.
            \item Configured an Nginx server to route traffic between Minecraft, SFTP, and Node.js servers.
          \end{itemize}
          \textbf{Senior Project: Machine Learning \& Web Development}
          \hspace*{\fill}
          \textit{September 2021 – June 2022}\\
          \begin{itemize}
            \item Led a team of 4 students to develop a data-cleansing importer for Excel and CSV files.
            \item Implemented support vector machine learning to categorize transactions for Schedule F tax forms.
            \item Developed a web page for financial statement uploads using Python Django.
          \end{itemize}
          
          \textbf{Personal Project: Game Engine}
          \hspace*{\fill}
          \textit{July 2020 – January 2022}
          \begin{itemize}
            \item Spearheaded the development of a 3D game engine with 3 colleagues, utilizing C++ and OpenGL.
            \item Directed development through flowcharts and documentation.
            \item Ensured cross-platform compatibility by adding compile scripts for both Windows and Linux.
          \end{itemize}
          
          \vspace{\sectionspacing}
          {\fontsize{14pt}{0pt}\textbf{Education}}
          \vspace{3pt}
          \hrule
          \vspace{3pt}
          {Oregon State University – \textit{Corvallis, OR}}\\
          \begin{itemize}[label={}]
            \item {Master of Engineering in computer science, 3.55/4.0 GPA}
            \hspace*{\fill}
            \textit{September 2022 – June 2023}
            \item {Bachelor of Science in computer science, 3.73/4.0 GPA}
            \hspace*{\fill}
            \textit{September 2019 – June 2022}
          \end{itemize}
          \textit{Relevant Courses:}
          \begin{commalistnoand}
            \item Cyber Attacks \& Defense
            \item Software Engineering 2
            \item Machine Learning
            \item Web Development
          \end{commalistnoand}
        \end{document}
        ");
    use std::fs::File;
    use std::io::Write;
    let f = File::create("output_resume/resume.tex");
    let _ = write!(f.expect("REASON"), 
        "{}{}{}{}{}", 
        document_heading, 
        contact_section,
        summary_section,
        professional_experience_heading,
        //professional_experience_body.join(" "),
        document_ending);
    // Generate PDF
    use std::process::Command;
    let result = Command::new("pdflatex")
        .arg("-output-directory=./output_resume")
        .arg("-jobname=Andrew_Johnson")
        .arg("output_resume/resume.tex")
        .output();
    match result {
        Ok(output) => println!("Success: {}", output.status),
        Err(e) => eprintln!("Failed to run command: {}", e),
    }
}
