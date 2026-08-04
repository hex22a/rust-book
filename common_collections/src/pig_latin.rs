const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn convert(word: &mut String) {
    let first_char = word.chars().next().unwrap();
    if VOWELS.contains(&first_char) {
        word.push_str("-hey");
    } else {
        word.remove(0);
        word.push_str(&format!("-{}ey", &first_char));
    }
}

pub fn run() {
    let mut words = vec![String::from("apple"), String::from("first")];
    for w in words.iter_mut() {
        convert(w);
        println!("{}", w);
    }
}
