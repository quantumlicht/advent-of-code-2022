/*
--- Day 14: Regolith Reservoir ---

The distress signal leads you to a giant waterfall!
Actually, hang on - the signal seems like it's coming from the waterfall itself,
and that doesn't make any sense. However, you do notice a little path that leads behind the waterfall.

Correction: the distress signal leads you behind a giant waterfall!
There seems to be a large cave system here, and the signal definitely leads further inside.

As you begin to make your way deeper underground, you feel the ground rumble for a moment.
Sand begins pouring into the cave! If you don't quickly figure out where the sand is going, you could quickly become trapped!

Fortunately, your familiarity with analyzing the path of falling material will come in handy here.
You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.

Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path,
where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan.
After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point.
For example:

498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9

This scan means that there are two paths of rock; the first path consists of two straight lines,
and the second path consists of three straight lines.
(Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)

The sand is pouring into the cave from point 500,0.

Drawing rock as #, air as ., and the source of the sand as +, this becomes:


  4444445555
  9999990000
  4567890123
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ........#.
9 #########.

Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest.
A unit of sand is large enough to fill one tile of air in your scan.

A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand),
the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked,
the unit of sand attempts to instead move diagonally one step down and to the right.
Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right.
If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves,
at which point the next unit of sand is created back at the source.

So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down and then stops:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.

The second unit of sand then falls straight down, lands on the first one, and then comes to rest to its left:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########.

After a total of five units of sand have come to rest, they form this pattern:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########.

After a total of 22 units of sand:

......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########.

Finally, only two more units of sand can possibly come to rest:

......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.

Once all 24 units of sand shown above have come to rest,
all further sand flows out the bottom, falling into the endless void.
Just for fun, the path any new sand takes before falling forever is shown here with ~:

.......+...
.......~...
......~o...
.....~ooo..
....~#ooo##
...~o#ooo#.
..~###ooo#.
..~..oooo#.
.~o.ooooo#.
~#########.
~..........
~..........
~..........

Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

*/

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(PartialEq)]
enum Material  {
    Sand,
    Rock,
    Floor
}

fn build_rock_path(prev: &str, next: &str, rock_path: &mut HashMap<(i32,i32), Material>, abyss:&mut (i32, i32)) {
    let mut s_prev = prev.split(",");
    let mut s_next = next.split(",");
    let (x_prev,y_prev): (i32, i32) = (s_prev.next().unwrap().parse().unwrap(), s_prev.next().unwrap().parse().unwrap());
    let (x_next,y_next): (i32, i32) = (s_next.next().unwrap().parse().unwrap(), s_next.next().unwrap().parse().unwrap());
    // println!("{},{} {},{}", x_prev, y_prev, x_next, y_next);

    let x_seq = if x_prev < x_next {x_prev..x_next+1} else {x_next..x_prev+1};
    let y_seq = if y_prev < y_next {y_prev..y_next+1} else {y_next..y_prev+1};
    for x in x_seq.clone() {
        for y in y_seq.clone() {
            // println!("insert -> {},{}", x, y);
            rock_path.insert((x,y), Material::Rock);
            // Update the lowest and left-most point
            if abyss.0 > x {
                abyss.0 = x;

            }
            if abyss.1 < y {
                abyss.1 = y;
            }
        }
    }
}

fn settle_sand( start: (i32, i32), rock_path:&HashMap<(i32,i32), Material>, abyss: (i32, i32)) -> (i32, i32) {
    let positions = vec![(start.0, start.1+ 1), (start.0-1, start.1+1), (start.0+1, start.1+1)];

    for position in positions {
        let is_blocked = match rock_path.get(&position) {
            Some(_) => true,
            None => false
        };
        // The sand cannot settle at the next location, try the next position
        if is_blocked {
            continue;
        }
        if  position.0 <= abyss.0 || position.1 >= abyss.1 {
            // println!("falling into abyss");
            return position;
        }
        // println!("falling to {:?}", position);
        return settle_sand(position, rock_path, abyss);
    }
    // the sand is settled. below, left-down and right-down are all blocked
    // println!("idle pos {},{}", start.0, start.1);
    return start
}

fn drop_sand(start: (i32, i32),  rock_path:&mut HashMap<(i32,i32), Material>, abyss: (i32, i32)) ->bool {
    let idle_pos =  settle_sand(start, rock_path, abyss);
    rock_path.insert(idle_pos, Material::Sand);
    let into_abyss = idle_pos.0 <= abyss.0 || idle_pos.1 >= abyss.1;
    println!("into abyss {}\n", into_abyss);
    into_abyss

}

fn drop_sand_v2(start: (i32, i32),  rock_path:&mut HashMap<(i32,i32), Material>, abyss: (i32, i32)) ->bool {
    let idle_pos =  settle_sand(start, rock_path, (-1,99999999));
    rock_path.insert(idle_pos, Material::Sand);
    let into_abyss = idle_pos.0 == 500 && idle_pos.1 == 0;
    println!("into abyss {}\n", into_abyss);
    into_abyss

}



fn plot_grid(rock_path: &mut HashMap<(i32, i32), Material>) {
    let mut bx = 500;
    let mut by = 0;
    let mut tx = 500;
    let ty = 0;

    for (x,y) in rock_path.keys() {
        if x < &bx {
            bx = *x;
        }
        if x > &tx {
            tx = *x;
        }
        if y > &by {
            by = *y;
        }
    }
    println!("New grid bounds {},{} {},{}", bx,by, tx,ty );
    for y in ty..by+1 {
        let mut row = vec![];
        for x in bx..tx+1 {
            let symbol = match rock_path.get(&(x,y)) {
                Some(Material::Sand) => "o",
                Some(Material::Rock) => "#",
                Some(Material::Floor) => "&",
                None => "."
            };
            // println!("{},{} -> {}", x,y, symbol);
            row.push(symbol);
        }
        println!("{} {:?}", y, row.into_iter().join(""));
    }

}


