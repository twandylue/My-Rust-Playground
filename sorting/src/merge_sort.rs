struct MergedSort {}

impl MergedSort {
    fn sort_recursive(items: Vec<i32>) -> Vec<i32> {
        if items.is_empty() {
            return items;
        }
        let output = Self::sort_helper(&items[..]).to_vec();

        return output;
    }

    fn sort_helper(items: &[i32]) -> Vec<i32> {
        if items.len() == 2 {
            if items[0] > items[1] {
                return vec![items[1], items[0]];
            } else {
                return items.to_vec();
            }
        } else if items.len() == 1 {
            return items.to_vec();
        }

        let mid = items.len() / 2;
        let left = Self::sort_helper(&items[..mid]);
        let right = Self::sort_helper(&items[mid..]);

        return Self::merge(&left[..], &right[..]);
    }

    fn merge(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let mut m = nums1.len();
        let mut n = nums2.len();
        let mut index = m + n - 1;
        let mut nums = vec![0; m + n];

        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums[index] = nums1[m - 1];
                m -= 1;
            } else {
                nums[index] = nums2[n - 1];
                n -= 1;
            }
            if index > 0 {
                index -= 1;
            }
        }

        while m > 0 || n > 0 {
            if m > 0 {
                nums[index] = nums1[m - 1];
                m -= 1;
            } else if n > 0 {
                nums[index] = nums2[n - 1];
                n -= 1;
            }

            if index > 0 {
                index -= 1;
            }
        }

        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merged_sort_recursive_case_1() {
        // arrange
        let input = Vec::from([4, 3, 2, 1]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_2() {
        // arrange
        let input = Vec::from([1, 2, 3, 4]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_3() {
        // arrange
        let input = Vec::from([1, 4, 2, 3]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_4() {
        // arrange
        let input = Vec::from([1, 3, 2, 4]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_5() {
        // arrange
        let input = Vec::from([1, 3, 2, 2, 4]);
        let expected = vec![1, 2, 2, 3, 4];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_6() {
        // arrange
        let input = Vec::<i32>::new();
        let expected: Vec<i32> = vec![];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_7() {
        // arrange
        let input = Vec::from([0, 0, 0, 0]);
        let expected = vec![0, 0, 0, 0];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_8() {
        // arrange
        let input = Vec::from([1, 3, 2]);
        let expected = vec![1, 2, 3];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_9() {
        // arrange
        let input = Vec::from([5, 1, 1, 2, 0, 0]);
        let expected = vec![0, 0, 1, 1, 2, 5];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_10() {
        // arrange
        let input = Vec::from([5, 2, 3, 1]);
        let expected = vec![1, 2, 3, 5];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn merged_sort_recursive_case_11() {
        // arrange
        let input = Vec::from([
            -74, 48, -20, 2, 10, -84, -5, -9, 11, -24, -91, 2, -71, 64, 63, 80, 28, -30, -58, -11,
            -44, -87, -22, 54, -74, -10, -55, -28, -46, 29, 10, 50, -72, 34, 26, 25, 8, 51, 13, 30,
            35, -8, 50, 65, -6, 16, -2, 21, -78, 35, -13, 14, 23, -3, 26, -90, 86, 25, -56, 91,
            -13, 92, -25, 37, 57, -20, -69, 98, 95, 45, 47, 29, 86, -28, 73, -44, -46, 65, -84,
            -96, -24, -12, 72, -68, 93, 57, 92, 52, -45, -2, 85, -63, 56, 55, 12, -85, 77, -39,
        ]);
        let expected = vec![
            -96, -91, -90, -87, -85, -84, -84, -78, -74, -74, -72, -71, -69, -68, -63, -58, -56,
            -55, -46, -46, -45, -44, -44, -39, -30, -28, -28, -25, -24, -24, -22, -20, -20, -13,
            -13, -12, -11, -10, -9, -8, -6, -5, -3, -2, -2, 2, 2, 8, 10, 10, 11, 12, 13, 14, 16,
            21, 23, 25, 25, 26, 26, 28, 29, 29, 30, 34, 35, 35, 37, 45, 47, 48, 50, 50, 51, 52, 54,
            55, 56, 57, 57, 63, 64, 65, 65, 72, 73, 77, 80, 85, 86, 86, 91, 92, 92, 93, 95, 98,
        ];

        // act
        let actual = MergedSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }
}
