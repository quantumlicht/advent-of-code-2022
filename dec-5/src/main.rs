/*
--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships.
Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates,
the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks.
To ensure none of the crates get crushed or fall over,
the crane operator will rearrange them in a series of carefully-planned steps.
 After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, b
ut they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as
possible so they can embark.

They do, however,
have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates.
Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top.
Stack 2 contains three crates; from bottom to top, they are crates M, C, and D.
Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given.
In each step of the procedure, a quantity of crates is moved from one stack to a different stack.
In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1,
resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

In the second step, three crates are moved from stack 1 to stack 3.
Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1.
Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack;
in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3,
so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use queues::*;
use std::collections::VecDeque;


fn parse_file() -> std::io::Result<()>{
    // for line in reader.lines() {
    //     crate_stack_idx = 0;
    //     let l = line?;
    //     if l.is_empty(){
    //         collect_crates = false;
    //         println!("line is empty");
    //     }

    //     if collect_crates {
    //         let char_iter: Vec<char> = l.chars().collect();
    //         for col in char_iter.chunks(4).into_iter() {

    //             let crate_stack =  crate_stacks.get(crate_stack_idx);
    //             let mut current_stack = Vec::<&str>::new();
    //             match crate_stack {
    //                 Some(v) => {current_stack = crate_stack.to_vec()},
    //                 None => ()
    //             };

    //             let column = col.iter().collect::<String>();
    //             // Empty column. next one.
    //             if column.trim().is_empty(){
    //                 crate_stack_idx += 1;
    //             }
    //             else {
    //                 // Crate in column
    //                 if regex.is_match(&column) {
    //                     let matches = regex.captures(&column).unwrap();
    //                     let letter = matches.get(1).unwrap().as_str();
    //                     // println!("{:?}", letter);
    //                     current_stack.push(letter);
    //                 }
    //                 //Matched something else. We are done parsing crates
    //                 else {
    //                     break
    //                     // println!("{}", cr);
    //                 }
    //             }

    //         }
    //     }
    //     else {
    //         println!("Collecting moves: {}", crate_stacks.len());
    //         for v in crate_stacks.iter() {
    //             println!("{:?}", v)
    //         }
    //     }
    // }
    Ok(())
}

fn part_one() -> std::io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r"\[([A-Z])\]").unwrap();
    let move_regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]*)$").unwrap();
    let mut collect_crates = true;
    let mut crate_stack_idx = 0;
    let mut crate_stacks: Vec<Vec<&str>> = Vec::new();


    let mut stacks = vec![
       VecDeque::from(["P", "D", "Q", "R", "V", "B", "H", "F"]),
       VecDeque::from(["V", "W", "Q", "Z", "D", "L"]),
       VecDeque::from(["C", "P", "R", "G", "Q", "Z", "L", "H"]),
       VecDeque::from(["B", "V", "J", "F", "H", "D", "R"]),
       VecDeque::from(["C", "L", "W", "Z"]),
       VecDeque::from(["M", "V", "G", "T", "N", "P", "R", "J"]),
       VecDeque::from(["S", "B", "M", "V", "L", "R", "J"]),
       VecDeque::from(["J", "P", "D"]),
       VecDeque::from(["V", "W", "N", "C", "D"]),
    ];
    // let mut stacks = vec![
    //     VecDeque::from(["N", "Z"]),
    //     VecDeque::from(["D", "C", "M"]),
    //     VecDeque::from(["P"]),
    // ];
    for line in reader.lines(){
        let l = line?;
        let matches = move_regex.captures(&l).unwrap();
        let qnt = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let from = matches.get(2).unwrap().as_str().parse::<i32>().unwrap()-1;
        let to = matches.get(3).unwrap().as_str().parse::<i32>().unwrap() -1;
        let i:usize = from as usize;
        let j:usize = to as usize;
        println!("{}", l);
        println!("before \nfrom {:?}  to{:?}", stacks[i], stacks[j]);
        // part 1
        for idx in 0..qnt {
            let mut ele = stacks[i].pop_front().unwrap();
            stacks[j].push_front(ele);
        }
        // part2
        let mut tmp_stack = vec![];
        for idx in 0..qnt {
            let mut ele = stacks[i].pop_front().unwrap();
            println!("ele {:?}", ele);
            tmp_stack.push(ele);

        }
        println!("tmpstack {:?}", tmp_stack);
        tmp_stack.reverse();
        for e in tmp_stack.iter() {
            stacks[j].push_front(e);
        }

        println!("after \nfrom {:?} to{:?}", stacks[i], stacks[j]);

    }
    for s in stacks{
        println!("{:?}", s[0]);
    }


    Ok(())
}

fn main() {
    part_one();
}
