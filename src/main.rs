use std::ops::{Index, IndexMut};
use std::usize;

fn main() {
    println!("{}", levenshtein("", ""));
    println!("{}", levenshtein("", "a"));
    println!("{}", levenshtein("a", "a"));
    println!("{}", levenshtein("aÖbc", "aÖbc"));
    println!("{}", levenshtein("abcö", "abcü"));
    println!("{}", levenshtein("annually", "annealing"));
    println!("{}", levenshtein("0123456789 0123456789 0123456789 0123456789",
                               "0123456789 0a2345679  023456789 012233456789"));

    println!("{}", fuzzy_contains("nana", "bananas", 0));
    println!("{}", fuzzy_contains("I", "team", 0));
    println!("{}", fuzzy_contains("bot", "zzzrocky bolboa", 1));
    println!("{}", fuzzy_contains("zot", "zzzrocky bolboa", 1));
    println!("{}", fuzzy_contains("annually", "simulated annealing", 2));
    println!("{}", fuzzy_contains("annually", "simulated annealing", 3));
}

struct MemoMatrix<T> {
    rows: usize,
    cols: usize,
    v: Vec<Option<T>>,
}

impl<T> MemoMatrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        let mut v = Vec::with_capacity(rows*cols);
        for _ in 0..(rows*cols) {
            v.push(None);
        }
        MemoMatrix { rows: rows, cols: cols, v: v }
    }
}

impl<T> Index<(usize, usize)> for MemoMatrix<T> {
    type Output = Option<T>;
    fn index<'a>(&'a self, index: &(usize, usize)) -> &'a Option<T> {
        let (r, c) = *index;
        assert!(r < self.rows, "row index '{}' is out of bounds", r);
        assert!(c < self.cols, "column index '{}' is out of bounds", c);
        &self.v[r * self.cols + c]
    }
}

impl<T> IndexMut<(usize, usize)> for MemoMatrix<T> {
    fn index_mut<'a>(&'a mut self, index: &(usize, usize)) -> &'a mut Option<T> {
        let (r, c) = *index;
        assert!(r < self.rows, "row index '{}' is out of bounds", r);
        assert!(c < self.cols, "column index '{}' is out of bounds", c);
        &mut self.v[r * self.cols + c]
    }
}


fn levenshtein<'a, 'b>(s: &'a str, t: &'b str) -> usize {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // M(i, j) is the minimal cost of an edit sequence that turns s[..i] into t[..j]
    let mut rect = MemoMatrix::new(s.chars().count() + 1, t.chars().count() + 1);
    lev_rec(&mut rect, &s_chars[], &t_chars[])
}

fn lev_rec<'a, 'b>(rect: &mut MemoMatrix<usize>, s: &'a [char], t: &'b [char]) -> usize {
    // check if this has already been computed
    let (i, j) = (s.len(), t.len());
    match rect[(i, j)] {
        Some(dist) => return dist,
        None => {},
    }

    let dist = if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        if s[a] == t[b] {
            // It can't be the case that M(i-1, j-1) > M(i-1, j) + 1 or M(i, j-1) + 1
            lev_rec(rect, &s[..a], &t[..b])
        } else {
            let mut v = vec![];
            v.push(lev_rec(rect, &s[..a], &t[..b]));
            v.push(lev_rec(rect, &s[..a], t));
            v.push(lev_rec(rect, s, &t[..b]));
            v.into_iter().min().unwrap() + 1
        }
    };

    rect[(i, j)] = Some(dist);
    dist
}

fn fuzzy_contains<'a, 'b>(p: &'a str, t: &'b str, k: usize) -> bool {
    lev_substring(p, t) <= k
}

// returns the minimum edit distance between `p` and all substrings of `t`
fn lev_substring<'a, 'b>(p: &'a str, t: &'b str) -> usize {
    let p_chars: Vec<char> = p.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // M(i, j) is the minimal cost of an edit sequence that turns p[..i] into t[..j]
    let n = t.chars().count();
    let mut rect = MemoMatrix::new(p.chars().count() + 1, n + 1);

    let mut min = usize::MAX;
    for k in 0..(n+1) {
        let dist = lev_sub_rec(&mut rect, &p_chars[], &t_chars[..k]);
        if dist < min {
            min = dist;
        }
    }

    min
}

// the difference from lev_rec is M(0, j) = 0 for all j to ensure that there is no cost to
// skipping ahead in the text string t
fn lev_sub_rec<'a, 'b>(rect: &mut MemoMatrix<usize>, p: &'a [char], t: &'b [char]) -> usize {
    // check if this has already been computed
    let (i, j) = (p.len(), t.len());
    match rect[(i, j)] {
        Some(dist) => return dist,
        None => {},
    }

    let dist = if i == 0 {
        0
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        if p[a] == t[b] {
            // It can't be the case that M(i-1, j-1) > M(i-1, j) + 1 or M(i, j-1) + 1
            lev_sub_rec(rect, &p[..a], &t[..b])
        } else {
            let mut v = vec![];
            v.push(lev_sub_rec(rect, &p[..a], &t[..b]));
            v.push(lev_sub_rec(rect, &p[..a], t));
            v.push(lev_sub_rec(rect, p, &t[..b]));
            v.into_iter().min().unwrap() + 1
        }
    };

    rect[(i, j)] = Some(dist);
    dist
}
