use std::collections::HashMap;
use std::io;
fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![10, 20, 30, 40, 50];
    let mut strs = "Hello World".to_string();

    for i in &mut v1 {
        *i += 1;
        strs.push_str("*");
        println!("{}", &strs[11..]);
    }
    for i in 0..100 {
        v.push(i)
    }

    let ele = v.get(14);
    match ele {
        Some(third) => println!("The third element is {third}{:?}", v1),
        None => println!("There is no third element."),
    }
    // right_pyramid();
    //  hashmape();
    //  hashee();
    //  ex1();
    //  ex2();
    ex3();
}

fn right_pyramid() {
    let mut str = String::new();
    loop {
        str.push('*');
        println!("{str}");
        if str.len() >= 5 {
            break;
        }
    }
}
fn hashmape() {
    let mut score = HashMap::new();
    score.insert(329, "Gagan");
    score.insert(34, "Prince");

    score.insert(34, "Pince");
    score.insert(35, "Prnce");
    score.insert(316, "Price");
    score.insert(36, "Price");
    score.insert(38, "Prine");
    score.insert(37, "Pince");
    score.insert(39, "rince");
    score.insert(313, "Prne");
    score.insert(318, "rinc");
    score.insert(315, "rinc");
    println!("{:?}", score);

    score.entry(329).or_insert("Vinay");
    let gagan = &score.get(&329).unwrap();
    for i in &score {
        println!("{} {} ", i.0, i.1);
    }
    println!("{}", gagan);
}

fn hashee() {
    let mut map: HashMap<&str, u8> = HashMap::new();
    let word = "my Gagan Hello my name my name Hello Gagan ";
    let words = word.split_whitespace();
    // println!("{words:}");

    for text in words {
        *map.entry(text).or_insert(0) += 1;
    }
    println!("{map:?}");
}

fn ex1() {
    let mut v = vec![10, 32, 43, 21, 1, 33, 10, 43, 5];
    let mut hash: HashMap<u32, u8> = HashMap::new();
    v.sort();
    println!("{:?}", v);
    let median = v.get(v.len() / 2);
    println!("Median Value is {:?}", median.unwrap());
    for hashe in v {
        *hash.entry(hashe).or_insert(0) += 1
    }
    let hashr = &hash.values().max();
    for (key, value) in &hash {
        if value == hashr.unwrap() {
            print!(" Mode Value is {key}: {value} ")
        }
    }
}

fn ex2() {
    let words = "Shut the door".split_whitespace();
    let predef = "ay";
    for word in words {
        let formatee = format!(" {}{}{}", &word[1..], &word[0..1].to_lowercase(), &predef);
        print!("{formatee}");
    }
}

fn ex3() {
    let mut companies: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Select the Department:");
        let mut dept = String::new();
        println!("Enter the Department that you want to select 1)Engineering 2)Sales 3)Technology (or type 'quit' to exit):");
        io::stdin()
            .read_line(&mut dept)
            .expect("Failed to read line");
        let dept = dept.trim().to_string();
        if dept == "quit" {
            break;
        }

        loop {
            println!("Enter the Employee Name to add (or type 'quit' to finish adding employees):");
            let mut empname = String::new();
            io::stdin()
                .read_line(&mut empname)
                .expect("Failed to read line");
            let empname = empname.trim().to_string();
            if empname == "quit" {
                break;
            }

            companies.entry(dept.clone()).or_default().push(empname);
            println!("{:?}", companies);
        }
    }
}

fn reademp() {
    println!("Enter the Employee name");
}
