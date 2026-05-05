use std::collections::{HashSet, VecDeque};

fn main() {
    let matix = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 1, 1, 1],
    ];
    let mut visited: HashSet<(usize, usize)> = HashSet::new(); //  使用集合作为访问标记
    for i in 0..matix.len() {
        for j in 0..matix[0].len() {
            if matix[i][j] == 1 && !visited.contains(&(i, j)) {
                // println!("Starting DFS at ({}, {})", i, j);
                // dfs_literation_3(&matix, &mut visited, i, j);
                // println!(); // 换行分隔不同的DFS结果
                println!("Starting BFS at ({}, {})", i, j);
                bfs(&matix, &mut visited, i, j);
                println!(); // 换行分隔不同的BFS结果
            }
        }
    }
}

fn dfs_literation_1(
    matrix: &[Vec<i32>],
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    // 结束条件
    if x >= matrix.len() || y >= matrix[0].len() || matrix[x][y] == 0 || visited.contains(&(x, y)) {
        return;
    }
    // 标记当前节点为已访问
    visited.insert((x, y));
    print!("({}, {}) ", x, y); // 打印当前访问的节点

    // 定义递归方向
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右、下、左、上
    // 递归访问四个方向的邻居节点
    for (dx, dy) in directions {
        let new_x = match x.checked_add_signed(dx) {
            Some(v) => v,
            None => continue,
        };
        let new_y = match y.checked_add_signed(dy) {
            Some(v) => v,
            None => continue,
        };
        dfs_literation_1(matrix, visited, new_x, new_y);
    }

    // 回溯时可以选择取消访问标记，视具体需求而定
}

// 改进版本：将结束条件放在递归函数入口处，减少不必要的递归调用
fn dfs_literation_2(
    matrix: &[Vec<i32>],
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    // 标记当前节点为已访问
    visited.insert((x, y));
    print!("({}, {}) ", x, y); // 打印当前访问的节点

    // 定义递归方向
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右、下、左、上
    // 递归访问四个方向的邻居节点
    for (dx, dy) in directions {
        let new_x = match x.checked_add_signed(dx) {
            Some(v) => v,
            None => continue,
        };
        let new_y = match y.checked_add_signed(dy) {
            Some(v) => v,
            None => continue,
        };
        // 递归退出条件
        if new_x >= matrix.len()
            || new_y >= matrix[0].len()
            || matrix[new_x][new_y] == 0
            || visited.contains(&(new_x, new_y))
        {
            continue;
        }
        dfs_literation_2(matrix, visited, new_x, new_y);
    }

    // 回溯时可以选择取消访问标记，视具体需求而定
}

/// 改进版本：使用显式栈 + 方向索引模拟递归调用和返回
/// 
/// 初始化：
/// - 使用一个栈存储待访问的节点，并将起始节点压栈。
/// - 定义一个方向数组，表示要遍历的方向。
/// 
/// 循环处理，当栈不为空时：
/// - 弹出栈顶节点 (x, y), 检查是否已访问，如已访问则跳过处理。
/// - 标记当前节点为已访问，并执行所需的操作（例如打印、计数）。
/// - 遍历四个方向的邻居 (nx, ny)。
/// - 如果邻居坐标合法、值为1且未被访问：
///   - 将邻居节点压栈。（可能将重复的节点压入栈中，故在出栈时需要检查）
/// 
/// 终止：
/// - 栈为空，所有可达节点均已访问。

fn dfs_literation_3(
    matrix: &[Vec<i32>],
    visited: &mut HashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) {
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut stack = vec![(start_x, start_y)];   //  使用显式栈模拟递归调用，初始时将起始节点压栈
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右、下、左、上

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        assert!(visited.insert((x, y)));     // 标记当前节点为已访问
        print!("({}, {}) ", x, y);  // 执行所需的操作（例如打印、计数）。

        // 访问四个方向的邻居节点，并将符合条件的邻居节点压栈
        for (dx, dy) in directions.into_iter().rev() {
            let new_x = match x.checked_add_signed(dx) {
                Some(v) => v,
                None => continue,
            };
            let new_y = match y.checked_add_signed(dy) {
                Some(v) => v,
                None => continue,
            };
            // 符合条件的压栈
            if new_x < rows && new_y < cols && matrix[new_x][new_y] == 1 && !visited.contains(&(new_x, new_y)) {
                stack.push((new_x, new_y));
            }
        }
    }
}


/// 初始化：
/// - 使用一个队列存储待访问的节点。
/// - 将起点标记为已访问，并加入队列。
/// - 定义一个方向数组，表示要遍历的方向。
/// 
/// 循环处理，当队列非空时：
/// - 弹出队首节点 (x, y)。
/// - 对该节点执行所需的操作（例如打印、计数）。
/// - 遍历四个方向的邻居 (nx, ny)。
/// - 如果邻居坐标合法、值为1且未被访问：
///   - 标记为已访问。
///   - （如需要可记录距离 = 当前距离 + 1）
///   - 将邻居加入队列尾部。
/// 
/// 终止：
/// - 队列为空，所有可达节点均已访问。


fn bfs(
    matrix: &[Vec<i32>],
    visited: &mut HashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) {
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut queue = VecDeque::new();  // 使用一个队列存储待访问的节点。
    visited.insert((start_x, start_y));                         // 入队前需标记为已访问
    queue.push_back((start_x, start_y));
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右、下、左、上
    while let Some((x, y)) = queue.pop_front() {
        print!("({}, {}) ", x, y);  // 出队时处理节点
        for (dx, dy) in directions {
            let new_x = match x.checked_add_signed(dx) {
                Some(v) => v,
                None => continue,
            };
            let new_y = match y.checked_add_signed(dy) {
                Some(v) => v,
                None => continue,
            };
            if new_x < rows
                && new_y < cols
                && matrix[new_x][new_y] == 1
                && !visited.contains(&(new_x, new_y))
            {
                visited.insert((new_x, new_y));
                queue.push_back((new_x, new_y));
            }
        }
    }
}
