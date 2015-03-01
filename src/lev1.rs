fn main() {
    println!("{}", lev("tyrannosaurus rex", "oedipus rex"));
}

fn lev<'a, 'b>(s: &'a str, t: &'b str) -> usize {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    ed(&s_chars[], &t_chars[])
}

fn ed<'a, 'b>(s: &'a [char], t: &'b [char]) -> usize {
    let (i, j) = (s.len(), t.len());
    if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        let v = vec![
            ed(&s[..a], &t[..b]) + if s[a] == t[b] { 0 } else { 1 },
            ed(&s[..a], t) + 1,
            ed(s, &t[..b]) + 1
        ];
        v.into_iter().min().unwrap()
    }
}
