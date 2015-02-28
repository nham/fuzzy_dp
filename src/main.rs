fn main() {
    println!("{}", levenshtein("aÖbc", "aÖbc"));
    println!("{}", levenshtein("abcö", "abcü"));
    println!("{}", levenshtein("annually", "annealing"));
}

fn levenshtein<'a, 'b>(s: &'a str, t: &'b str) -> usize {
    let mut s_iter = s.char_indices();
    let mut t_iter = t.char_indices();

    match (s_iter.next_back(), t_iter.next_back()) {
        (None, _) => t.chars().count(),
        (_, None) => s.chars().count(),
        (Some((i, si)), Some((j, tj))) => {
            let mut v = vec![];
            v.push(levenshtein(&s[..i], &t[..j]) + if si == tj {0} else {1});
            v.push(levenshtein(&s[..i], t) + 1);
            v.push(levenshtein(s, &t[..j]) + 1);
            v.into_iter().min().unwrap()
        }
    }
}

