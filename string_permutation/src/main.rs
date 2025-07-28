use string_permutation::*;

fn main() {
    let word = "p";
    let word1 = "a";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}