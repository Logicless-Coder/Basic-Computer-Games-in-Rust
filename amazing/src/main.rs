use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Unable to take input.");
    let inputs: Result<Vec<u8>, _> = line.trim().split(' ').map(str::parse).collect();

    let nums: Vec<u8>;
    match inputs {
        Ok(ns) => {
            nums = ns;
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    let width = nums[0];
    let height = nums[1];
    println!("Maze: {} x {}", width, height);

    let vr = '│';
    let hr = '─';
    let tl = '┌';
    let tr = '┐';
    let bl = '└';
    let br = '┘';
    let rt = '├';
    let lt = '┤';
    let tt = '┴';
    let bt = '┬';
    let jn = '┼';
    let sp = '.';

    let sample = format!("{tl}{hr}{bt}{hr}{bt}{hr}{tr}\n{vr}{sp}{vr}{sp}{vr}{sp}{vr}\n{rt}{hr}{jn}{hr}{jn}{hr}{lt}\n{vr}{sp}{vr}{sp}{vr}{sp}{vr}\n{bl}{hr}{tt}{hr}{tt}{hr}{br}");

    println!("{}", sample);
}

/*

tl hr bt hr bt hr tr
vr .. vr .. vr .. vr
rt hr jn hr jn hr lt
vr .. vr .. vr .. vr
bl hr tt hr tt hr br

*/
