/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
         // 将新元素添加到堆的末尾
        self.items.push(value);
        self.count += 1;

        // 向上调整
        let mut idx = self.count ; // 新元素的索引

        // 父节点的索引
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            
            // 如果当前元素小于（或大于）父节点，交换位置
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx; // 继续向上调整
            } else {
                break; // 堆性质已经满足
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child = self.left_child_idx(idx);
        let right_child = self.right_child_idx(idx);

        // 仅考虑存在的子节点
        if right_child > self.count {
            return left_child; // 只有左子节点
        }

        // 返回较小的子节点
        if (self.comparator)(&self.items[left_child], &self.items[right_child]) {
            left_child
        } else {
            right_child
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 如果堆为空，返回 None
        }
    
        // 取出根节点（堆顶元素）
        let root_value = self.items.swap_remove(1); // 将根节点与最后一个节点交换，并移除最后一个节点
        self.count -= 1; // 更新计数
    
        // 重新调整堆以保持堆性质
        let mut idx = 1; // 从根节点开始
        while self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);
            
            // 如果当前节点大于最小子节点，交换位置
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                self.items.swap(idx, smallest_child_idx);
                idx = smallest_child_idx; // 继续向下调整
            } else {
                break; // 堆性质已经满足
            }
        }
    
        Some(root_value) // 返回根节点的值
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}