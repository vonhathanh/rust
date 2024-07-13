// Since the result must have k elements != k at the start
// we could start the loop at the end of the array and then
// swap the value != k to the start index
// 1. Init swap_index = 0, i = nums.len() - 1
// 2. Loop until i <= swap_index
// 3. While nums[swap_index] != val && swap_index <= i: swap_index += 1
// 4. While nums[i] == val && i > swap_index: i -= 1
// 5. Swap when i > swap_index and nums[i] != val
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // handles edge cases
    // empty array
    if nums.len() == 0 {
        return 0;
    }
    let mut swap_idx = 0;
    let mut i = nums.len() - 1;
    let mut k = 0;

    while i >= swap_idx {

        if i == swap_idx {
            if nums[i] == val {
                return k;
            } else {
                return k+1;
            }
        }

        while swap_idx < i && nums[swap_idx] != val {
            swap_idx += 1;
            k += 1;
        }

        while nums[i] == val && i > swap_idx {
            i -= 1;
        }

        // three possible cases happen after the swap
        // 1. swap_idx = i
        // 2. swap_idx < i
        // 3. swap_idx > i
        // in cases 2, 3 the loop will operate/halt as normal
        // in case 1. if nums[swap_idx] == val then break else increase k by 1 and exit
        if i > swap_idx && nums[i] != val {
            nums[swap_idx] = nums[i];
            nums[i] = val;
            k += 1;
            swap_idx += 1;
            i -= 1;
        }
    }

    return k;
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let mut result = remove_element(&mut nums, 3);
    println!("{}", result);

    nums = vec![0,1,2,2,3,0,4,2];
    result = remove_element(&mut nums, 2);
    println!("{}", result);
    
    nums = vec![];
    result = remove_element(&mut nums, 0);
    println!("{}", result);    

    nums = vec![1];
    result = remove_element(&mut nums, 1);
    println!("{}", result); 
}
