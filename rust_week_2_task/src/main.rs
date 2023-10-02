#[derive(Debug)]

struct FilterCondition {
    filter_num: i32,
}

impl FilterCondition {
    fn is_match(&self, num: &i32) -> bool {
        *num == self.filter_num
    }
}

fn custom_filter(nums: &Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    let filtered_nums = nums
        .iter()
        .filter(|item| filter.is_match(item))
        .cloned()
        .collect();
    return filtered_nums;
}

fn main() {
    let input_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_condition = FilterCondition { filter_num: 1 };

    let filtered_nums = custom_filter(&input_nums, &filter_condition);

    println!("{:?}", filtered_nums);
}