use std::collections::HashMap;
use std::hash::Hash;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TMParser;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Transition {
    write: char,
    move_to: char,
    next_state: String,
}

fn cut_tape(tape: &mut Vec<char>) {
    // Cuts away all trailing and leading &'s in the tape
    while tape.last() == Some(&'&') {
        tape.pop();
    }

    while tape.first() == Some(&'&') {
        tape.remove(0);
    }
}

impl Transition {
    pub fn new(write: char, mut move_to: char, next_state: String) -> Self {
        match move_to {
            'L' | 'R' | 'S' | '_' | '.' | '<' | '>' => { },
            _ => {
                panic!("Invalid move_to value: must be one of L, R, S, _, ., <, >");
            }
        }

        match move_to {
            '<' => {
                move_to = 'L';
            }

            '>' => {
                move_to = 'R';
            }

            '_' => {
                move_to = 'S';
            }

            '.' => {
                move_to = 'S';
            }

            _ => { },
        }

        Self {
            write,
            move_to,
            next_state,
        }
    }
}

#[derive(Debug)]
pub struct TMInterpreter {
    tape: Vec<char>,
    expected_output: Vec<char>,
    caret_position: usize,
    overflow_counter: i32,
    transitions: HashMap<String, HashMap<String, Transition>>,
    debug: bool,
}

impl TMInterpreter {
    pub fn new(tape: Vec<char>, expected_output: Vec<char>, caret_position: usize, overflow_counter: i32, transitions: HashMap<String, HashMap<String, Transition>>, debug: bool) -> Self {
        Self {
            tape,
            expected_output,
            caret_position,
            overflow_counter,
            transitions,
            debug,
        }
    }

    pub fn run(&self) {
        let mut state = &"q1".to_string();
        let mut tape = self.tape.clone();
        let mut caret_position = self.caret_position;
        let mut overflow_counter = 0;
        let mut prev_transition = &Transition {
            write: ' ',
            move_to: ' ',
            next_state: String::new(),
        };
        let mut step_counter = 0;

        loop {
            if self.overflow_counter != 0 && overflow_counter >= self.overflow_counter {
                let mut output = String::new();

                for (i, mut symbol) in tape.iter().enumerate() {
                    if symbol == &'&' {
                        symbol = &' ';
                    }

                    if i == caret_position {
                        output.push_str(&format!("[{}]", symbol));
                    }
                    else {
                        output.push(*symbol);
                    }
                }

                panic!("Overflowed! Tape: {:?}", output);
            }

            let symbol = &tape[caret_position].to_string();

            let transition = &self.transitions[symbol][state];

            if transition.write != '_' {
                tape[caret_position] = transition.write;
            }

            match transition.move_to {
                'L' => {
                    if caret_position == 0 {
                        tape.insert(0, '&');
                    }
                    else {
                        caret_position -= 1;
                    }
                }

                'R' => {
                    if caret_position == tape.len() - 1 {
                        tape.push('&');
                    }

                    caret_position += 1;
                }

                'S' => { },

                _ => unreachable!(),
            }

            if transition.next_state != "e" && transition.next_state != "_" {
                state = &transition.next_state;
            }
            else if transition.next_state == "e" {
                cut_tape(&mut tape);

                if &tape.len() > &caret_position {
                    caret_position = tape.len() - 1;
                }

                if &self.expected_output[0] != &'~' {
                    if &tape != &self.expected_output {
                        let mut output = String::new();

                        for (i, mut symbol) in tape.iter().enumerate() {
                            if symbol == &'&' {
                                symbol = &' ';
                            }

                            if i == caret_position {
                                output.push_str(&format!("[{}]", symbol));
                            }
                            else {
                                output.push(*symbol);
                            }
                        }

                        panic!("Tape does not match expected output! Expected: {:?}, got: {:?}", self.expected_output, tape);
                    }

                    println!("Tape matches expected output: {:?}", tape);
                    return;
                }

                let mut output = String::new();

                for (i, mut symbol) in tape.iter().enumerate() {
                    if symbol == &'&' {
                        symbol = &' ';
                    }

                    if i == caret_position {
                        output.push_str(&format!("[{}]", symbol));
                    }
                    else {
                        output.push(*symbol);
                    }
                }

                println!("{}", output);
                return;
            }

            else {

            }

            if prev_transition == transition {
                overflow_counter += 1;
            }
            else {
                overflow_counter = 0;
            }

            prev_transition = transition;

            if self.debug {
                let mut output = String::new();

                for (i, mut symbol) in tape.iter().enumerate() {
                    if symbol == &'&' {
                        symbol = &' ';
                    }

                    if i == caret_position {
                        output.push_str(&format!("[{}]", symbol));
                    }
                    else {
                        output.push(*symbol);
                    }
                }
                step_counter += 1;

                println!("({}) {} | {}", step_counter, state, output);
            }
        }
    }
}

