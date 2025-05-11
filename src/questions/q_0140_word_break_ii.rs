use std::collections::HashSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let word_dict: HashSet<String> = word_dict.into_iter().collect();

    fn backtrack(
        index: usize,
        s: &String,
        word_dict: &HashSet<String>,
        current: &mut Vec<String>,
        result: &mut Vec<String>,
    ) {
        if index == s.len() {
            result.push(current.join(" "));
            return;
        }
        for j in index..s.len() {
            let word: String = s.chars().skip(index).take(j - index + 1).collect();
            if word_dict.contains(&word) {
                current.push(word);
                backtrack(j + 1, s, word_dict, current, result);
                current.pop();
            }
        }
    }

    let mut current = vec![];
    let mut result = vec![];
    backtrack(0, &s, &word_dict, &mut current, &mut result);
    result
}

#[test]
fn test_word_break() {
    let s = "catsanddog".to_string();
    let word_dict = vec![
        "cat".to_string(),
        "cats".to_string(),
        "and".to_string(),
        "sand".to_string(),
        "dog".to_string(),
    ];
    let mut result = word_break(s, word_dict);
    result.sort();
    assert_eq!(result, vec!["cat sand dog", "cats and dog"]);
}
