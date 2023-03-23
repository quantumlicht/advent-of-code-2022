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
use rand::thread_rng;
use rand::seq::SliceRandom;
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

struct PathSolution {
    cost_so_far: HashMap<Location, i32>,
    came_from: HashMap<Location, Location>,
    path: Vec<Location>,
    found: bool
}

impl PathSolution {
    fn show_explored(self: &Self, grid: Vec<Vec<Location>>, start: Location, end: Location) {

        // SHOW explored
        for row in &grid {
            let mut row_vec = vec![];
            for col in row {
                if start.x == col.x && start.y == col.y {
                    row_vec.push(String::from("S"));
                }
                else if end.x == col.x && end.y == col.y {
                    row_vec.push(String::from("E"));
                }
                else {
                    match self.cost_so_far.get(&col) {
                        Some(&i) => {row_vec.push(i.to_string())},
                        None => {
                            row_vec.push(String::from("."));
                        },
                    }
                }
            }
            let str = row_vec.join(",").clone();
            println!("{:?}", str);
        }
    }
    fn show_came_from(self: &Self,grid: Vec<Vec<Location>>, start: Location, end: Location ) {
        // SHOW CAME FROM
        for row in &grid {
            let mut row_vec = vec![];
            for col in row {
                if start.x == col.x && start.y == col.y {
                    row_vec.push(String::from("S"));
                }
                else if end.x == col.x && end.y == col.y {
                    row_vec.push(String::from("E"));
                }
                else {
                    match self.came_from.get(&col) {
                        Some(&i) => {row_vec.push(String::from("X"))},
                        None => {
                            row_vec.push(String::from("."));
                        },
                    }
                }
            }
            let str = row_vec.join("").clone();
            println!("{:?}", str);
        }
    }

    fn compute_path(self: &Self, start: Location, end: Location) -> Vec<Location> {
        // COMPUTE PATH
        let mut current: Location = end;
        let mut path: Vec<Location> = vec![];
        while current != start {
            path.push(current.clone());
            current = *self.came_from.get(&current).unwrap_or_else(|| panic!("Ohoh: {:?}", &current))
        }
        path.push(start);
        path.reverse();
        println!("PathSolution len {}", path.len() -1);// discard end
        path
    }
    fn show_path(self: &Self,grid: Vec<Vec<Location>>, start: Location, end: Location, num_rows: usize, num_cols: usize ) {
        // SHOW PATH ON GRID
        let mut path_grid: Vec<Vec<char>> = vec![];
        println!("num_rows {}", num_rows);
        println!("num_cols {}", num_cols);
        // init grid
        for i in 0..num_rows {
            let mut row_vec = vec![];
            for j in 0..num_cols {
                if start.x == i && start.y == j {
                    row_vec.push('S');
                }
                else if end.x == i && end.y == j {
                    row_vec.push('E');
                }
                else {
                    row_vec.push('.');
                }
            }
            path_grid.push(row_vec);
        }
        let g = grid.clone();
        // add path nodes
        for p in &self.path {
            if path_grid[p.x][p.y] != 'S' && path_grid[p.x][p.y] != 'E'  {
                path_grid[p.x][p.y] = g[p.x][p.y].repr;
            }
        }
        for row in path_grid {
            let mut row_vec = vec![];
            for col in row {
                row_vec.push(col);
            }
            println!("{:?}",  row_vec.iter().collect::<String>());
        }
    }
}

