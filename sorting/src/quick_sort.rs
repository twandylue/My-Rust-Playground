pub struct QuickSort {}

impl QuickSort {
    pub fn sort_non_recursive(items: Vec<i32>) -> Vec<i32> {
        todo!();
    }

    pub fn sort_recursive(items: Vec<i32>) -> Vec<i32> {
        if items.is_empty() {
            return items;
        }

        let mut output = items.clone();
        Self::quick_sort_helper(0, output.len() - 1, &mut output);

        return output;
    }

    // pivot is the last item.
    fn quick_sort_helper(left: usize, right: usize, items: &mut Vec<i32>) {
        if left < right && right < items.len() {
            let pivot: usize = right;
            println!("original items: {items:?}");
            println!("left: {left}");
            println!("pivot: {pivot}");
            println!("right: {right}");

            let new_pivot = Self::partition(left, pivot - 1, pivot, items);
            println!("new pivot: {new_pivot}");
            println!("new items: {items:?}");
            println!("--------");

            if new_pivot > 0 {
                Self::quick_sort_helper(left, new_pivot - 1, items);
            }

            Self::quick_sort_helper(new_pivot + 1, right, items);
        }
    }

    fn partition(mut left: usize, mut right: usize, pivot: usize, items: &mut Vec<i32>) -> usize {
        while left <= right && right < items.len() && right > 0 {
            if items[left] <= items[pivot] {
                left += 1;
            } else if items[left] > items[pivot] && items[right] > items[pivot] {
                right -= 1;
            } else if items[left] > items[pivot] && items[right] <= items[pivot] {
                Self::swap(left, right, items);
                left += 1;
                right -= 1;
            }
        }

        if items[left] > items[pivot] {
            Self::swap(left, pivot, items);
            return left;
        }

        return pivot;
    }

    fn swap(l: usize, r: usize, items: &mut Vec<i32>) {
        let temp = items[l];
        items[l] = items[r];
        items[r] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_recursive_case_1() {
        // arrange
        let input = Vec::from([4, 3, 2, 1]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_2() {
        // arrange
        let input = Vec::from([1, 2, 3, 4]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_3() {
        // arrange
        let input = Vec::from([1, 4, 2, 3]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_4() {
        // arrange
        let input = Vec::from([1, 3, 2, 4]);
        let expected = vec![1, 2, 3, 4];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_5() {
        // arrange
        let input = Vec::from([1, 3, 2, 2, 4]);
        let expected = vec![1, 2, 2, 3, 4];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_6() {
        // arrange
        let input = Vec::<i32>::new();
        let expected: Vec<i32> = vec![];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_7() {
        // arrange
        let input = Vec::from([0, 0, 0, 0]);
        let expected = vec![0, 0, 0, 0];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_8() {
        // arrange
        let input = Vec::from([1, 3, 2]);
        let expected = vec![1, 2, 3];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_9() {
        // arrange
        let input = Vec::from([5, 1, 1, 2, 0, 0]);
        let expected = vec![0, 0, 1, 1, 2, 5];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_10() {
        // arrange
        let input = Vec::from([5, 2, 3, 1]);
        let expected = vec![1, 2, 3, 5];

        // act
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn quick_sort_recursive_case_11() {
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
        let actual = QuickSort::sort_recursive(input);

        // assert
        assert_eq!(expected, actual);
    }
}
