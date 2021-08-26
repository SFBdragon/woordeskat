mod uncolored;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, Read, stdin};
use std::path::Path;
use rand::random;
#[cfg(feature = "colored")]
use colored::*;
#[cfg(not(feature = "colored"))]
use uncolored::*;


fn main() {
    let mut wsdb_file = File::open(Path::new("wsdb.txt")).unwrap();
    let mut wsdb_string = String::new(); // include_str!("../wsdb.txt");
    wsdb_file.read_to_string(&mut wsdb_string).unwrap();

    {   // ensure against duplicate tranlations
        let mut set = HashSet::new();
        for tl in wsdb_string.lines() {
            let str = tl.to_string();
            if !set.insert(str) {
                println!("DUPLICATE TL FOUND: {} - PLEASE REMOVE", tl);
            }
        }
        println!("");
    }

    let mut afr_map: HashMap<String, Vec<String>> = HashMap::with_capacity(1250);
    let mut eng_map: HashMap<String, Vec<String>> = HashMap::with_capacity(1400);

    for tl in wsdb_string.lines() {
        if let Some((afr, eng)) = tl.split_once('.') {
            let (afr, eng) = (
                afr.to_string().trim().to_lowercase(), 
                eng.to_string().trim().to_lowercase());

            if let Some(tls) = afr_map.get_mut(&afr) {
                tls.push(eng.clone());
            } else {
                afr_map.insert(afr.clone(), vec![eng.clone()]);
            }

            if let Some(tls) = eng_map.get_mut(&eng) {
                tls.push(afr);
            } else {
                eng_map.insert(eng, vec![afr]);
            }
        } else {
            println!("PARSE ERROR. TL: \"{}\"", tl);
        }
    }
    let afr_vec = afr_map.keys().collect::<Vec<&String>>();
    let eng_vec = eng_map.keys().collect::<Vec<&String>>();

    println!("{}", "    WOORDESKAT APPLICATION".bold());
    println!("------------------------------\n");

    let mut count = 0usize;
    let mut errors = VecDeque::with_capacity(5);
    let mut enforces = VecDeque::with_capacity(10);
    for _ in 0..5 {
        errors.push_back(None);
        enforces.push_back(None);
        enforces.push_back(None);
    }
    
    loop {
        let mut is_err = false;
        let err = errors.pop_front().expect("ERROR QUEUE EMPTY");
        let enf = enforces.pop_front().expect("ENFORCE QUEUE EMPTY");

        let counter;
        // fixme: enforce is eaten if both err and enf are Option::Some
        let (ask, asklang, ans, anslang)
        = if let Some(t) = err {
            is_err = true;
            counter = "RE".red();
            t
        } else if let Some(t) = enf {
            counter = "RE".green();
            t
        } else {
            count += 1;
            counter = count.to_string().white();
            if random() {
                let key = afr_vec[random::<usize>() % afr_vec.len()];
                (key, "Afrikaans".magenta(), afr_map.get(key).expect(""), "English".blue())
            } else {
                let key = eng_vec[random::<usize>() % eng_vec.len()];
                (key, "English".blue(), eng_map.get(key).expect(""), "Afrikaans".magenta())
            }
        };
        
        println!("{}. {} to {}: {}", counter, &asklang, &anslang, &ask);

        let mut input = String::with_capacity(30);
        stdin().lock().read_line(&mut input).expect("INPUT CAPTURE ERROR.");
        input = input.trim().to_lowercase().to_string();

        if ans.iter().any(|x| levenshtein::levenshtein(&input, x) < 2) {
            println!("{}", "Correct".green());
            errors.push_back(None);
            enforces.push_back(if is_err { Some((ask, asklang, ans, anslang)) } else { None });
        } else {
            println!("{}", "Incorrect".red());
            errors.push_back(Some((ask, asklang, ans, anslang)));
            enforces.push_back(None);
        }
        print!("Tranlations: {}", ans[0]);
        for tl in ans.iter().skip(1) {
            print!(", {}", tl);
        }
        println!("");
    }
}
