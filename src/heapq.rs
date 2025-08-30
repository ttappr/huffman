use std::cmp::Ordering::{self, *};


pub fn heapify<T, C, A>(heap: &mut [T], cmp: &C, aux: &A) 
where
    C: Fn(&T, &T, &A) -> Ordering
{
    for i in (0..heap.len() / 2).rev() {
        sift_up(heap, i, cmp, aux);
    }
}

pub fn heap_push<T, C, A>(heap: &mut Vec<T>, item: T, cmp: &C, aux: &A) 
where
    C: Fn(&T, &T, &A) -> Ordering
{
    let len = heap.len();
    heap.push(item);
    sift_down(heap, 0, len, cmp, aux);
}

pub fn heap_pop<T, C, A>(heap: &mut Vec<T>, cmp: &C, aux: &A) -> Option<T>
where
    C: Fn(&T, &T, &A) -> Ordering
{
    if !heap.is_empty() {
        let returnitem = heap.swap_remove(0);
        sift_up(heap, 0, cmp, aux);
        Some(returnitem)
    } else {
        None
    }
}

fn sift_up<T, C, A>(heap: &mut [T], pos: usize, cmp: &C, aux: &A)
where
    C: Fn(&T, &T, &A) -> Ordering 
{
    let mut pos      = pos;
    let     endpos   = heap.len();
    let     startpos = pos;
    let mut childpos = 2 * pos + 1;

    while childpos < endpos {
        let rightpos = childpos + 1;
        if rightpos < endpos 
            && cmp(&heap[childpos], &heap[rightpos], aux) != Less {
            childpos = rightpos;
        }
        heap.swap(pos, childpos);
        pos = childpos;
        childpos = 2 * pos + 1;
    }
    sift_down(heap, startpos, pos, cmp, aux);
}

fn sift_down<T, C, A>(heap     : &mut [T], 
                      startpos : usize, 
                      pos      : usize, 
                      cmp      : &C, 
                      aux      : &A) 
where 
    C: Fn(&T, &T, &A) -> Ordering
{
    let mut pos = pos;

    while pos > startpos {
        let parentpos = (pos - 1) >> 1;

        if cmp(&heap[pos], &heap[parentpos], aux) == Less {
            heap.swap(pos, parentpos);
            pos = parentpos;
        } else {
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify() {
        let mut heap = vec![23, 22, 55, 87, 59, 27, 90, 14, 82, 21, 44, 75, 
                            20, 50, 3, 34, 83, 72, 68, 8, 57, 58, 6, 95, 16, 
                            28, 13, 86, 76, 30, 79, 54, 24, 80, 65, 84, 53, 
                            78, 67, 56, 18, 93, 61, 42, 10, 77, 40, 2, 71, 
                            47, 85, 7, 26, 33, 32, 62, 9, 92, 43, 38, 88, 
                            73, 74, 41, 4, 35, 70, 19, 69, 15, 94, 0, 66, 
                            39, 31, 63, 89, 5, 25, 99, 91, 51, 98, 97, 1, 
                            96, 29, 37, 36, 45, 17, 11, 52, 60, 49, 81, 48, 
                            12, 46, 64];

        let expected = (0..100).collect::<Vec<_>>();
        let cmp = |a: &i32, b: &i32, _: &()| a.cmp(&b);

        heapify(&mut heap, &cmp, &());

        let mut result = Vec::with_capacity(heap.len());

        while let Some(v) = heap_pop(&mut heap, &cmp, &()) {
            result.push(v);
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn test_heap_push() {
        let vals = vec![23, 22, 55, 87, 59, 27, 90, 14, 82, 21, 44, 75, 
                        20, 50, 3, 34, 83, 72, 68, 8, 57, 58, 6, 95, 16, 
                        28, 13, 86, 76, 30, 79, 54, 24, 80, 65, 84, 53, 
                        78, 67, 56, 18, 93, 61, 42, 10, 77, 40, 2, 71, 
                        47, 85, 7, 26, 33, 32, 62, 9, 92, 43, 38, 88, 
                        73, 74, 41, 4, 35, 70, 19, 69, 15, 94, 0, 66, 
                        39, 31, 63, 89, 5, 25, 99, 91, 51, 98, 97, 1, 
                        96, 29, 37, 36, 45, 17, 11, 52, 60, 49, 81, 48, 
                        12, 46, 64];

        let expected = (0..100).collect::<Vec<_>>();
        let cmp = |a: &i32, b: &i32, _: &()| a.cmp(&b);

        let mut heap = Vec::with_capacity(vals.len());
        let mut result = Vec::with_capacity(vals.len());

        for val in vals {
            heap_push(&mut heap, val, &cmp, &());
        }

        while let Some(v) = heap_pop(&mut heap, &cmp, &()) {
            result.push(v);
        }

        assert_eq!(result, expected);
    }
}