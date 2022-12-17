


use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn priority_enqueue (arr: &mut [i32], cnt: i32) {
    for i in 0..3 {
        if cnt >= arr[i] {
            for j in (i+1..3).rev() {
                arr[j] = arr[j-1];
            }
            arr[i] = cnt;
            break;
        }
    }

}

fn main()-> std::io::Result<()> {
    let file = File::open("input-test.txt")?;
    let reader = BufReader::new(file);
    
    let mut cnt: i32 = 0;
    let mut max: [i32;3] = [0;3] ;
    for line in reader.lines() {
        let l = line?;
        let is_empty = l.is_empty();
        if is_empty {
            priority_enqueue(&mut max, cnt);
            cnt = 0;
        }
        else {
            let i = l.parse::<i32>().unwrap();
            cnt += i;
        }
    }
    priority_enqueue(&mut max, cnt);

    let sum: i32 = max.iter().sum();
    println!("Sum is {}", sum);

    Ok(())
}