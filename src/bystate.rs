use std::collections::BTreeMap;

fn get_state(s: &str) -> &str {
    match s {
        "AZ" => "Arizona",
        "CA" => "California",
        "ID" => "Idaho",
        "IN" => "Indiana",
        "MA" => "Massachusetts",
        "OK" => "Oklahoma",
        "PA" => "Pennsylvania",
        "VA" => "Virginia",
        _ => panic!(),
    }
}

fn by_state(str: &str) -> String {
    // your code
    let mut hmap: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for record in str.split('\n') {
        //        println!("{}", record);
        let last_two = record.get(record.len() as usize - 2..).unwrap();
        let record = format!("..... {}", record.replace(",", ""));
        if hmap.contains_key(last_two) {
            hmap.get_mut(last_two).unwrap().push(record.clone());
        } else {
            hmap.insert(last_two, vec![record.clone()]);
        }
    }

    hmap.iter()
        .map(|(k, v)| {
            let state = get_state(k);
            let mut v = v.clone();
            v.sort();
            let s = v.join("\n").replace(k, state);
            format!("{}\n{}", state, s)
        })
        .collect::<Vec<String>>()
        .join("\n ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        println!("s:{}", s);
        let ans = by_state(s);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        let ad = "John Pulsett, 321 King Street, Palmouth MA\nAlisa Gord, 22 Prin Broadway, Georges VA\nOreste Thulas, 11354 East Bridge Road, Pensa OK\nPerry Falpas, 420 Land Road, Beaver Halls PA\nErica Adamson, 200 Station Road, Westbury MA\nPaulo Sims, 8A River Street, Richmond VA\nAnn Wildon, 334 Shore Parkway, Hill View CA\nAl Carpenter, 730 3rd Street, Boston MA";
        let adsol = "California\n..... Ann Wildon 334 Shore Parkway Hill View California\n Massachusetts\n..... Al Carpenter 730 3rd Street Boston Massachusetts\n..... Erica Adamson 200 Station Road Westbury Massachusetts\n..... John Pulsett 321 King Street Palmouth Massachusetts\n Oklahoma\n..... Oreste Thulas 11354 East Bridge Road Pensa Oklahoma\n Pennsylvania\n..... Perry Falpas 420 Land Road Beaver Halls Pennsylvania\n Virginia\n..... Alisa Gord 22 Prin Broadway Georges Virginia\n..... Paulo Sims 8A River Street Richmond Virginia";
        dotest(ad, adsol);

        let ad3 = "John Daggett, 341 King Road, Plymouth MA\nAlice Ford, 22 East Broadway, Richmond VA\nSal Carpenter, 73 6th Street, Boston MA";
        let ad3sol = "Massachusetts\n..... John Daggett 341 King Road Plymouth Massachusetts\n..... Sal Carpenter 73 6th Street Boston Massachusetts\n Virginia\n..... Alice Ford 22 East Broadway Richmond Virginia";
        dotest(ad3, ad3sol);
    }
}
