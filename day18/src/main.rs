use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let file = File::open("./18input.txt")?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();


    let grid_dimension = 71;

    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for line in &lines{
        let parsed: Vec<&str> = line.split(',').collect();
        if let (Ok(a), Ok(b)) = (parsed[0].trim().parse::<i32>(), parsed[1].trim().parse::<i32>()) {
            pairs.push((a, b));
        }
    }

    p2(pairs, grid_dimension);
    Ok(())
}

fn p2(pairs: Vec<(i32, i32)>, grid_dimension: usize){
    let mut grid: Vec<Vec<bool>> = vec![vec![true; grid_dimension]; grid_dimension];

    for byte in &pairs{
        let row = byte.0 as usize;
        let col = byte.1 as usize;
        grid[row][col] = false;         
        match bfs(grid.clone(), grid_dimension) {
            Some(_option) => continue,
            None => {
                println!("no path for {},{}", byte.0, byte.1);
                break;
            }
        }
    }
}
/* 
fn p1(pairs: Vec<(i32, i32)>, grid_dimension: usize){
    let grid = bytefall(pairs, grid_dimension);

    match bfs(grid, grid_dimension) {
        Some(distance) => println!("path length: {}", distance),
        None => println!("no path????"), 
    }
}

fn bytefall(bytes: Vec<(i32, i32)>, dimension: usize) -> Vec<Vec<bool>>{
    let max = 1024;
    let mut fallen = 0;
    let mut grid: Vec<Vec<bool>> = vec![vec![true; dimension]; dimension];

    for byte in &bytes{
        if fallen >= max {
            break;
        }
        let row = byte.0 as usize;
        let col = byte.1 as usize;
        grid[row][col] = false;         
        fallen += 1;
    }

    grid
}
*/
fn bfs(grid: Vec<Vec<bool>>, dimension: usize) -> Option<usize>{
    let target = (dimension - 1, dimension - 1);
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

    let mut visited = vec![vec![false; dimension]; dimension];
    let mut queue = VecDeque::new();

    // BFS from top-left 
    if grid[0][0] {
        queue.push_back(((0, 0), 0)); // (position, distance)
        visited[0][0] = true;
    }

    while let Some(((x, y), dist)) = queue.pop_front() {
        // reached the target?
        if (x, y) == target {
            return Some(dist + 1);
        }

        // breadth
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if nx < dimension && ny < dimension && grid[nx][ny] && !visited[nx][ny] {
                    queue.push_back(((nx, ny), dist + 1));
                    visited[nx][ny] = true;
                }
            }
        }
    }

    // no path :(
    None
}