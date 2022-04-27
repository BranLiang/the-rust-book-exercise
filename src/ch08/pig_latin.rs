fn pig_latin(sentence: &String) -> String {
    let vowels = vec!['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
    
    let mut result = Vec::new();
    let words = sentence.split_whitespace();
    for word in words {
        if word.starts_with(|c| vowels.contains(&c)) {
            let mut pig_latin_word = String::new();
            pig_latin_word.push_str(word.clone());
            pig_latin_word.push_str("hay");
            result.push(pig_latin_word);
        } else {
            let mut pig_latin_word = String::new();
            let mut chars = word.clone().chars();
            let first_char = chars.next().unwrap();
            for c in chars {
                pig_latin_word.push(c)
            }
            pig_latin_word.push(first_char);
            pig_latin_word.push_str("ay");
            result.push(pig_latin_word);
        }
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_latin() {
        let test_sentence = String::from("pig latin apple");
        assert_eq!(pig_latin(&test_sentence), "igpay atinlay applehay")
    }
}