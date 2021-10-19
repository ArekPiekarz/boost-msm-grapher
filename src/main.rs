#![allow(non_snake_case)]

mod character_reader;
mod flow;
mod row;
mod row_section_parser;
mod token;
mod transition_table_lexer;
mod transition_table_parser;

use crate::character_reader::CharacterReader;
use crate::row::Row;
use crate::transition_table_lexer::lexTransitionTable;
use crate::transition_table_parser::parseTransitionTable;

use regex::Regex;
use std::path::PathBuf;


fn main() -> Result<(),String>
{
    let args: Vec<String> = std::env::args().collect();
    let filePath = match args.len() {
        0 => return Err("Unexpected no arguments passed to program.".into()),
        1 => return Err("Please provide a path to a file to analyze.".into()),
        2 => PathBuf::from(&args[1]),
        n => return Err(format!("Too many arguments passed to program, expected only one with a file path, got {}", n-1))
    };

    let fileContent = match std::fs::read_to_string(&filePath) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file: {:?}, error: {}", filePath, e))
    };

    let startOfTransitionTable = match findStartOfTransitionTable(&fileContent) {
        Some(start) => start,
        None => return Err("Transition table was not found.".into())
    };

    let characterReader = CharacterReader::new(&fileContent[startOfTransitionTable..]);
    let tokens = lexTransitionTable(characterReader);
    let rows = match parseTransitionTable(&tokens) {
        Ok(rows) => rows,
        Err(e) => return Err(e)
    };

    let mut outputString = format!("@startuml\nhide empty description\n[*] --> {}\n", rows[0].start);
    for row in rows {
        outputString.push_str(&format!("{} --> {}", row.start, row.target));
        if let Some(transitionText) = makeTransitionText(&row) {
            outputString.push_str(&transitionText);
        }
        outputString.push('\n');
    }
    outputString.push_str("@enduml");
    println!("{}", outputString);
    Ok(())
}

fn findStartOfTransitionTable(text: &str) -> Option<usize>
{
    if text.is_empty() {
        return None;
    }
    let transitionTableRegex = Regex::new(r"^\s*struct transition_table\b").unwrap();
    let mut startIndex = 0;
    loop {
        let endIndex = match text[startIndex..].find('\n') {
            Some(index) => startIndex + index,
            None => {
                if text.len() - 1 > startIndex {
                    text.len()
                } else {
                    return None;
                }
            }
        };
        let textSlice = &text[startIndex..endIndex];
        if transitionTableRegex.is_match(textSlice) {
            return Some(startIndex);
        }
        startIndex = endIndex + 1;
        if endIndex >= text.len() {
            return None;
        }
    }
}

const TRANSITION_PREFIX: &str = " : ";

fn makeTransitionText(row: &Row) -> Option<String>
{
    let mut text = String::from(TRANSITION_PREFIX);
    if shouldBeShown(&row.event) {
        text.push_str(&format!("on {}", row.event));
    }

    if shouldBeShown(&row.guard) {
        addNewLineIfNeeded(&mut text);
        text.push_str(&format!("if {}", row.guard));
    }

    if shouldBeShown(&row.action) {
        addNewLineIfNeeded(&mut text);
        text.push_str(&format!("do {}", row.action));
    }

    match text.as_str() {
        TRANSITION_PREFIX => None,
        _ => Some(text)
    }
}

fn shouldBeShown(name: &str) -> bool
{
    !matches!(name, "" | "None" | "none" | "front::none" | "msm::front::none" | "boost::msm::front::none")
}

fn addNewLineIfNeeded(text: &mut String)
{
    if text != TRANSITION_PREFIX {
        text.push_str("\\n");
    }
}
