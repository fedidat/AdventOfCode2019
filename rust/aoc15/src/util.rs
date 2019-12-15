fn print_grid(grid: &mut HashMap<Point,char>) {
    let min_x = (*grid.iter().min_by_key(|&(pt, _)| pt.x).unwrap().0).x;
    let max_x = (*grid.iter().max_by_key(|&(pt, _)| pt.x).unwrap().0).x;
    let min_y = (*grid.iter().min_by_key(|&(pt, _)| pt.y).unwrap().0).y;
    let max_y = (*grid.iter().max_by_key(|&(pt, _)| pt.y).unwrap().0).y;
    println!("{} {} {} {}", min_x, max_x, min_y, max_y);
    for i in (min_y..(max_y+1)).rev() {
        for j in min_x..(max_x+1) {
            let res = if (i, j) == (0, 0) { Some(&'D') } else { grid.get(&Point::new(j,i)) };
            print!("{}", *(res.unwrap_or(&' ')));
        }
        println!("");
    }
}