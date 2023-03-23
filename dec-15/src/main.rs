/*
--- Day 15: Beacon Exclusion Zone ---

You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels.
 You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors
 that you imagine were originally built to locate lost Elves.

The sensors aren't very powerful, but that's okay;
your handheld device indicates that you're close enough to the source of the distress signal to use them.
You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.

Once a sensor finds a spot it thinks will give it a good reading,
it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon.
Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely;
however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance.
(There is never a tie where two beacons are the same distance to a sensor.)

It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input). For example:

Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3

So, consider the sensor at 2,18; the closest beacon to it is at -2,15.
For the sensor at 9,16, the closest beacon to it is at 10,16.

Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like this:

               1    1    2    2
     0    5    0    5    0    5
 0 ....S.......................
 1 ......................S.....
 2 ...............S............
 3 ................SB..........
 4 ............................
 5 ............................
 6 ............................
 7 ..........S.......S.........
 8 ............................
 9 ............................
10 ....B.......................
11 ..S.........................
12 ............................
13 ............................
14 ..............S.......S.....
15 B...........................
16 ...........SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....

This isn't necessarily a comprehensive map of all beacons in the area, though.
Because each sensor only identifies its closest beacon, if a sensor detects a beacon,
you know there are no other beacons that close or closer to that sensor.
There could still be beacons that just happen to not be the closest beacon to any sensor. Consider the sensor at 8,7:

               1    1    2    2
     0    5    0    5    0    5
-2 ..........#.................
-1 .........###................
 0 ....S...#####...............
 1 .......#######........S.....
 2 ......#########S............
 3 .....###########SB..........
 4 ....#############...........
 5 ...###############..........
 6 ..#################.........
 7 .#########S#######S#........
 8 ..#################.........
 9 ...###############..........
10 ....B############...........
11 ..S..###########............
12 ......#########.............
13 .......#######..............
14 ........#####.S.......S.....
15 B........###................
16 ..........#SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....

This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or closer (in any positions marked #).

None of the detected beacons seem to be producing the distress signal,
so you'll need to work out where the distress beacon is by working out where it isn't.
For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.

So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10,
you'd like to count the number of positions a beacon cannot possibly exist.
The coverage from all sensors near that row looks like this:

                 1    1    2    2
       0    5    0    5    0    5
 9 ...#########################... 25
10 ..####B######################.. 26
11 .###S#############.###########. 27

In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.

Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?

*/

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::cmp;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Pair {
    beacon: Point,
    sensor: Point
}
impl Pair {
    fn radius(self: &Self) -> i32 {
        return (self.beacon.x - self.sensor.x).abs() + (self.beacon.y - self.sensor.y).abs()
    }
    fn delta_y(self: &Self) -> i32 {
        return (self.beacon.y - self.sensor.y).abs()
    }
    fn delta_x(self: &Self) -> i32 {
        return (self.beacon.x - self.sensor.x).abs()
    }
    fn covers(self: &Self, y: i32, (min, max): (i32, i32)) ->Vec<i32> {
        let mut coverage = vec![];
        let radius = self.radius();
        let distance =  (self.sensor.y - y).abs();
        let range_on_line = (cmp::max(0, radius - distance)).abs();
        if distance <= radius {
            // let range_on_line = radius - distance;
            // let delta_x = self.delta_y();
            let mut start_x = self.sensor.x - range_on_line;
            let mut end_x = self.sensor.x + range_on_line;
            if start_x < min {
                start_x = min;
            }

            if end_x > max {
                end_x = max;
            }
            coverage = (start_x..end_x+1).collect();
        }
        // println!("{:?}",self.radius());
        return coverage
    }

    fn anti_covers(self: &Self, y: i32, (min, max): (i32, i32)) -> (i32, i32) {
        // let mut coverage = vec![];
        let radius = self.radius();
        let distance =  (self.sensor.y - y).abs();
        let range_on_line = (cmp::max(0, radius - distance)).abs();
        if distance <= radius {
            // let range_on_line = radius - distance;
            // let delta_x = self.delta_y();
            let mut start_x = self.sensor.x - range_on_line;
            let mut end_x = self.sensor.x + range_on_line;
            if start_x < min {
                start_x = min;
            }

            if end_x > max {
                end_x = max;
            }
            return (start_x, end_x)
            // let start: Vec<i32> = (min..start_x+1).collect();
            // let end: Vec<i32> = (end_x..max+1).collect();
            // coverage.extend(start);
            // coverage.extend(end);
            // coverage = (start_x..end_x+1).collect()
        }
        return (min, max)
        // println!("{:?}",self.radius());
        // return coverage
    }
}

