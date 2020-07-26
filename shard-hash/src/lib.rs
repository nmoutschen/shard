pub fn get_first_shard(data: usize, count: usize) -> usize {
    data % count
}

pub fn get_shards(data: usize, count: usize, replicas: usize) -> Vec<usize> {
    let mut shards = (0..count).into_iter().collect::<Vec<usize>>();
    let mut output = Vec::with_capacity(replicas);
    let mut data = data;

    for i in 0..replicas {
        let pos = data % (count-i);
        data /= count-i;
        output.push(shards.remove(pos));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::hash::Hash;
    use std::collections::HashSet;

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    fn shards() {
        for _ in 0..100 {
            let count = (random::<usize>() % 100) + 1;
            let replicas = (random::<usize>() % count) + 1;
            let data: usize = random();
            println!("{} {} {}", count, replicas, data);

            let shards = get_shards(data, count, replicas);

            assert_eq!(shards.len(), replicas);
            assert!(has_unique_elements(shards));
        }
    }

    #[test]
    fn first_shard_5() {
        let shard = get_first_shard(2237, 5);
        println!("{:?}", shard);
        assert_eq!(shard, 2);
    }

    #[test]
    fn first_shard_7() {
        let shard = get_first_shard(2237, 7);
        println!("{:?}", shard);
        assert_eq!(shard, 4);
    }

    #[test]
    fn shards_5_3() {
        let shards = get_shards(2237, 5, 3);
        println!("{:?}", shards);
        assert_eq!(shards, vec![2, 4, 0]);
    }

    #[test]
    fn shards_5_5() {
        let shards = get_shards(2237, 5, 5);
        println!("{:?}", shards);
        assert_eq!(shards, vec![2, 4, 0, 3, 1]);
    }

    #[test]
    fn shards_7_3() {
        let shards = get_shards(2237, 7, 3);
        println!("{:?}", shards);
        assert_eq!(shards, vec![4, 1, 5]);
    }

    #[test]
    fn shards_7_7() {
        let shards = get_shards(2237, 7, 7);
        println!("{:?}", shards);
        assert_eq!(shards, vec![4, 1, 5, 3, 6, 0, 2]);
    }
}
