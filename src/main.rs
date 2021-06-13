#![allow(non_snake_case)]
use regex::Regex;
use std::path::PathBuf;

type MainResult = Result<(),String>;
const SUCCESSFUL_EXIT: MainResult = Ok(());

fn main() -> MainResult {
    let args: Vec<String> = std::env::args().collect();
    let filePath = match args.len() {
        0 => {
            return Err("Unexpected no arguments passed to program.".into());
        },
        1 => {
            return Err("Please provide a path to a file to analyze.".into());
        },
        2 => { PathBuf::from(&args[1]) }
        n => {
            return Err(format!("Too many arguments passed to program, expected only one with a file path, got {}", n-1));
        }
    };

    let fileContent = match std::fs::read_to_string(&filePath) {
        Ok(content) => content,
        Err(e) => { return Err(format!("Failed to read file: {:?}, error: {}", filePath, e)); }
    };


    let transitionTableRegex = Regex::new(r"^\s*struct transition_table\b").unwrap();
    let mut lines = fileContent.lines();
    let mut transitionTableFound = false;
    for line in &mut lines {
        if transitionTableRegex.is_match(line) {
            transitionTableFound = true;
            break;
        }
    }

    if !transitionTableFound {
        return Err("Transition table was not found.".into());
    }

    let commentRegex = Regex::new(r"^\s*//").unwrap();
    let rowStartRegex = Regex::new(r"^\s*[\w:]*[rR]ow\s*<").unwrap();
    let mut rows = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if commentRegex.is_match(line) {
            continue;
        }
        if !rowStartRegex.is_match(line) {
            break;
        }

        let startOfRowContent = 1 + match line.find('<') {
            Some(index) => index,
            None => { return Err(format!("Character \"<\" was not found in row: {}", line)); }
        };
        let endOfRowContent = match line.rfind('>') {
            Some(index) => index,
            None => { return Err(format!("Character \">\" was not found in row: {}", line)); }
        };
        let substring = match line.get(startOfRowContent..endOfRowContent) {
            Some(substring) => substring,
            None => {
                return Err(format!("Failed to get substring with indexes: {}..{} from line: {}",
                                   startOfRowContent, endOfRowContent, line));
            }
        };
        let row = match Row::new(&substring.split(',').map(str::trim).map(removePrefix).collect::<Vec<_>>()) {
            Ok(row) => row,
            Err(_) => { return Err("Failed to create Row object from string sections".into()); }
        };
        rows.push(row);
    }

    if rows.is_empty() {
        return Err("No rows in transition table were found".into());
    }

    let mut outputString = String::from(format!("@startuml\nhide empty description\n[*] --> {}\n", rows[0].start));
    for row in rows {
        outputString.push_str(&format!("{} --> {}", row.start, row.next));
        if let Some(transitionText) = makeTransitionText(&row) {
            outputString.push_str(&transitionText);
        }
        outputString.push('\n');
    }
    outputString.push_str("@enduml\n");
    println!("{}", outputString);
    SUCCESSFUL_EXIT
}

#[derive(Debug)]
struct Row
{
    start: String,
    event: Option<String>,
    next: String,
    action: Option<String>,
    guard: Option<String>
}

impl Row
{
    fn new(sections: &Vec<&str>) -> Result<Self,()>
    {
        match sections.len() {
            n if n < 3 => { eprintln!("Too few sections in a row, expected at least 3, got: {}", n); return Err(()); },
            3 => { Ok(Self{
                    start: sections[0].into(),
                    event: getOpt(sections[1]),
                    next: getNextState(&sections),
                    action: None,
                    guard: None}) },
            4 => { Ok(Self{
                    start: sections[0].into(),
                    event: getOpt(sections[1]),
                    next: getNextState(&sections),
                    action: getOpt(sections[3]),
                    guard: None}) },
            5 => { Ok(Self{
                    start: sections[0].into(),
                    event: getOpt(sections[1]),
                    next: getNextState(&sections),
                    action: getOpt(sections[3]),
                    guard: getOpt(sections[4]) })},
            n => { eprintln!("Too many sections in a row, expected at most 5, got: {}", n); return Err(()); }
        }
    }
}

fn removePrefix(text: &str) -> &str
{
    match text.rfind(':') {
        Some(index) => &text[index+1..],
        None => text
    }
}

fn getOpt(section: &str) -> Option<String>
{
    match section {
        "none" => None,
        _ => Some(section.into())
    }
}

fn getNextState(sections: &[&str]) -> String
{
    match sections[2] {
        "none" => sections[0].into(),
        section => section.into()
    }
}

const TRANSITION_PREFIX: &str = " : ";

fn makeTransitionText(row: &Row) -> Option<String>
{
    let mut text = String::from(TRANSITION_PREFIX);
    if let Some(event) = &row.event {
        text.push_str(&format!("on {}", event));
    }
    if let Some(guard) = &row.guard {
        if text != TRANSITION_PREFIX {
            text.push_str("\\n");
        }
        text.push_str(&format!("if {}", guard));
    }
    if let Some(action) = &row.action {
        if text != TRANSITION_PREFIX {
            text.push_str("\\n");
        }
        text.push_str(&format!("do {}", action));
    }

    match text.as_str() {
        TRANSITION_PREFIX => None,
        _ => Some(text)
    }
}