fn get_pairs(filename: &str) -> Vec<Pair> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut pairs: Vec<Pair> = vec![];
    for line in lines {
        let l = line.unwrap();
        let mut split = l.split(":");

        let sensor_data =  &split.next().unwrap().to_string()[10..];
        let mut sensor_split = sensor_data.split(",");
        let sensor_x:i32 = sensor_split.next().unwrap().to_string()[2..].parse().unwrap();
        let sensor_y:i32 = sensor_split.next().unwrap().to_string()[3..].parse().unwrap();
        let sensor = Point{x: sensor_x, y: sensor_y};
        // sensors.push(sensor);

        let beacon_data =  &split.next().unwrap().to_string()[22..];
        let mut beacon_split = beacon_data.split(",");
        let beacon_x: i32 = beacon_split.next().unwrap().to_string()[2..].parse().unwrap();
        let beacon_y:i32 = beacon_split.next().unwrap().to_string()[3..].parse().unwrap();
        let beacon = Point{x: beacon_x, y: beacon_y};
        // beacons.push(beacon);

        pairs.push(Pair{beacon, sensor});

    }
    pairs

}

fn part_one(filename: &str, target_line: i32) -> std::io::Result<()> {
    let mut set_on_target_line: HashSet<i32> = HashSet::new();
    let pairs: Vec<Pair> = get_pairs(filename);
    // println!("target_line {} min {} max {}", target_line);
    let mut covers = HashSet::new();
    for pair in pairs {
        if pair.sensor.y == target_line {
            set_on_target_line.insert(pair.sensor.x);
        }

        if pair.beacon.y == target_line {
            set_on_target_line.insert(pair.beacon.x);
        }
        let cover = pair.covers(target_line, (std::i32::MIN, std::i32::MAX));
        let cover_set:HashSet<i32> = HashSet::from_iter(cover.iter().cloned());
        covers.extend(cover_set);
    }

    let result = &covers - &set_on_target_line;
    // let mut arr = result.into_iter().collect::<Vec<i32>>();
    // arr.sort();
    println!("{}", result.len());
    Ok(())
}

/*
--- Part Two ---

Your handheld device indicates that the distress signal is coming from a beacon nearby.
The distress beacon is not detected by any sensor,
but the distress beacon must have x and y coordinates each no lower than 0 and no larger than 4000000.

To isolate the distress beacon's signal, you need to determine its tuning frequency,
which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.

In the example above, the search space is smaller: instead,
the x and y coordinates can each be at most 20. With this reduced search area,
there is only a single position that could have a beacon: x=14, y=11. The tuning frequency for this distress beacon is 56000011.

Find the only possible position for the distress beacon. What is its tuning frequency?
*/

fn compute_tuning_frequency(x:i32, y:i32) -> i32 {
    x* 4000000 + y
}

fn part_two(filename: &str, search_space: i32) -> std::io::Result<()> {
    // let mut candidates: HashSet<i32> = HashSet::new();
    let line: HashSet<i32> = HashSet::from_iter(0..search_space+1);
    let pairs: Vec<Pair> = get_pairs(filename);
    for y in 0..search_space+1 {
        let mut left = 0;
        let mut right = search_space;
        println!("{}/{}", y, search_space+1);
        let pairs: Vec<Pair> = get_pairs(filename);
        let mut covers: HashSet<i32> = HashSet::new();
        for pair in pairs {
            let (min, max) = pair.anti_covers(y, (0, search_space+1));
            if min > left {
                left = min;
            }
            if max < right {
                right = max;
            }
            // let cover_set:HashSet<i32> = HashSet::from_iter(cover);
            // covers.extend(cover_set);
        }
        println!("y:{} {} {}", y, left, right);
        // let candidates:HashSet<_> = line.difference(&covers).collect();
        if left == right {
            println!("x: {:?} y:{}", left, y);
            break
        }
        // println!("y {:?} covers {:?} candidates {:?}", y, candidates, covers);

    }

    Ok(())
}

fn main() {
    // part_one("input.txt", 2000000);
    // part_one("input-test.txt",10);
    part_two("input-test.txt", 20);
    // part_two("input.txt", 4000000);
}
