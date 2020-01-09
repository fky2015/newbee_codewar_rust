// https://www.codewars.com/kata/5a420163b6cfd7cde5000077/train/rust
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Default, Debug)]
struct Team {
    name: String,
    win: u32,
    draw: u32,
    lose: u32,
    scored: u32,
    conceded: u32,
    marks: u32,
}

impl Team {
    fn new(name: &str) -> Team {
        Team {
            name: name.to_string(),
            ..Default::default()
        }
    }
    fn record_a_match(&mut self, s1: u32, s2: u32) {
        self.conceded += s2;
        self.scored += s1;
        match s1.cmp(&s2) {
            Ordering::Greater => {
                self.win += 1;
                self.marks += 3;
            }
            Ordering::Equal => {
                self.draw += 1;
                self.marks += 1;
            }
            Ordering::Less => self.lose += 1,
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}:W={};D={};L={};Scored={};Conceded={};Points={}",
            self.name, self.win, self.draw, self.lose, self.scored, self.conceded, self.marks
        )
    }
}

fn nba_cup(ro: &str, to_find: &str) -> String {
    if to_find.is_empty() {
        return "".to_string();
    }
    let mut hmap: HashMap<&str, Team> = HashMap::new();
    // your code
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<p1>(\w+\s)+\w+)\s(?P<s1>\d+)\s(?P<p2>(\w+\s)+\w+)\s(?P<s2>\d+)")
                .unwrap();
    }
    for row in ro.split(',') {
        match RE.captures(row) {
            Some(cap) => {
                let p1 = cap.name("p1").unwrap().as_str();
                let p2 = cap.name("p2").unwrap().as_str();
                let s1: u32 = cap.name("s1").unwrap().as_str().parse().unwrap();
                let s2: u32 = cap.name("s2").unwrap().as_str().parse().unwrap();

                hmap.entry(p1)
                    .or_insert_with(|| Team::new(p1))
                    .record_a_match(s1, s2);
                hmap.entry(p2)
                    .or_insert_with(|| Team::new(p2))
                    .record_a_match(s2, s1);
            }
            None => {
                if row.contains(to_find) {
                    return format!("Error(float number):{}", row);
                }
            }
        }
    }
    match hmap.get(to_find) {
        Some(team) => team.to_string(),
        None => format!("{}:This team didn't play!", to_find),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, to_find: &str, exp: &str) -> () {
        println!("to_find:{:?}", to_find);
        let ans = nba_cup(s, to_find);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("-");
    }

    fn ro() -> String {
        return String::from("Los Angeles Clippers 104 Dallas Mavericks 88,New York Knicks 101 Atlanta Hawks 112,Indiana Pacers 103 Memphis Grizzlies 112,\
    Los Angeles Lakers 111 Minnesota Timberwolves 112,Phoenix Suns 95 Dallas Mavericks 111,Portland Trail Blazers 112 New Orleans Pelicans 94,\
    Sacramento Kings 104 Los Angeles Clippers 111,Houston Rockets 85 Denver Nuggets 105,Memphis Grizzlies 76 Cleveland Cavaliers 106,\
    Milwaukee Bucks 97 New York Knicks 122,Oklahoma City Thunder 112 San Antonio Spurs 106,Boston Celtics 112 Philadelphia 76ers 95,\
    Brooklyn Nets 100 Chicago Bulls 115,Detroit Pistons 92 Utah Jazz 87,Miami Heat 104 Charlotte Hornets 94,\
    Toronto Raptors 106 Indiana Pacers 99,Orlando Magic 87 Washington Wizards 88,Golden State Warriors 111 New Orleans Pelicans 95,\
    Atlanta Hawks 94 Detroit Pistons 106,Chicago Bulls 97 Cleveland Cavaliers 95,\
    San Antonio Spurs 111 Houston Rockets 86,Chicago Bulls 103 Dallas Mavericks 102,Minnesota Timberwolves 112 Milwaukee Bucks 108,\
    New Orleans Pelicans 93 Miami Heat 90,Boston Celtics 81 Philadelphia 76ers 65,Detroit Pistons 115 Atlanta Hawks 87,\
    Toronto Raptors 92 Washington Wizards 82,Orlando Magic 86 Memphis Grizzlies 76,Los Angeles Clippers 115 Portland Trail Blazers 109,\
    Los Angeles Lakers 97 Golden State Warriors 136,Utah Jazz 98 Denver Nuggets 78,Boston Celtics 99 New York Knicks 85,\
    Indiana Pacers 98 Charlotte Hornets 86,Dallas Mavericks 87 Phoenix Suns 99,Atlanta Hawks 81 Memphis Grizzlies 82,\
    Miami Heat 110 Washington Wizards 105,Detroit Pistons 94 Charlotte Hornets 99,Orlando Magic 110 New Orleans Pelicans 107,\
    Los Angeles Clippers 130 Golden State Warriors 95,Utah Jazz 102 Oklahoma City Thunder 113,San Antonio Spurs 84 Phoenix Suns 104,\
    Chicago Bulls 103 Indiana Pacers 94,Milwaukee Bucks 106 Minnesota Timberwolves 88,Los Angeles Lakers 104 Portland Trail Blazers 102,\
    Houston Rockets 120 New Orleans Pelicans 100,Boston Celtics 111 Brooklyn Nets 105,Charlotte Hornets 94 Chicago Bulls 86,\
    Cleveland Cavaliers 103 Dallas Mavericks 97");
    }

    #[test]
    fn basic_tests() {
        dotest(
            &ro(),
            "Boston Celtics",
            "Boston Celtics:W=4;D=0;L=0;Scored=403;Conceded=350;Points=12",
        );
        dotest(&ro(), "Boston Celt", "Boston Celt:This team didn't play!");
    }
}
