use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TMParser;

pub struct Transition {
    write: char,
    move_to: char,
    next_state: String,
}

#[derive(Debug)]
pub struct TMInterpreter {
    alphabet: Vec<char>,
    states: Vec<String>,
    tape: Vec<char>,
    expected_output: Option<Vec<char>>,
    // TODO: transitions: Vec<Transition>,
}

fn main() {
    let file_path = "G:\\Progs\\turinger\\src\\example.tur";
    let file_data = std::fs::read_to_string(file_path).unwrap();

    let data = TMParser::parse(Rule::file, &file_data).unwrap_or_else(|e| panic!("{}", e));

    let mut alphabet = Vec::new();
    let mut states = Vec::new();
    let mut tape = Vec::new();
    let mut expected_output = Vec::new();

    for pair in data {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alphabet_declaration => {
                    for alphabet_pair in inner_pair.into_inner() {
                        match alphabet_pair.as_rule() {
                            Rule::symbol => {
                                alphabet.push(alphabet_pair.as_str().chars().next().unwrap());
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                Rule::states_declaration => {
                    for state_pair in inner_pair.into_inner() {
                        match state_pair.as_rule() {
                            Rule::state => {
                                states.push(state_pair.as_str().to_string());
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                Rule::tape_definition => {
                    for tape_pair in inner_pair.into_inner() {
                        match tape_pair.as_rule() {
                            Rule::symbol => {
                                tape.push(tape_pair.as_str().chars().next().unwrap());
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                Rule::expect_definition => {
                    for output_pair in inner_pair.into_inner() {
                        match output_pair.as_rule() {
                            Rule::symbol => {
                                expected_output.push(output_pair.as_str().chars().next().unwrap());
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                _ => { },
            }
        }
    }

    let tm = TMInterpreter {
        alphabet: alphabet.clone(),
        states: states.clone(),
        tape: tape.clone(),
        expected_output: Some(expected_output),
    };

    println!("{:?}, {:?}, {:?}", &alphabet, &states, &tape);
}
