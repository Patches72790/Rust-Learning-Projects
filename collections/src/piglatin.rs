
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

/**
 * Converts string to pig latin.
 *
 * For a word w as input, it appends the first syllable of the word
 * to the end with the suffix 'ay' attached.
 *
 */ 
pub fn piglatinify(w: String) -> String {
    let mut c_iter = w.chars();
    let first = c_iter.next();
    let first = match first {
        Some(i) => i,
        None => panic!("Cannot piglatinify that char!")
    };

    let res: String = match VOWELS.iter().find(|c| **c == first) {
        Some(_) => w.clone() + "-hay",
        None => c_iter.collect::<String>() + "-" + &first.to_string() + "ay"
    };

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piglatinify_1() {
        assert_eq!("ear-day".to_string(), piglatinify("dear".to_string()));
    }
    #[test]
    fn test_piglatinify_2() {
        assert_eq!("abc-hay".to_string(), piglatinify("abc".to_string()));
    }

    #[test]
    fn test_piglatinify_3() {
        assert_eq!("apple-hay".to_string(), piglatinify("apple".to_string()));
    }
}
