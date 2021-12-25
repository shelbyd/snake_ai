use crate::KeyedQueue;

pub struct TreeSearch<T, FRank, O, FExpand> {
    ranker: FRank,
    expander: FExpand,
    queue: KeyedQueue<O, T>,
}

impl<T, FRank, O, FExpand> TreeSearch<T, FRank, O, FExpand>
where
    FRank: FnMut(&T) -> O,
    FExpand: FnMut(&T) -> Vec<T>,
    O: Ord + Eq,
{
    pub fn new(seed: T, mut ranker: FRank, expander: FExpand) -> Self {
        let mut queue = KeyedQueue::new();
        queue.insert(ranker(&seed), seed);

        TreeSearch {
            queue,
            ranker,
            expander,
        }
    }
}

impl<T, FRank, O, FExpand> Iterator for TreeSearch<T, FRank, O, FExpand>
where
    FRank: FnMut(&T) -> O,
    FExpand: FnMut(&T) -> Vec<T>,
    O: Ord + Eq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.queue.pop()?;
        for child in (self.expander)(&ret) {
            self.queue.insert((self.ranker)(&child), child);
        }
        Some(ret)
    }
}
