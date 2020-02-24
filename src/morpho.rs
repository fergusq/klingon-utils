use std::collections::{BTreeSet, HashSet};
use std::cmp::Ordering;

use crate::zrajm::{ZrajmPOS, ZrajmWord, ZrajmDictionary};

#[derive(Serialize)]
pub struct Completions {
    pub parsed: HashSet<Vec<BTreeSet<ZrajmWord>>>,
    pub suggestions: Vec<ZrajmWord>,
}

pub fn completions(dict: &ZrajmDictionary, word: &str) -> Completions {
    let grammar = grammar();
    let mut parsed_words = HashSet::new();
    let mut suggestion_words = HashSet::new();
    for mut g in grammar {
        let (ending, parsed, suggestions) = complete_grammar(&dict, &mut g.1, word);
        //println!(" {} {}", ending, suggestions.len());
        if !parsed.is_empty() && (ending.is_empty() || !suggestions.is_empty()) {
            parsed_words.insert(parsed);
        }
        if ending.is_empty() {
            continue
        }
        suggestion_words.extend(suggestions)
    }
    let mut suggestions = suggestion_words.into_iter().collect::<Vec<_>>();
    suggestions.sort_by(|a, b| {
        let asw = a.tlh.starts_with(word);
        let bsw = b.tlh.starts_with(word);
        if asw && !bsw {
            Ordering::Less
        } else if !asw && bsw {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
    Completions {
        parsed: parsed_words,
        suggestions,
    }
}

pub fn complete_grammar(dict: &ZrajmDictionary, grammar: &mut Vec<(ZrajmPOS, bool)>, word: &str) -> (String, Vec<BTreeSet<ZrajmWord>>, HashSet<ZrajmWord>) {
    let (ending, parsed, poses) = complete_pos(dict, Vec::new(), grammar, word);
    //println!("{:?} {:?}", ending, poses.len());
    let mut ans_parsed = Vec::new();
    for (word, pos) in parsed {
        ans_parsed.push(dict.pos_index.get(&pos).unwrap_or(&HashSet::new()).iter().filter(|w| w.tlh == word).cloned().collect());
    }
    let mut ans_words = HashSet::new();
    for pos in poses {
        for dword in dict.pos_index.get(&pos.0).map_or(Vec::new(), |a| a.iter().collect()) {
            if dword.tlh.starts_with(ending.as_str()) {
                ans_words.insert(dword.clone());
            }
        }
        if pos.1 {
            break
        }
    }
    (ending, ans_parsed, ans_words)
}

fn complete_pos(dict: &ZrajmDictionary, mut parsed: Vec<(String, ZrajmPOS)>, grammar: &mut Vec<(ZrajmPOS, bool)>, word: &str) -> (String, Vec<(String, ZrajmPOS)>, Vec<(ZrajmPOS, bool)>) {
    if grammar.is_empty() {
        return (String::from(word), parsed, grammar.clone())
    }
    let mut dwords = Vec::new();
    if let Some(index_words) = dict.pos_index.get(&grammar[0].0) {
        dwords.extend(index_words);
    }
    dwords.sort_by(|a, b| b.tlh.len().cmp(&a.tlh.len()));
    for dword in dwords {
        let dword_ending = dword.tlh.trim_end_matches("-");
        // Valitaan ahneesti
        if word == dword_ending {
            return (String::from(word), parsed, grammar.clone())
        }
        if word.starts_with(dword_ending) {
            let t = if dword.tlh.ends_with("-") {
                ""
            } else {
                "-"
            };
            let ending = format!("{}{}", t, &word[dword_ending.len()..]);
            let pos = grammar.remove(0);
            parsed.push((dword.tlh.clone(), pos.0));
            return complete_pos(dict, parsed, grammar, &ending)
        }
    }
    if grammar[0].1 {
        return (String::from(word), parsed, grammar.clone())
    }
    let mut new_grammar = grammar.clone();
    new_grammar.remove(0);
    let (word2, parsed2, grammar2) = complete_pos(dict, parsed.clone(), &mut new_grammar, word);
    if word == word2 {
        (String::from(word), parsed, grammar.clone())
    } else {
        (word2, parsed2, grammar2)
    }
}

fn grammar() -> Vec<(&'static str, Vec<(ZrajmPOS, bool)>)> {
    vec![
        ("Noun track", vec![
            (ZrajmPOS::Noun, true),
            (ZrajmPOS::NounSuffix1, false),
            (ZrajmPOS::NounSuffix2, false),
            (ZrajmPOS::NounSuffix3, false),
            (ZrajmPOS::NounSuffix4, false),
            (ZrajmPOS::NounSuffix5, false),
        ]),
        ("Verb track", vec![
            (ZrajmPOS::VerbPrefix, false),
            (ZrajmPOS::Verb, true),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix1, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix2, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix3, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix4, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix5, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix6, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix7, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix8, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix9, false),
            (ZrajmPOS::VerbSuffixRover, false),
        ]),
        ("Nominalized verb track", vec![
            (ZrajmPOS::Verb, true),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix1, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix2, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix3, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix4, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix5, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix6, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix7, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix8, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix9, true),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::NounSuffix1, false),
            (ZrajmPOS::NounSuffix2, false),
            (ZrajmPOS::NounSuffix3, false),
            (ZrajmPOS::NounSuffix4, false),
            (ZrajmPOS::NounSuffix5, false),
        ]),
        ("Adjective track", vec![
            (ZrajmPOS::Verb, true),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::NounSuffix5, false),
        ]),
        ("Pronoun track (verb)", vec![
            (ZrajmPOS::Pronoun, true),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix1, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix2, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix3, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix4, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix5, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix6, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix7, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix8, false),
            (ZrajmPOS::VerbSuffixRover, false),
            (ZrajmPOS::VerbSuffix9, true),
            (ZrajmPOS::VerbSuffixRover, false),
        ]),
        ("Pronoun track (noun)", vec![
            (ZrajmPOS::Pronoun, true),
            (ZrajmPOS::NounSuffix5, false),
        ]),
        ("Numerals", vec![(ZrajmPOS::Numeral, true)]),
        ("Adverbials", vec![(ZrajmPOS::Adverbial, true)]),
        ("Conjunctions", vec![(ZrajmPOS::Conjunction, true)]),
        ("Question words", vec![(ZrajmPOS::QuestionWord, true)]),
    ]
}