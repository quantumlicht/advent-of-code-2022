/*
--- Day 8: Treetop Tree House ---

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid.
 The Elves explain that a previous expedition planted these trees as a reforestation effort.
 Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden.
To do this, you need to count the number of trees that are visible from outside
the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input).
For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge,
there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top.
    (It isn't visible from the right or bottom since other trees of height 5 are in the way.)

    The top-middle 5 is visible from the top and right.

    The top-right 1 is not visible from any direction; for it to be visible,
    there would need to only be trees of height 0 between it and an edge.

    The left-middle 5 is visible, but only from the right.

    The center 3 is not visible from any direction; for it to be visible,
    there would need to be only trees of at most height 2 between it and an edge.

    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior,
a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?

*/



use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;


fn mark_visible(i: i32, j:i32, grid:&Vec<Vec<i32>>, visibility_map: &mut HashMap<(i32, i32), bool> ) {
    let len_x = grid.len();
    let len_y = grid[0].len();
    let x = i as usize;
    let y = j as usize;
    let target = grid[x][y];

    let mut is_visible_east = true;
    for idx_y in y+1..len_y {
        if grid[x][idx_y] >= target {
            // println!("east {} >= {}", grid[x][idx_y], target);
            is_visible_east = false;
        }
    }

    let mut is_visible_south = true;
    for idx_x in x+1..len_x {
        if grid[idx_x][y] >= target {
            // println!("south {} >= {}", grid[idx_x][y], target);
            is_visible_south = false;
        }
    }

    let mut is_visible_north = true;
    for idx_x in 0..x {
        if grid[idx_x][y] >= target {
            // println!("north {} >= {}", grid[idx_x][y], target);
            is_visible_north = false;
        }
    }

    let mut is_visible_west = true;
    for idx_y in 0..y {
        if grid[x][idx_y] >= target {
            // println!("west {} >= {}", grid[x][idx_y], target);
            is_visible_west = false;
        }
    }


    if is_visible_east || is_visible_north || is_visible_south || is_visible_west {
        visibility_map.insert((i,j), true);
    }
}


fn scenic_score(i: i32, j:i32, grid:&Vec<Vec<i32>>, high_score: &mut i32 ) {
    let len_x = grid.len();
    let len_y = grid[0].len();
    let x = i as usize;
    let y = j as usize;
    let target = grid[x][y];

    let mut east_score = 0;
    for idx_y in y+1..len_y {
        if grid[x][idx_y] < target {
            east_score += 1;
        }
        else {
            east_score += 1;
            break;
        }
    }

    let mut south_score = 0;
    for idx_x in x+1..len_x {
        if grid[idx_x][y] < target {
            south_score += 1;
        }
        else {
            south_score +=1;
            break;
        }
    }

    let mut north_score = 0;
    for idx_x in (0..x).rev() {
        if grid[idx_x][y] < target {
            north_score += 1;
        }
        else {
            north_score +=1;
            break;
        }
    }

    let mut west_score = 0;
    for idx_y in (0..y).rev() {
        if grid[x][idx_y] < target {
            west_score += 1;
        }
        else {
            west_score += 1;
            break;
        }
    }

    println!("{},{}->{} score: e{} n{} s{} w{}", x, y, grid[x][y], east_score, north_score, south_score, west_score);

    let score =  east_score * north_score * south_score * west_score;
    println!("score {}", score);
    if score > *high_score {
        *high_score = score;
    }
}

fn part_one() ->  std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut grid:Vec<Vec<i32>> = vec![];
    for line in reader.lines(){
        let l = line?;
        let chars: Vec<i32> = l.chars().map(|i| i.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let _ = &grid.push(chars);
    }
    {
        let mut visibility_map: HashMap<(i32, i32), bool> = HashMap::new();
        let len_x: i32 = grid.len().try_into().unwrap();
        let len_y: i32 = grid[0].len().try_into().unwrap();

        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                // println!("--------------\n i:{} j:{} val:{}",i,j, grid[i][j] );
                if i == 0 || j == 0 || i == (len_x-1).try_into().unwrap() || j == (len_y-1).try_into().unwrap()   {
                    // println!("on edge {},{}", i , j);
                    visibility_map.insert((i.try_into().unwrap(),j.try_into().unwrap()), true);
                    continue
                }
                // let h = grid[i][j];
                // mark_visible(i.try_into().unwrap(), j.try_into().unwrap(),h, &grid, &mut visibility_map);
                mark_visible(i.try_into().unwrap(), j.try_into().unwrap(),&grid, &mut visibility_map);
            }
        }
        println!("{:?}", visibility_map);
        let sum = visibility_map.keys().len();
        println!("SUM {}", sum);
    }
    Ok(())
}


/*
--- Part Two ---

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree;
stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration.
(If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above;
the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390

    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).

A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390

    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).

This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?

*/
fn part_two() ->  std::io::Result<()> {

     let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut grid:Vec<Vec<i32>> = vec![];
    for line in reader.lines(){
        let l = line?;
        let chars: Vec<i32> = l.chars().map(|i| i.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let _ = &grid.push(chars);
    }
    {
        let len_x: i32 = grid.len().try_into().unwrap();
        let len_y: i32 = grid[0].len().try_into().unwrap();
        let mut high_score = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if i == 0 || j == 0 || i == (len_x-1).try_into().unwrap() || j == (len_y-1).try_into().unwrap()   {
                    continue
                }
                scenic_score(i.try_into().unwrap(), j.try_into().unwrap(),&grid, &mut high_score);
            }
        }
        println!("{}", high_score);
    }
    Ok(())
}

fn main() {
    part_one();
    part_two();
}
