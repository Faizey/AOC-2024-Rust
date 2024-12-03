// With co-pilots help
pub fn run_part_1(contents: String) -> i32 {
    contents
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    return Some((left, right));
                }
            }
            None
        })
        .map(|(left, right)| (left - right).abs())
        .sum()
}

pub fn run_part_2(contents: String) -> i32 {
    let mut left_counts = std::collections::HashMap::new();
    let mut right_counts = std::collections::HashMap::new();

    contents.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                *left_counts.entry(left).or_insert(0) += 1;
                *right_counts.entry(right).or_insert(0) += 1;
            }
        }
    });

    left_counts
        .iter()
        .map(|(&left, &left_count)| left * right_counts.get(&left).unwrap_or(&0) * left_count)
        .sum()
}

// My first attempt at a rust solution

// pub fn run_part_1(contents: String) -> i32 {
//     let lines: Vec<&str> = contents.split("\n").collect();
//     let mut answer = 0;

//     let mut left_column: Vec<i32> = Vec::new();
//     let mut right_column: Vec<i32> = Vec::new();

//     for line in lines.iter() {
//         let parts: Vec<&str> = line.split_whitespace().collect();

//         if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
//             left_column.push(left);
//             right_column.push(right);
//         }
//     }

//     left_column.sort();
//     right_column.sort();

//     for (index, _) in left_column.iter().enumerate() {
//         let left_column_number = left_column[index];
//         let right_column_number = right_column[index];
//         let differnce = i32::abs(left_column_number - right_column_number);

//         answer = answer + differnce;
//     }

//     return answer;
// }

// pub fn run_part_2(contents: String) -> i32 {
//     let lines: Vec<&str> = contents.split("\n").collect();
//     let mut answer = 0;

//     let mut left_column: Vec<i32> = Vec::new();
//     let mut right_column: Vec<i32> = Vec::new();

//     for line in lines.iter() {
//         let parts: Vec<&str> = line.split_whitespace().collect();

//         if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
//             left_column.push(left);
//             right_column.push(right);
//         }
//     }

//     for (left_index, _) in left_column.iter().enumerate() {
//         let left_column_number = left_column[left_index];
//         let mut count = 0;

//         for (right_index, _) in right_column.iter().enumerate() {
//             let right_column_number = right_column[right_index];

//             if left_column_number == right_column_number {
//                 count += 1;
//             }
//         }

//         answer = answer + (left_column_number * count)
//     }

//     return answer;
// }
