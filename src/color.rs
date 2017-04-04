use rand::{self, Rng};

use std::collections::{HashSet, HashMap};

use brdgme_color;

#[derive(Debug, ToSql, FromSql, PartialEq, Eq, Hash, Clone, Copy)]
#[postgres(name = "color")]
pub enum Color {
    Green,
    Red,
    Blue,
    Amber,
    Purple,
    Brown,
    BlueGrey,
}

pub static COLORS: &'static [Color] = &[
    Color::Green,
    Color::Red,
    Color::Blue,
    Color::Amber,
    Color::Purple,
    Color::Brown,
    Color::BlueGrey,
];

impl Into<brdgme_color::Color> for Color {
    fn into(self) -> brdgme_color::Color {
        match self {
            Color::Green => brdgme_color::GREEN,
            Color::Red => brdgme_color::RED,
            Color::Blue => brdgme_color::BLUE,
            Color::Amber => brdgme_color::AMBER,
            Color::Purple => brdgme_color::PURPLE,
            Color::Brown => brdgme_color::BROWN,
            Color::BlueGrey => brdgme_color::BLUE_GREY,
        }
    }
}

type LocPref = (usize, Vec<Color>);

pub fn choose(available: &HashSet<&Color>, prefs: &[Vec<Color>]) -> Vec<Color> {
    if available.is_empty() || prefs.is_empty() {
        return vec![];
    }
    let mut rng = rand::thread_rng();
    let mut remaining = available.clone();
    let mut assigned: HashMap<usize, Color> = HashMap::new();
    let mut rem_prefs = filter_prefs(&remaining,
                                     &prefs
                                          .iter()
                                          .enumerate()
                                          .map(|(l, pref)| (l, pref.clone()))
                                          .collect::<Vec<LocPref>>());
    rng.shuffle(&mut rem_prefs);
    'outer: loop {
        'inner: for &(pos, ref pref) in rem_prefs.clone().iter() {
            if assigned.contains_key(&pos) || pref.is_empty() {
                continue 'inner;
            }
            assigned.insert(pos, pref[0]);
            remaining.remove(&pref[0]);
            if remaining.is_empty() {
                // No colors left
                break 'outer;
            }
            rem_prefs = filter_prefs(&remaining, &rem_prefs);
        }
        // No more preferences, exit
        break 'outer;
    }
    let mut left = remaining.drain();
    let mut res = vec![];
    for p in 0..prefs.len() {
        res.push(assigned
                     .get(&p)
                     .cloned()
                     .unwrap_or_else(|| left.next().cloned().unwrap_or_else(|| Color::Green))
                     .to_owned());
    }
    res
}

fn filter_prefs(available: &HashSet<&Color>, prefs: &[LocPref]) -> Vec<LocPref> {
    prefs
        .iter()
        .map(|&(p, ref pref)| {
                 (p,
                  pref.iter()
                      .filter(|c| available.contains(c))
                      .cloned()
                      .collect())
             })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choose_works() {
        use std::iter::FromIterator;
        assert_eq!(vec![Color::Amber, Color::Blue, Color::Green],
                   choose(&HashSet::from_iter(vec![Color::Amber, Color::Blue, Color::Green]
                                                  .iter()),
                          &[vec![], vec![Color::Blue, Color::Green], vec![Color::Green]]));
    }
}
