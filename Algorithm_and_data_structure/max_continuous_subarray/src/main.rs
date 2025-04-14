use max_continuous_subarray::model::max_continuous_subarray;
fn main() {
    let array = [1, -2, 4, 5, -2, 8, 3, -2, 6,3 ,7,-1 ];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [5, 9, 2, 3, 7];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [-3, -1, -5, -2];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [-10, 5, -3, -1];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [0, 0, 0, 0];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [-1, -2, 3, 4];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [10, -2, -3, 1];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [2, -1, 2, -1, 2];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [3, -2, 5, -1, 4, -6, 2, 7];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);

    let array = [-5, -3, 0, -8];
    let (optimum, (startpos,endpos)) = max_continuous_subarray(&array);
    println!("{array:?}");
    println!("的最大子数组为{optimum}, 从第{startpos}项[{}]加到第{endpos}项[{}]", array[startpos], array[endpos]);
}


