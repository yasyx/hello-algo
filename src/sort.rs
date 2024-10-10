pub fn selection_sort(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }

    let n = nums.len();

    for i in 0..n - 1 {
        let mut k = i;
        for j in (i + 1)..n {
            if nums[j] < nums[k] {
                k = j;
            }
        }
        nums.swap(i, k)
    }

    println!("selection_sort result: {:?}", nums)
}

pub fn bubble_sort(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }

    for i in (1..nums.len()).rev() {
        let mut flag = false;

        for j in 0..i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                flag = true;
            }
        }
        if !flag {
            break;
        }
    }

    println!("bubble_sort result: {:?}", nums)
}
