use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::iter::Iterator;

#[derive(Copy, Clone)]
pub struct ShardHash<H: Hasher + Sized> {
    count: u64,
    hasher: H,
}

impl ShardHash<DefaultHasher> {
    pub fn new(count: u64) -> Self {
        Self {
            count,
            hasher: DefaultHasher::default(),
        }
    }
}

impl<H: Hasher + Sized> Hasher for ShardHash<H> {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}

impl<H: Hasher + Sized> IntoIterator for ShardHash<H> {
    type Item = u64;
    type IntoIter = ShardIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.finish(), self.count, self.count)
    }
}

#[derive(Clone)]
pub struct ShardIterator {
    state: u64,
    pos: u64,
    min: u64,
    visited: Vec<u64>,
}

impl ShardIterator {
    pub fn new(state: u64, pos: u64, size: u64) -> Self {
        Self {
            state,
            pos,
            min: pos-size,
            visited: Vec::with_capacity(size as usize),
        }
    }
}

impl Iterator for ShardIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.min {
            return None
        }

        // Calculate the base shard ID
        let mut ret = self.state % self.pos;

        // Update internal state
        self.state /= self.pos;
        self.pos -= 1;

        // Derive next available value
        while self.visited.contains(&ret) {
            ret += 1;
        }
        // Save in visited nodes
        self.visited.push(ret.clone());

        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.pos as usize, Some(self.pos as usize));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;
    use rand::prelude::*;

    // Static test for 7 to prevent alteration to the algorithm
    #[test]
    fn hash_7() {
        let mut sh = ShardHash::new(7);
        sh.write_u64(2237);
        let shards = sh.into_iter().collect::<Vec<u64>>();

        assert_eq!(shards, vec![1, 5, 4, 0, 2, 3, 6]);
    }

    // Static test for 7/1 to prevent alteration to the algorithm
    #[test]
    fn iterator_7_1() {
        let shards = ShardIterator::new(2237, 7, 1).collect::<Vec<u64>>();

        assert_eq!(shards, vec![4]);
    }

    // Static test for 7/3 to prevent alteration to the algorithm
    #[test]
    fn iterator_7_3() {
        let shards = ShardIterator::new(2237, 7, 3).collect::<Vec<u64>>();

        assert_eq!(shards, vec![4, 1, 3]);
    }

    // Static test for 7/7 to prevent alteration to the algorithm
    #[test]
    fn iterator_7_7() {
        let shards = ShardIterator::new(2237, 7, 7).collect::<Vec<u64>>();

        assert_eq!(shards, vec![4, 1, 3, 2, 5, 0, 6]);
    }

    // Test that the shards length is equal to the number of replicas
    #[test]
    fn length() {
        for _ in 0..100 {
            let value: u64 = random();
            let count = (random::<u64>() % 256) + 1;
            let replicas = (random::<u64>() % count) + 1;

            let shards = ShardIterator::new(value, count, replicas).collect::<Vec<u64>>();
            assert_eq!(shards.len() as u64, replicas);
        }
    }

    // Test that a number of replicas greater than count trigger a panic
    #[test]
    #[should_panic]
    fn invalid_replicas() {
        let value: u64 = random();
        let replicas = (random::<u64>() % 256) + 1;
        let count = (random::<u64>() % replicas) + 1;

        let _shards = ShardIterator::new(value, count, replicas).collect::<Vec<u64>>();
    }

    // Test that all values are unique
    #[test]
    fn unique() {
        fn has_unique_elements<T>(iter: T) -> bool
        where
            T: IntoIterator,	
            T::Item: Eq + Hash,	
        {	
            let mut uniq = HashSet::new();	
            iter.into_iter().all(move |x| uniq.insert(x))	
        }

        for _ in 0..100 {
            let value: u64 = random();
            let count = (random::<u64>() % 256) + 1;
            let replicas = (random::<u64>() % count) + 1;

            let shards = ShardIterator::new(value, count, replicas).collect::<Vec<u64>>();
            assert_eq!(shards.len() as u64, replicas);
            assert!(has_unique_elements(shards));
        }
    }

    // Test that the same parameters with less replicas start with the same sequence
    #[test]
    fn successive() {
        for _ in 0..100 {
            let value: u64 = random();
            let count = (random::<u64>() % 256) + 1;
            let replicas = (random::<u64>() % count) + 1;
            let replicas2 = (random::<u64>() % replicas) + 1;

            let shards = ShardIterator::new(value, count, replicas).collect::<Vec<u64>>();
            let shards2 = ShardIterator::new(value, count, replicas2).collect::<Vec<u64>>();
            assert_eq!(shards2[..], shards[..replicas2 as usize]);
        }
    }
}