

#[derive(Debug)]
struct Word {
    lemma: String,
    gloss: String,
}

impl Word {
    fn to_string(&self) -> String {
        let s1 = self.lemma.clone();
        let s2 = self.gloss.clone();
        String::from("Lemma: ".to_owned() + &s1 + " ===> Gloss: " + &s2)
    }

    fn is_palindrome_iter(&self) -> bool {
        self.lemma.as_str().char_indices().filter(|&(_,c)| c.is_alphabetic())
            .zip(self.lemma.as_str().char_indices().rev().filter(|&(_,c)| c.is_alphabetic()))
            .take_while(|&((first_count, _), (second_count, _))| { first_count < second_count })
            .all(|((_, first_char), (_, second_char))| {
                first_char.to_ascii_lowercase() == second_char.to_ascii_lowercase()
            })
    }

    fn is_palindrome(&self) -> bool {
        let chars: Vec<char> = self.lemma.chars().collect();

        let mut first_idx = 0;
        let mut last_idx = chars.len() - 1;

        while first_idx != last_idx {
            if !chars[first_idx].is_alphabetic() { first_idx += 1; continue }
            if !chars[last_idx].is_alphabetic() { last_idx -= 1; continue }

            if chars[first_idx].to_ascii_lowercase() != chars[last_idx].to_ascii_lowercase() {
                return false;
            }

            first_idx += 1; last_idx -= 1;
        }

        return true;
    }
}

fn main() {
    let w = Word { lemma: String::from("Racecar"), gloss: String::from("German") };

    println!("Word is a palindrome? {}, {}", w.to_string(), w.is_palindrome_iter());
}
