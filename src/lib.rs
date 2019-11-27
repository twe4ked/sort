pub fn bubble_sort(list: &mut Vec<usize>) {
    loop {
        let mut swapped = false;
        for i in 0..list.len() {
            if let Some(j) = list.get(i + 1) {
                if &list[i] > j {
                    list.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        if !swapped {
            break;
        }
    }
}

// Lomuto partition scheme
pub fn quick_sort(list: &mut Vec<usize>) {
    fn inner(list: &mut Vec<usize>, lo: usize, hi: usize) {
        if lo < hi {
            let p = partition(list, lo, hi);
            inner(list, lo, p.saturating_sub(1));
            inner(list, p + 1, hi);
        }
    }

    fn partition(list: &mut Vec<usize>, lo: usize, hi: usize) -> usize {
        let pivot = list[hi];
        let mut i = lo;
        for j in lo..=hi {
            if list[j] < pivot {
                list.swap(i, j);
                i += 1;
            }
        }
        list.swap(i, hi);
        i
    }

    inner(list, 0, list.len().saturating_sub(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_bubble_sorts_with_duplicates() {
        let mut list = vec![3, 1, 5, 1, 4, 2];
        bubble_sort(&mut list);
        assert_eq!(list, vec![1, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_bubble_sorts_empty_lists() {
        let mut list = vec![];
        bubble_sort(&mut list);
    }

    #[test]
    fn it_quick_sorts_with_duplicates() {
        let mut list = vec![3, 1, 5, 1, 4, 2];
        quick_sort(&mut list);
        assert_eq!(list, vec![1, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_quick_sorts_empty_lists() {
        quick_sort(&mut vec![]);
    }
}