fn part_one() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rock_path: HashMap<(i32, i32), Material> = HashMap::new();
    let mut abyss = (500,0);
    while let Some(line) = lines.next() {
        let l = line?;
        let split = l.split(" -> ").collect::<Vec<&str>>();
        let iter = Itertools::tuple_windows(split.iter());
        for (prev, next) in iter {
            // println!("\n------\npath: {:?} -> {:?}", prev, next);
            build_rock_path(prev, next, &mut rock_path, &mut abyss);
            // plot_grid(&mut rock_path);
        }
    }
    let mut counter = 0;
    plot_grid(&mut rock_path);
    while !drop_sand((500, 0), &mut rock_path, abyss) {
        plot_grid(&mut rock_path);
        println!("-------- round {}-----------\n", counter);
        println!("Abyss {},{}", abyss.0, abyss.1);

        counter +=1 ;
        if counter > 3000 {
            break
        }
    }
    println!("Counter {}", counter);
    Ok(())
}

/*
--- Part Two ---

You realize you misread the scan.
There isn't an endless void at the bottom of the scan - there's floor, and you're standing on it!

You don't have time to scan the floor,
so assume the floor is an infinite horizontal line with a y coordinate equal to two plus
the highest y coordinate of any point in your scan.

In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11.
(This is as if your scan contained one extra rock path like -infinity,11 -> infinity,11.)
With the added floor, the example above now looks like this:

        ...........+........
        ....................
        ....................
        ....................
        .........#...##.....
        .........#...#......
        .......###...#......
        .............#......
        .............#......
        .....#########......
        ....................
<-- etc #################### etc -->

To find somewhere safe to stand,
you'll need to simulate falling sand until a unit of sand comes to rest at 500,0,
blocking the source entirely and stopping the flow of sand into the cave.
In the example above, the situation finally looks like this after 93 units of sand come to rest:

............o............
...........ooo...........
..........ooooo..........
.........ooooooo.........
........oo#ooo##o........
.......ooo#ooo#ooo.......
......oo###ooo#oooo......
.....oooo.oooo#ooooo.....
....oooooooooo#oooooo....
...ooo#########ooooooo...
..ooooo.......ooooooooo..
#########################

Using your scan, simulate the falling sand until the source of the sand becomes blocked. How many units of sand come to rest?

*/
fn build_floor(y_offset:i32, rock_path: &mut HashMap<(i32, i32), Material>) {
    let mut bx = 500;
    let mut by = 0;
    let mut tx = 500;
    let ty = 0;
    let mut y_floor = -1;
    let mut bx_floor = 99999999;
    let mut tx_floor = -1;
    for ((x,y), material) in rock_path.into_iter() {
        if x < &bx && *material != Material::Floor {
            bx = *x;
        }
        if x > &tx && *material != Material::Floor{
            tx = *x;
        }
        if y > &by && *material != Material::Floor {
            by = *y;
        }
        if x >= &tx_floor && *material == Material::Floor {
            tx_floor = *x;
        }
        if x <= &bx_floor && *material == Material::Floor {
            bx_floor = *x;
        }
        if y >= &y_floor && *material == Material::Floor {
            y_floor = *y;
        }
    }

    // initial case without floor;
    println!("floor {} {} {}", bx_floor, tx_floor, y_floor );
    if y_floor == -1 {
        y_floor = by+2;
        bx_floor = bx;
        tx_floor = tx;
        for x in bx_floor-20..tx_floor+20 {
            rock_path.insert((x, y_floor), Material::Floor);
        }
    }
    else {
        println!("GROW FLOOR {} {} {} {}", bx, bx_floor, tx, tx_floor);
        //grow floor
        if bx <=  bx_floor+3 {
            rock_path.insert((bx-1, y_floor), Material::Floor);
            rock_path.insert((bx-2, y_floor), Material::Floor);
        }
        if tx >= tx_floor - 3 {
            rock_path.insert((tx+1, y_floor), Material::Floor);
            rock_path.insert((tx+2, y_floor), Material::Floor);
        }
    }



}
fn part_two() -> std::io::Result<()> {
    let file = File::open("input-test.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rock_path: HashMap<(i32, i32), Material> = HashMap::new();
    let mut abyss = (500,0);
    while let Some(line) = lines.next() {
        let l = line?;
        let split = l.split(" -> ").collect::<Vec<&str>>();
        let iter = Itertools::tuple_windows(split.iter());
        for (prev, next) in iter {
            // println!("\n------\npath: {:?} -> {:?}", prev, next);
            build_rock_path(prev, next, &mut rock_path, &mut abyss);
        }
    }
    build_floor(2, &mut rock_path);
    plot_grid(&mut rock_path);

    let mut counter = 0;
    while !drop_sand_v2((500, 0), &mut rock_path, abyss) {
        build_floor(2, &mut rock_path);
        // plot_grid(&mut rock_path);
        println!("-------- round {}-----------\n", counter);
        println!("Abyss {},{}", abyss.0, abyss.1);

        counter +=1 ;
        if counter > 30000 {
            break
        }
    }
    plot_grid(&mut rock_path);
    println!("Counter {}", counter);
    Ok(())
}
fn main() {
    // part_one();
    part_two();
}
