use std::{collections::HashMap};

fn mean(list: &Vec<i32>) -> i32 {
    let mut total = 0;
    for n in list {
        total += n
    }
    total / list.len() as i32
}

fn median(list: &Vec<i32>) -> i32 {
    let middle = list.len() / 2;
    let mut my_list = list.clone();
    my_list.sort();
    if list.len() % 2 == 0 {
        (my_list[middle] + my_list[middle - 1]) / 2
    } else {
        my_list[middle]
    }
}

fn mode(list: &Vec<i32>) -> Option<i32> {
    let mut hash = HashMap::new();
    for n in list {
        let counter = hash.entry(n).or_insert(0);
        *counter += 1;
    }
    let mut result: Option<i32> = None;
    let mut max_count = 0;
    for (k, v) in hash {
        if v > max_count {
            result = Some(*k);
            max_count = v;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_mean() {
        let list = vec![1, 2, 2, 3];
        assert_eq!(mean(&list), 2);
    }

    #[test]
    fn test_median() {
        let list1: Vec<i32> = vec![1, 4, 2, 2, 4];
        assert_eq!(median(&list1), 2);

        let list2: Vec<i32> = vec![1, 4, 3];
        assert_eq!(median(&list2), 3);
    }

    #[test]
    fn test_mode() {
        let list: Vec<i32> = vec![1, 2, 2, 3, 4, 3, 3, 5];
        assert_eq!(mode(&list), Some(3));
    }
}