fn main() {
    let file_path = "G:\\Progs\\turinger\\src\\example.tur";
    let file_data = std::fs::read_to_string(file_path).unwrap();

    let data = TMParser::parse(Rule::file, &file_data).unwrap_or_else(|e| panic!("{}", e));

    let mut alphabet = Vec::new();
    let mut alphabet_header = Vec::new();
    let mut states = Vec::new();
    let mut tape = Vec::new();
    let mut expected_output = Vec::new();
    let mut caret_position = 0;
    let mut overflow_counter = 0;
    let mut transitions = HashMap::new();

    expected_output.push('~');

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
                                if !alphabet.contains(&tape_pair.as_str().chars().next().unwrap()) {
                                    let mut loc_alphabet = String::new();

                                    for symbol in alphabet.iter() {
                                        loc_alphabet.push_str(&format!("{} ", symbol));
                                    }

                                    panic!("Symbol {} is not in the alphabet: must be one of {}", tape_pair.as_str().chars().next().unwrap(), loc_alphabet);
                                }

                                tape.push(tape_pair.as_str().chars().next().unwrap());
                            }

                            Rule::lead_symbol => {
                                if !alphabet.contains(&tape_pair.clone().into_inner().as_str().chars().next().unwrap()) {
                                    let mut loc_alphabet = String::new();

                                    for symbol in alphabet.iter() {
                                        loc_alphabet.push_str(&format!("{} ", symbol));
                                    }

                                    panic!("Symbol {} is not in the alphabet: must be one of {}", tape_pair.into_inner().as_str().chars().next().unwrap(), loc_alphabet);
                                }

                                tape.push(tape_pair.into_inner().as_str().chars().next().unwrap());
                                if caret_position == 0 {
                                    caret_position = tape.len() as i32 - 1;
                                }
                                else {
                                    panic!("Caret position already defined");
                                }
                            }

                            _ => unreachable!(),
                        }
                    }
                }

                Rule::expect_definition => {
                    if '~' == expected_output[0] {
                        expected_output.clear();
                    }

                    for output_pair in inner_pair.into_inner() {
                        match output_pair.as_rule() {
                            Rule::symbol => {
                                expected_output.push(output_pair.as_str().chars().next().unwrap());
                            }

                            _ => unreachable!(),
                        }
                    }
                }

                Rule::overflow_definition => {
                    for overflow_pair in inner_pair.into_inner() {
                        match overflow_pair.as_rule() {
                            Rule::number => {
                                overflow_counter = overflow_pair.as_str().parse::<i32>().unwrap();
                            }

                            _ => unreachable!(),
                        }
                    }
                }

                Rule::state_table => {
                    for state_table_pair in inner_pair.into_inner() {
                        match state_table_pair.as_rule() {
                            Rule::state_table_header => {
                                for state_table_header_pair in state_table_pair.into_inner() {
                                    match state_table_header_pair.as_rule() {
                                        Rule::column_header => {
                                            let symbol = state_table_header_pair.into_inner().as_str().to_string();

                                            if transitions.contains_key(&symbol) {
                                                panic!("Symbol {} already defined", symbol);
                                            }

                                            if !alphabet.contains(&symbol.chars().next().unwrap()) && symbol != "&" {
                                                let mut loc_alphabet = String::new();

                                                for symbol in alphabet.iter() {
                                                    loc_alphabet.push_str(&format!("{} ", symbol));
                                                }

                                                panic!("Symbol {} is not in the alphabet: must be one of {}", symbol, alphabet.iter().collect::<String>());
                                            }

                                            transitions.insert(symbol.clone(), HashMap::new());
                                            alphabet_header.push(symbol);
                                        }

                                        Rule::root => { },

                                        _ => {
                                            panic!("Unexpected element of header: {:?}", state_table_header_pair);
                                        },
                                    }
                                }
                            }

                            Rule::state_table_row => {
                                let mut state = String::new();

                                for (i, state_table_row_pair) in state_table_pair.into_inner().enumerate() {
                                    match state_table_row_pair.as_rule() {
                                        Rule::state => {
                                            state = state_table_row_pair.as_str().to_string();
                                        }

                                        Rule::action => {
                                            let mut action = Transition {
                                                write: ' ',
                                                move_to: ' ',
                                                next_state: String::new(),
                                            };

                                            for action_pair in state_table_row_pair.into_inner() {
                                                match action_pair.as_rule() {
                                                    Rule::symbol => {
                                                        action.write = action_pair.as_str().chars().next().unwrap();
                                                    }

                                                    Rule::direction => {
                                                        action.move_to = action_pair.as_str().chars().next().unwrap();
                                                    }

                                                    Rule::state => {
                                                        let mut inner_state = action_pair.as_str().to_string();

                                                        if inner_state.chars().nth(0usize).unwrap() != 'q' && inner_state != "e" && inner_state != "_" {
                                                            inner_state = format!("q{}", inner_state);
                                                        }

                                                        if !states.contains(&inner_state) && inner_state != "e" && inner_state != "_" {
                                                            // Get states list as a space-separated string
                                                            let mut loc_states = String::new();

                                                            for each_state in states.iter() {
                                                                loc_states.push_str(&format!("{} ", each_state));
                                                            }

                                                            panic!("State {} is undefined: must be one of {}", inner_state, loc_states);
                                                        }

                                                        action.next_state = inner_state;
                                                    }

                                                    _ => unreachable!(),
                                                }
                                            }

                                            transitions.get_mut(&alphabet_header[i - 1].to_string()).unwrap().insert(state.clone(), action);
                                        }

                                        _ => unreachable!(),
                                    }
                                }
                            }

                            _ => unreachable!(),
                        }
                    }
                },

                _ => {
                    panic!("Unhandled row: {:?}", inner_pair);
                }
            }
        }
    }

    let tm = TMInterpreter {
        tape,
        expected_output,
        caret_position: caret_position as usize,
        overflow_counter,
        transitions,
        debug: false,
    };

    tm.run();
}
