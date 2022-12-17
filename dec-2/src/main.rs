
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;


// A for Rock, B for Paper, and C for Scissors.
// X for Rock, Y for Paper, and Z for Scissors

fn win_pts(x: &str, y: &str)-> i32 {
    // rock    
    if x == "A" {
        //rock
        if y == "X" {
            return 3
        }
        //paper
        if y == "Y" {
            return 6 
        }
        //cissor
        return 0
    }
    // paper
    else if x =="B" {
        //rock
        if y == "X" {
            return 0
        }
        //paper
        if y == "Y" {
            return 3 
        }
        //cissor
        return 6
    }
    // cissor
    else if x =="C" {
        //rock
        if y == "X" {
            return 6
        }
        //paper
        if y == "Y" {
            return 0 
        }
        //cissor
        return 3
    }
    else {
        return 0
    }
    
}

fn round_sign<'a>(x: &str, y: &'a str)-> &'a str {
    // rock
    if x == "A" {
        //loss
        if y == "X" {
            //cissor
            return "Z"
        }
        //draw
        if y == "Y" {
            return "X"
        }
        //paper
        return "Y"
    }
    // paper
    else if x =="B" {
        if y == "X" {
            //rock
            return "X"
        }
        //paper
        if y == "Y" {
            return "Y" 
        }
        //cissor
        return "Z"
    }
    // cissor
    else if x =="C" {
        if y == "X" {
            //paper
            return "Y"
        }
        //paper
        if y == "Y" {
            return "Z"
        }
        //cissor
        return "X"
    }
    else {
        return ""
    }
    
}

fn main()-> std::io::Result<()> {
    let file = File::open("input-test.txt")?;
    let reader = BufReader::new(file);
    let mut sign_points = HashMap::new();
    let mut win_mapping: HashMap<String, i32> = HashMap::new(); 
    win_mapping.insert(String::from("X"), 0);
    win_mapping.insert(String::from("Y"), 3);
    win_mapping.insert(String::from("Z"), 6);

    sign_points.insert(String::from("X"), 1);
    sign_points.insert(String::from("Y"), 2);
    sign_points.insert(String::from("Z"), 3);
    let mut pts = 0;
    for line in reader.lines() {
        let l = line?;
        let v: Vec<&str> = l.split_whitespace().collect();
        println!("{:?} {:?}", v[0], v[1]);
        let x = v[0];
        let y = v[1];
        let tmp_pts = sign_points.get(round_sign(x,y)).copied().unwrap_or(0) + win_mapping.get(y).copied().unwrap_or(0);
        pts +=  tmp_pts;
        println!("{} {}", tmp_pts, pts);
    }
    println!(" Points: {}", pts);

    Ok(())
}