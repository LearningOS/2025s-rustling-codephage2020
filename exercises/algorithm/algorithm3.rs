/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn sort<T>(array: &mut [T])
where T: Ord
{
    //Bubble Sort
	// let len = array.len();
    // for i in 0..len {
    //     for j in 0..len - i - 1 {
    //         if array[j] > array[j + 1] {
    //             array.swap(j, j + 1);
    //         }
    //     }
    // }

    // Insertion Sort
    // for i in 1..array.len() {
    //     let mut j = i;
    //     while j > 0 && array[j - 1] > array[j] {
    //         array.swap(j - 1, j);
    //         j -= 1;
    //     }
    // }
    
    // Quick Sort
    // fn quick_sort<T: Ord>(array: &mut [T]) {
    //     let len = array.len();
    //     if len <= 1 {
    //         return;
    //     }
    //     let pivot_index = partition(array);
    //     quick_sort(&mut array[0..pivot_index]); 
    //     quick_sort(&mut array[pivot_index + 1..len]);
    // }
    // fn partition<T: Ord>(array: &mut [T]) -> usize {
    //     let len = array.len();
    //     let pivot_index = len / 2;
    //     array.swap(pivot_index, len - 1);
    //     let mut i = 0;
    //     for j in 0..len - 1 {
    //         if array[j] <= array[len - 1] {
    //             array.swap(i, j);
    //             i += 1;
    //         }
    //     }
    //     array.swap(i, len - 1);
    //     i
    // }
    // quick_sort(array);
    
    // Heap Sort
    let len = array.len();

    for i in (0..len / 2).rev() {
        heapify(array, i, len);
    }

    for i in (1..len).rev() {
        array.swap(0, i);
        heapify(array, 0, i);
    }

    fn heapify<T>(array: &mut [T], root: usize, heap_size: usize)
    where
    T: Ord,
    {
        let mut largest = root;
        let left = 2 * root + 1;
        let right = 2 * root + 2;

        if left < heap_size && array[left] > array[largest] {
            largest = left;
        }

        if right < heap_size && array[right] > array[largest] {
            largest = right;
        }

        if largest != root {
            array.swap(root, largest);
            heapify(array, largest, heap_size);
        }
    }



    //???
    //array.sort();


}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}