# Day 23: Sudoku
<br>
Sudoku is a logic-based,[2][3] combinatorial[4] number-placement puzzle. The objective is to fill a 9×9 grid with digits so that each column, each row, and each of the nine 3×3 subgrids that compose the grid (also called "boxes", "blocks", or "regions") contains all of the digits from 1 to 9

```
fn main() {
    let matrix = array![
        [8, 0, 0, 1, 0, 9, 0, 7, 0],
        [0, 9, 0, 0, 0, 0, 8, 0, 0],
        [5, 0, 3, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 7, 9, 0],
        [0, 0, 7, 2, 6, 5, 3, 0, 0],
        [0, 3, 8, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 9, 0, 4, 0, 1],
        [0, 0, 6, 0, 0, 0, 0, 2, 0],
        [0, 5, 0, 4, 0, 2, 0, 0, 3]
    ];

    let mut s = Sudoku::new(matrix);
    s.sudoku(0, 0);
    println!("{:?}", s.get_result());
}
```
output:
```
[[8, 4, 5, 1, 3, 9, 6, 7, 2],
 [4, 9, 1, 3, 2, 6, 8, 5, 7],
 [5, 1, 3, 9, 4, 7, 2, 6, 8],
 [1, 2, 4, 6, 8, 3, 7, 9, 5],
 [9, 8, 7, 2, 6, 5, 3, 1, 4],
 [2, 3, 8, 7, 5, 1, 9, 4, 6],
 [7, 6, 2, 5, 9, 8, 4, 3, 1],
 [3, 7, 6, 8, 1, 4, 5, 2, 9],
 [6, 5, 9, 4, 7, 2, 1, 8, 3]] shape=[9, 9], strides=[9, 1], layout=C (0x1)
```
