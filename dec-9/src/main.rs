/*

--- Day 9: Rope Bridge ---

This rope bridge creaks as you walk along it.
You aren't sure how old it is, or whether it can even support your weight.

It seems to support the Elves just fine, though.
The bridge spans a gorge which was carved out by the massive river far below you.

You step carefully; as you do, the ropes stretch and twist.
You decide to distract yourself by modeling rope physics; maybe you can even figure out where not to step.

Consider a rope with a knot at each end; these knots mark the head and the tail of the rope.
If the head moves far enough away from the tail, the tail is pulled toward the head.

Due to nebulous reasoning involving Planck lengths,
you should be able to model the positions of the knots on a two-dimensional grid.
Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.

Due to the aforementioned Planck lengths,
the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):

....
.TH.
....

....
.H..
..T.
....

...
.H. (H covers T)
...

If the head is ever two steps directly up, down, left, or right from the tail,
the tail must also move one step in that direction so it remains close enough:

.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...

Otherwise, if the head and tail aren't touching and aren't in the same row or column,
the tail always moves one step diagonally to keep up:

.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

You just need to work out where the tail goes as the head follows a series of motions.
Assume the head and the tail both start at the same position, overlapping.

For example:

R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

This series of motions moves the head right four steps, then up four steps,
then left three steps, then down one step, and so on.
After each step, you'll need to update the position of the tail if the step means
the head is no longer adjacent to the tail. Visually, these motions occur as follows (s marks the starting position as a reference point):

== Initial State ==

......
......
......
......
H.....  (H covers T, s)

== R 4 ==

......
......
......
......
TH....  (T covers s)

......
......
......
......
sTH...

......
......
......
......
s.TH..

......
......
......
......
s..TH.

== U 4 ==

......
......
......
....H. (4,1)
s..T.. (3,0)

......
......
....H. (4,2)
....T. (4,1)
s.....

......
....H. (4,3)
....T. (4,2)
......
s.....

....H. (4,4)
....T. (4,3)
......
......
s.....

== L 3 ==

...H.. (3,4)
....T. (4,3)
......
......
s.....

..HT.. (2,4) (3,4)
......
......
......
s.....

.HT... (1,4) (2,4)
......
......
......
s.....

== D 1 ==

..T... (2,4)
.H.... (1,3)
......
......
s.....

== R 4 ==

..T... (2,4)
..H... (2,3)
......
......
s.....

..T... (2,4)
...H.. (3,3)
......
......
s.....

......
...TH. (3,3) (4,3)
......
......
s.....

......
....TH (4,3) (5,3)
......
......
s.....

== D 1 ==

......
....T. (4,3)
.....H (5,2)
......
s.....

== L 5 ==

......
....T.
....H.
......
s.....

......
....T.
...H..
......
s.....

......
......
..HT.. (2,2) (3,2)
......
s.....

......
......
.HT... (1,2) (2,2)
......
s.....

......
......
HT.... (0,2) (1,2)
......
s.....

== R 2 ==

......
......
.H....  (H covers T) (1,2)(1,2)
......
s.....

......
......
.TH...
......
s.....

After simulating the rope, you can count up all of the positions the tail visited at least once.
In this diagram, s again marks the starting position (which the tail also visited) and # marks other positions the tail visited:

..##..
...##.
.####.
....#.
s###..

So, there are 13 positions the tail visited at least once.

Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?

*/

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    println!("dx {} dy {} h: {:?} t: {:?}", dx, dy, head, tail);

    // NNE
    if dx == 1 && dy == 2 {
        tail.0 +=1;
        tail.1 += 1;
    }
    // NEE
    else if dx == 2 && dy == 1 {
        tail.0 +=1;
        tail.1 += 1;
    }
    // NNW
    else if dx == -1 && dy == 2 {
        tail.0 -=1;
        tail.1 += 1;
    }
    // NWW
    else if dx == -2 && dy == 1 {
        tail.0 -=1;
        tail.1 += 1;
    }

    // SSE
    else if dx == 1 && dy == -2 {
        tail.0 +=1;
        tail.1 -= 1;
    }
    // SEE
    else if dx == 2 && dy == -1 {
        tail.0 +=1;
        tail.1 -= 1;
    }

    // SSW
    else if dx == -1 && dy == -2 {
        tail.0 -=1;
        tail.1 -= 1;
    }
    // SWW
    else if dx == -2 && dy == -1 {
        tail.0 -=1;
        tail.1 -= 1;
    }

    // N
    else if dy > 1 && dx == 0 {
        tail.1 += 1;
    }
    // S
    else if dy < -1 && dx == 0 {
        tail.1 -= 1;
    }
    // W
    else if dy ==0  && dx < -1 {
        tail.0 -= 1;
    }
    // E
    else if dy == 0 && dx > 1 {
        tail.0 += 1;
    }
    // OTHER
    else if dy ==2 && dx == 2 {
        tail.0 +=1;
        tail.1 +=1;
    }
    // OTHER
    else if dy ==-2 && dx == 2 {
        tail.0 +=1;
        tail.1 -=1;
    }
    // OTHER
    else if dy ==-2 && dx == -2 {
        tail.0 -=1;
        tail.1 -=1;
    }
    // OTHER
    else if dy ==2 && dx == -2 {
        tail.0 -=1;
        tail.1 +=1;
    }

    println!("new tail pos {:?}\n", tail);
}

