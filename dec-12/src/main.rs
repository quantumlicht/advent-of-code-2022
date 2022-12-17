/*
--- Day 12: Hill Climbing Algorithm ---

You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input).
The heightmap shows the local area from above broken into a grid; the elevation of each square of the grid is given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

Also included on the heightmap are marks for your current position (S) and the location that should get the best signal (E).
Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible.
During each step, you can move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear,
the elevation of the destination square can be at most one higher than the elevation of your current square;
that is, if your current elevation is m, you could step to elevation n, but not to elevation o.
(This also means that the elevation of the destination square can be much lower than the elevation of your current square.)

For example:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

Here, you start in the top-left corner; your goal is near the middle.
You could start by moving down or right, but eventually you'll need to head toward the e at the bottom.
From there, you can spiral around to the goal:

v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^

In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<), or right (>).
The location that should get the best signal is still E, and . marks unvisited squares.

This path reaches the goal in 31 steps, the fewest possible.

What is the fewest steps required to move from your current position to the location that should get the best signal?

*/
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use priority_queue::PriorityQueue;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Location {
    v: i32,
    x: usize,
    y: usize,
    repr: char,
    is_goal: bool
}

#[derive(Eq, Debug, PartialEq)]
struct Graph {
    edges_map: HashMap<Location, Vec<Location>>,
    weigths: HashMap<Location, i32>
}

impl Graph {
    fn cost(self: &Self, from: Location, to: Location) -> i32 {
        match self.weigths.get(&from) {
            Some(&i) => 2,
            None => panic!("Weigth not found")
        }
    }
    fn heuristic(self: &Self, a: Location, b: Location) -> i32 {
        let x:i32 = (a.x as i32 - b.x as i32).try_into().unwrap();
        let y:i32 = (a.y as i32 - b.y as i32).try_into().unwrap();

        x.abs() + y.abs()
    }


    fn new(grid: &Vec<Vec<Location>>) -> Graph {
        let mut edges_map: HashMap<Location, Vec<Location>>  = HashMap::new();
        let mut weigths: HashMap<Location,i32>  = HashMap::new();
        let len_x = grid.len() as isize;
        let len_y = grid[0].len() as isize;
        for i in 0..len_x {
            for j in 0..len_y {
                let x = i as usize;
                let y = j as usize;
                let neighbors: Vec<(isize, isize)> = vec![(i, j+1 ), (i,j-1), (i-1, j), (i+1, j)];
                for neigh in neighbors {
                    if neigh.0 < 0 || neigh.0 >= len_x || neigh.1 < 0 || neigh.1 >= len_y {
                        continue
                    }
                    // valid path
                    else {
                        // println!("{}{} {}{}", x,y, neigh.0, neigh.1);
                        // println!("{:?} {:?}", grid[neigh.0 as usize][neigh.1 as usize], grid[x][y]);
                        let diff: i32 = grid[neigh.0 as usize][neigh.1 as usize].v - grid[x][y].v;
                        // println!(" diff->{}", diff);
                        if diff == 0 || diff == 1  {
                            match edges_map.get_mut(&grid[x][y]) {
                                Some(edges) => {
                                    edges.push(grid[neigh.0 as usize][neigh.1  as usize].clone());
                                    weigths.insert(grid[x][y].clone(), 1);
                                    // }
                                    // else {
                                    //     weigths.insert(grid[x][y].clone(), 9999999);
                                    // }

                                }
                                None => {
                                    edges_map.insert(grid[x][y].clone(), vec![grid[neigh.0 as usize][neigh.1  as usize]].clone());
                                    // if diff == 0 || diff == 1  {
                                    weigths.insert(grid[x][y].clone(), 1);
                                    // }
                                    // else {
                                    // weigths.insert(grid[x][y].clone(), 9999999);
                                    // }
                                }
                            }
                        }
                    }
                }

            }
        }
        return Graph{ edges_map, weigths}
    }

