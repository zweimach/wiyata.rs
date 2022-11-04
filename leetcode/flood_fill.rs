pub fn flood_fill(
    image: Vec<Vec<i32>>,
    sr: i32,
    sc: i32,
    color: i32,
) -> Vec<Vec<i32>> {
    let mut image = image;
    let row = image.len();
    let col = image[0].len();
    let c1 = image[sr as usize][sc as usize];
    fill(row, col, &mut image, sr, sc, c1, color);
    image
}

const DIRECTION_LEN: usize = 4;

const DIRECTION_ROW: [i32; DIRECTION_LEN] = [1, 0, -1, 0];

const DIRECTION_COL: [i32; DIRECTION_LEN] = [0, 1, 0, -1];

fn fill(
    row: usize,
    col: usize,
    grid: &mut Vec<Vec<i32>>,
    c_row: i32,
    c_col: i32,
    c1: i32,
    c2: i32,
) {
    if c_row < 0 || c_col < 0 {
        return;
    }
    let c_row = c_row as usize;
    let c_col = c_col as usize;
    if c_row >= row || c_col >= col {
        return;
    }
    if c1 == c2 || grid[c_row][c_col] != c1 {
        return;
    }
    grid[c_row][c_col] = c2;
    let mut i = 0;
    while i < DIRECTION_LEN {
        fill(
            row,
            col,
            grid,
            c_row as i32 + DIRECTION_ROW[i],
            c_col as i32 + DIRECTION_COL[i],
            c1,
            c2,
        );
        i += 1;
    }
}