fn move_head(direction: &str, nb_steps: i32, knots: &mut Vec<(i32, i32)>, hashmap: &mut HashMap<(i32, i32), bool>)  {
    println!("-----\nDirection {} {} {:?}", direction, nb_steps, knots);

    let mut head = knots.first().unwrap().clone();
    let mut tails = Vec::from_iter(knots[1..].iter().cloned());
    let tails_len = tails.len();
    for i in 0..nb_steps {
        match direction {
            "R" => {
                head.0 +=1;
            },
            "L" => {
                head.0 -=1;
            },
            "U" => {
                head.1 +=1;
            },
            "D" => {
                head.1 -=1;
            },
            _ => ()
        }
        // println!(">>> head movement {} #{}/{:?} >>> head {:?}",direction, i+1, nb_steps, head);
        let mut current_head = head.clone();

        let mut j = 0;

        for mut tail in &mut tails {
            // println!("tail #{:?} head {:?} tail {:?}", j, current_head, tail);
            move_tail(&current_head, &mut tail);
            if j == tails_len -1 {
                hashmap.insert(*tail, true);
            }
            current_head = *tail;
            j+=1;
        }
    }

    knots[0] = head;
    for (i, tail) in tails.into_iter().enumerate() {
        knots[i+1] = tail;
    }
}
fn part_one() ->  std::io::Result<()> {
    let regex = Regex::new(r"([A-Z]?) ([0-9]+)").unwrap();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hashmap: HashMap<(i32, i32), bool> = HashMap::new();
    let mut knots = vec![(0,0); 2];
    hashmap.insert(knots[1], true);
    for line in reader.lines() {
        let l = line?;
        let matches = regex.captures(&l).unwrap();
        let direction = matches.get(1).unwrap().as_str().to_string();
        let nb_steps = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();
        move_head(&direction, nb_steps, &mut knots, &mut hashmap);
    }
    // println!("hashmap {:?}", hashmap);
    println!("count {}", hashmap.keys().len());
    Ok(())
}

/*
--- Part Two ---

A rope snaps! Suddenly, the river is getting a lot closer than you remember.
The bridge is still there, but some of the ropes that broke are now whipping toward you as you fall through the air!

The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid being hit.
Fortunately, your simulation can be extended to support longer ropes.

Rather than two knots, you now must simulate a rope consisting of ten knots.
One knot is still the head of the rope and moves according to the series of motions.
Each knot further down the rope follows the knot in front of it using the same rules as before.

Using the same series of motions as the above example, but with the knots marked H, 1, 2, ..., 9, the motions now occur as follows:

== Initial State ==

......
......
......
......
H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

== R 4 ==

......
......
......
......
1H....  (1 covers 2, 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
21H...  (2 covers 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
321H..  (3 covers 4, 5, 6, 7, 8, 9, s)

......
......
......
......
4321H.  (4 covers 5, 6, 7, 8, 9, s)

== U 4 ==

......
......
......
....H.
4321..  (4 covers 5, 6, 7, 8, 9, s)

......
......
....H.
.4321.
5.....  (5 covers 6, 7, 8, 9, s)

......
....H.
....1.
.432..
5.....  (5 covers 6, 7, 8, 9, s)

....H.
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

== L 3 ==

...H..
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

..H1..
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

.H1...
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

..1...
.H.2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== R 4 ==

..1...
..H2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

..1...
...H..  (H covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...1H.  (1 covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21H
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

......
...21.
..43.H
.5....
6.....  (6 covers 7, 8, 9, s)

== L 5 ==

......
...21.
..43H.
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21.
..4H..  (H covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
..H1..  (H covers 4; 1 covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
.H13..  (1 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
H123..  (2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

== R 2 ==

......
......
.H23..  (H covers 1; 2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
.1H3..  (H covers 2, 4)
.5....
6.....  (6 covers 7, 8, 9, s)

Now, you need to keep track of the positions the new tail, 9, visits.
In this example, the tail never moves, and so it only visits 1 position.
However, be careful: more types of motion are possible than before, so you might want to visually compare your simulated rope to the one above.

Here's a larger example:

R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

These motions occur as follows (individual steps are not shown):

== Initial State ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........H..............  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== R 5 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........54321H.........  (5 covers 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== U 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
................H......... (5,8)
................1......... 5
................2.........
................3.........
...............54.........
..............6...........
.............7............
............8.............
...........9..............  (9 covers s)
..........................
..........................
..........................
..........................
..........................

== L 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
........H1234.............
............5.............
............6.............
............7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 3 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
.........2345.............
........1...6.............
........H...7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== R 17 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
................987654321H
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 10 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s.........98765
.........................4
.........................3
.........................2
.........................1
.........................H

== L 25 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
H123456789................

== U 20 ==

H.........................
1.........................
2.........................
3.........................
4.........................
5.........................
6.........................
7.........................
8.........................
9.........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

Now, the tail (9) visits 36 positions (including s) at least once:

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
#.........................
#.............###.........
#............#...#........
.#..........#.....#.......
..#..........#.....#......
...#........#.......#.....
....#......s.........#....
.....#..............#.....
......#............#......
.......#..........#.......
........#........#........
.........########.........

Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?

*/
fn part_two() ->  std::io::Result<()> {
    let regex = Regex::new(r"([A-Z]?) ([0-9]+)").unwrap();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hashmap: HashMap<(i32, i32), bool> = HashMap::new();
    let mut knots = vec![(0,0); 10];
    // println!("knots {:?}", knots);
    hashmap.insert(knots[9], true);
    for line in reader.lines() {
        let l = line?;
        let matches = regex.captures(&l).unwrap();
        let direction = matches.get(1).unwrap().as_str().to_string();
        let nb_steps = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();
        move_head(&direction, nb_steps, &mut knots, &mut hashmap);
        // println!("knots {:?}", knots);
        // println!("hashmap {:?}", hashmap);
    }
    // println!("hashmap {:?}", hashmap);
    println!("count {}", hashmap.keys().len());

   Ok(())
}

fn main() {
//    part_one();
   part_two();
}