    fn neighbors(self: &Self, id: Location) -> Vec<Location> {
        let neigh = self.edges_map.get(&id).expect("key not found").to_vec();
        neigh
    }
    fn bfs(self: &Self, start: Location, target: Location) -> (HashMap<Location, Location>,  HashMap<Location, i32>) {
        let mut frontier = PriorityQueue::new();
        frontier.push(start.clone(), 0);

        let mut came_from = HashMap::<Location, Location>::new();

        let mut cost_so_far = HashMap::<Location, i32>::new();
        cost_so_far.insert(start, 2);

        while !frontier.is_empty() {
            let (current, _) = frontier.pop().unwrap();
            if current == target {
                println!("Found!");
                break
            }
            for next in self.neighbors(current) {
                let new_cost = cost_so_far.get(&current).unwrap() + self.cost(current, next);
                let (is_reached, current_cost) = match cost_so_far.get(&next) {
                    Some(c) => (true, *c) ,
                    _ => (false, 1),
                };


                if !is_reached || new_cost < current_cost {
                    cost_so_far.insert(next, new_cost);
                    let priority = new_cost ;//+ self.heuristic(next, target);
                    frontier.push(next.clone(), priority);
                    came_from.insert(next.clone(), current);
                }

            }
        }
        // cost_so_far.insert(target, 0);
        (came_from, cost_so_far)
    }
}


fn part_one() ->  std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut char_grid:Vec<Vec<char>> = vec![];
    for line in reader.lines(){
        let l = line?;
        let chars = l.chars().collect::<Vec<char>>();
        let _ = &char_grid.push(chars);
    }
    let mut grid: Vec<Vec<Location>> = vec![];
    let mut start = Location{x: 0, y: 0, v: 0, repr: '@', is_goal: false};
    let mut end = Location{x: 0, y: 0, v: 0, repr: '@', is_goal: false};

    for (i,row) in char_grid.iter().enumerate() {
        let mut row_vec = vec![];
        for (j,col) in row.iter().enumerate() {
            if *col == 'S' {
                start = Location{x: i, y:j, v: 1, repr: 'S', is_goal: false};
                row_vec.push(start);
            }
            else if *col == 'E' {
                end = Location{x: i, y:j, v: 26, repr: 'E', is_goal: true};
                row_vec.push(end);
            }
            else {
                let charcode = *col as u8;
                let mut s: i32 = 0;
                if charcode >  96 {
                    s = (charcode - 96) as i32;
                }
                else {
                   s = (charcode - 64 + 26) as i32;
                }

                let l = Location{x: i, y:j, v: s, repr:*col, is_goal: false};
                row_vec.push(l);
            }
        }
        grid.push(row_vec);
    }

    // println!("grid {:?}", grid);
    let graph = Graph::new(&grid);

    let (mut came_from, mut cost_so_far) = graph.bfs(start, end);
    println!("came from {:?}", came_from.keys().len());
    // println!("cost_so_far {:?}", cost_so_far);
    // let mut cost_grid: Vec<&str> = vec![];
    // for row in grid {
    //     let mut row_vec = vec![];
    //     for col in row {
    //         // println!("row {:?}", col);
    //         // println!("cost {:?}", cost_so_far.get(&col).unwrap());
    //         match cost_so_far.get(&col) {
    //             Some(&i) => {row_vec.push(i.to_string())},
    //             None => {
    //                 row_vec.push(String::from("0"));
    //             },
    //         }
    //     }
    //     let mut str = row_vec.join("").clone();
    //     println!("{:?}", str);
    //     // cost_grid.push(&str);
    // }

    let mut current: Location = end;
    let mut path: Vec<Location> = vec![];
    while current != start {
        println!("test");
        path.push(current.clone());
        current = *came_from.get(&current).unwrap_or_else(|| panic!("ah: {:?}", &current))
    }
    path.push(start);
    path.reverse();
    println!("path {:?}", path.iter().map(|l| l.repr.to_string() + " " + &l.x.to_string() + "," + &l.y.to_string()).collect::<Vec<String>>());
    println!("path len {:?}", path.len());
    Ok(())
}
fn main() {
    part_one();
}
