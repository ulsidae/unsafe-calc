mod ast;
mod error;
mod evaluator;
mod lexer;
mod parser;
mod unsafe_engine;

use wasm_bindgen::prelude::*;

use evaluator::{AngleMode, Evaluator};
use lexer::tokenize;
use parser::Parser;

#[wasm_bindgen]
pub struct Calculator {
    mode: AngleMode,
    ans: f64,
    memory: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        Calculator {
            mode: AngleMode::Deg,
            ans: 0.0,
            memory: 0.0,
        }
    }

    pub fn set_mode(&mut self, mode: &str) {
        self.mode = match mode.to_lowercase().as_str() {
            "rad" => AngleMode::Rad,
            _ => AngleMode::Deg,
        };
    }

    pub fn mode(&self) -> String {
        match self.mode {
            AngleMode::Deg => "DEG".into(),
            AngleMode::Rad => "RAD".into(),
        }
    }

    pub fn calculate(&mut self, input: &str) -> String {
        let input = input.replace("ANS", &self.ans.to_string());
        let input = input.replace("π", "pi");

        let result = match tokenize(&input)
            .and_then(|tokens| {
                let mut parser = Parser::new(tokens);
                parser.parse()
            })
            .and_then(|expr| {
                let evaluator = Evaluator::new(self.mode);
                evaluator.evaluate(&expr)
            }) {
                Ok(value) => {
                    self.ans = value;
                    format_number(value)
                }

                Err(error) => format!("Error: {error}"),
            };

        result
    }

    pub fn memory_add(&mut self) {
        self.memory += self.ans;
    }

    pub fn memory_sub(&mut self) {
        self.memory -= self.ans;
    }

    pub fn memory_recall(&self) -> String {
        format_number(self.memory)
    }

    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }
}

fn format_number(value: f64) -> String {
    if value.abs() >= 1e10 || (value != 0.0 && value.abs() < 1e-9) {
        format!("{value:.10e}")
    } else {
        let mut result = format!("{value:.12}");

        while result.contains('.') && result.ends_with('0') {
            result.pop();
        }

        if result.ends_with('.') {
            result.pop();
        }

        result
    }
}
