// src/completer.rs
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::History;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use crate::trie::Trie;


pub struct TrieCompleter {
    pub trie: Trie,
}

impl Completer for TrieCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let prefix = &line[..pos];
        let mut completions = self.trie.starts_with(prefix);
        completions.sort();
        // let internal_completions = 
        let pairs = completions
            .into_iter()
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: format!("{} ", cmd),
            })
            .collect();
        Ok((0, pairs))
    }
}

impl Helper for TrieCompleter {}
impl Hinter for TrieCompleter {
    type Hint = String;
}

impl Highlighter for TrieCompleter {}
impl Validator for TrieCompleter {}
