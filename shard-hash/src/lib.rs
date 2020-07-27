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
