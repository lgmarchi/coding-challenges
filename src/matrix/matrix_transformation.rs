// You are given the following parameters:
//  - A `matrix` of `m x n` integers filled with 0's and 1's.
//  - A set of coordinates `(a, b)` representing the position `matrix[b][a]`

// Develop a solution that fills all adjacent cells connected to `matrix[b][a]` and having the same original value as `matrix[a][b]` with the integer `5`.
// Only horizontal and vertical adjacency should be considered.
// Return the updated matrix after the operation.

// 4o

// Examples:

// Start: (1,1)

// Input:
// 1 1 1 1
// 1 0 0 1
// 1 0 0 1
// 1 1 1 1

// Output:
// 1 1 1 1
// 1 5 5 1
// 1 5 5 1
// 1 1 1 1

// Start: (1, 3)

// Input:
// 1 1 1 1 1 1 1 1
// 1 1 1 0 0 1 1 0
// 1 1 1 0 0 1 0 1
// 0 0 0 0 0 0 0 1
// 0 0 0 0 0 0 0 1
// 1 1 1 0 0 1 1 1
// 1 1 1 0 0 1 1 1
// 1 1 1 0 0 1 1 1

// Output:
// 1 1 1 1 1 1 1 1
// 1 1 1 5 5 1 1 0
// 1 1 1 5 5 1 5 1
// 5 5 5 5 5 5 5 1
// 5 5 5 5 5 5 5 1
// 1 1 1 5 5 1 1 1
// 1 1 1 5 5 1 1 1
// 1 1 1 5 5 1 1 1

// # First example
// input1 = [
//     [1, 1, 1, 0],
//     [1, 0, 1, 0],
//     [1, 0, 0, 1],
//     [1, 1, 1, 1]
// ]

// output1 = [
//     [1, 1, 1, 1],
//     [1, 5, 5, 1],
//     [1, 5, 5, 1],
//     [1, 1, 1, 1]
// ]

// # Second example
// input2 = [
//     [1, 1, 0],
//     [1, 0, 0],
//     [0, 0, 1]
// ]

// output2 = [
//     [5, 5, 0],
//     [5, 0, 0],
//     [0, 0, 1]
// ]

// # Third example
// input3 = [
//     [1, 1, 1, 1, 1, 1, 1, 1],
//     [1, 1, 1, 0, 0, 1, 1, 0],
//     [1, 1, 1, 0, 0, 1, 0, 1],
//     [0, 0, 0, 0, 0, 0, 0, 1],
//     [0, 0, 0, 0, 0, 0, 0, 1],
//     [1, 1, 1, 0, 0, 1, 1, 1],
//     [1, 1, 1, 0, 0, 1, 1, 1],
//     [1, 1, 1, 0, 0, 1, 1, 1]
// ]

// output3 = [
//     [1, 1, 1, 1, 1, 1, 1, 1],
//     [1, 1, 1, 5, 5, 1, 1, 0],
//     [1, 1, 1, 5, 5, 1, 5, 1],
//     [5, 5, 5, 5, 5, 5, 5, 1],
//     [5, 5, 5, 5, 5, 5, 5, 1],
//     [1, 1, 1, 5, 5, 1, 1, 1],
//     [1, 1, 1, 5, 5, 1, 1, 1],
//     [1, 1, 1, 5, 5, 1, 1, 1]
// ]

pub fn fill_area(matrix: &mut Vec<Vec<i32>>, a: usize, b: usize) {
    let m = matrix.len();
    let n = matrix[0].len();
    let original_value = matrix[b][a];

    // Se o valor já for 5, não há nada para fazer
    if original_value == 5 {
        return;
    }

    // Stack para DFS
    let mut stack = vec![(a, b)];

    while let Some((x, y)) = stack.pop() {
        if x >= n || y >= m || matrix[y][x] != original_value {
            continue;
        }

        matrix[y][x] = 5;

        // Empilha vizinhos válidos (esquerda, direita, cima, baixo)
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < n {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < m {
            stack.push((x, y + 1));
        }
    }
}