impl Graph {
    fn cost(self: &Self, from: Location, to: Location) -> i32 {
        match self.weigths.get(&to) {
            Some(&i) => 1,
            None => panic!("Weigth not found {:?}", to)
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
                    else {
                        let diff: i32 = grid[neigh.0 as usize][neigh.1 as usize].v - grid[x][y].v;
                        let is_connected = diff <= 1; // can go down, one up or flat
                        match edges_map.get_mut(&grid[x][y]) {
                            Some(edges) => {
                                weigths.insert(grid[x][y].clone(),1);
                                if is_connected {
                                    edges.push(grid[neigh.0 as usize][neigh.1 as usize].clone());
                                }
                            }
                            None => {
                                weigths.insert(grid[x][y].clone(), 1);
                                if is_connected {
                                    edges_map.insert(grid[x][y].clone(), vec![grid[neigh.0 as usize][neigh.1  as usize]].clone());
                                }
                            }
                            }
                    }
                }
                // println!("{},{}: {} [{:?}]", x, y, grid[x][y].repr, edges_map.get_mut(&grid[x][y]))
            }
        }

        return Graph{ edges_map, weigths}
    }

    fn neighbors(self: &Self, id: Location) -> Vec<Location> {
        let mut neighbours = match self.edges_map.get(&id) {
            Some(neigh) => neigh.to_vec(),
            None => vec![]
        };
        // neighbours.shuffle(&mut rand::thread_rng());
        return neighbours
    }
    fn bfs(self: &Self, start: Location, target: Location) -> PathSolution {
        let mut frontier = PriorityQueue::new();
        frontier.push(start.clone(), 0);

        let mut came_from = HashMap::<Location, Location>::new();

        let mut cost_so_far = HashMap::<Location, i32>::new();
        cost_so_far.insert(start, 0);
        let mut found_solution = false;
        while !frontier.is_empty() {
            let (current, _) = frontier.pop().unwrap();
            if current == target {
                found_solution = true;
                // println!("Found!");
            //     break
            }

            // println!("\n-----------{:?}-----------", current);
            for neigh in self.neighbors(current) {
                let current_cost = cost_so_far.get(&current).unwrap();

                let new_cost = current_cost + self.cost(current, neigh);
                let (neigh_is_visited, neigh_cost_so_far) = match cost_so_far.get(&neigh) {
                    Some(c) => (true, *c) ,
                    _ => (false, 9999),
                };
                if !neigh_is_visited {
                    cost_so_far.insert(neigh, new_cost);
                    // println!("not visited {:?} {}", neigh, new_cost);
                    // if current.repr >= 'n' {
                    // println!("new path {:?} -> {:?} {:?}", current, neigh, new_cost);
                    // }
                    let priority = new_cost; //+ self.heuristic(neigh, target);
                    // frontier.push(neigh.clone(), priority);
                    frontier.push(neigh.clone(), 1);
                    came_from.insert(neigh.clone(), current);
                }
                else if new_cost < neigh_cost_so_far {
                    cost_so_far.insert(neigh, new_cost);
                    let priority = new_cost; //+ self.heuristic(neigh, target);
                    // frontier.push(neigh.clone(), priority);
                    frontier.push(neigh.clone(), 1);
                    came_from.insert(neigh.clone(), current);
                }
                else {
                    // println!("ignored {:?}", neigh)
                }

            }
        }

        // cost_so_far.insert(target, 0);
        let mut sol = PathSolution{came_from, cost_so_far, path: vec![], found: found_solution};
        if found_solution {
            let path = sol.compute_path(start, target);
            sol.path = path;
        }
        sol
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
    let mut num_rows =0;
    let mut num_cols = 0;
    for (i,row) in char_grid.iter().enumerate() {
        num_rows += 1;
        num_cols = 0;
        let mut row_vec = vec![];
        for (j,col) in row.iter().enumerate() {
            num_cols +=1;
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
    let graph = Graph::new(&grid);

    let path = graph.bfs(start, end);
    path.show_came_from(grid.clone(), start, end);
    path.show_path(grid.clone(), start, end, num_rows, num_cols);


    // println!("path {:?}", path.iter().map(|l| l.repr.to_string() + " " + &l.x.to_string() + "," + &l.y.to_string()).collect::<Vec<String>>());
    Ok(())
}


/*
--- Part Two ---
As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The beginning isn't very scenic, though; perhaps you can find a better starting point.

To maximize exercise while hiking, the trail should start as low as possible: elevation a.
The goal is still the square marked E.
However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll need to find the shortest path from any square at elevation a to the square marked E.

Again consider the example from above:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

...v<<<<
...vv<<^
...v>E^^
.>v>>>^^
>^>>>>>^

This path reaches the goal in only 29 steps, the fewest possible.

What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?
*/

fn part_two() ->  std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut char_grid:Vec<Vec<char>> = vec![];
    for line in reader.lines(){
        let l = line?;
        let chars = l.chars().collect::<Vec<char>>();
        let _ = &char_grid.push(chars);
    }
    let mut grid: Vec<Vec<Location>> = vec![];
    let mut possible_starts: Vec<Location> = vec![];

    // let mut start = Location{x: 0, y: 0, v: 0, repr: '@', is_goal: false};
    let mut end = Location{x: 0, y: 0, v: 0, repr: '@', is_goal: false};

    let mut num_rows =0;
    let mut num_cols = 0;
    for (i,row) in char_grid.iter().enumerate() {
        num_rows += 1;
        num_cols = 0;
        let mut row_vec = vec![];
        for (j,col) in row.iter().enumerate() {
            num_cols +=1;
            if *col == 'S' || *col == 'a' {

                let start = Location{x: i, y:j, v: 1, repr: 'a', is_goal: false};
                possible_starts.push(start);
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
    let mut min_len = 9999999999;
    possible_starts.sort(|a, b| {
    } );
    for start in possible_starts {
        let graph = Graph::new(&grid);
        let path: PathSolution = graph.bfs(start, end);
        if path.found {
            // println!("Start {:?}", start);
            let l = path.path.len()-1;
            if l < min_len {
                min_len = l;
                println!("{}", min_len);
            }
            // path.show_path(grid.clone(), start, end, num_rows, num_cols);
        }
    }
    // path.show_came_from(grid.clone(), start, end);
    Ok(())
}

fn main() {
    // part_one();
    part_two();
}
