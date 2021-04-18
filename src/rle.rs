pub trait RLE {
    type Data;
    fn compress(&self) -> Vec<Run<Self::Data>>;
    fn decompress(runs: Vec<Run<Self::Data>>) -> Vec<Self::Data>;
}

pub type Run<T> = (T, usize);

impl<T> RLE for Vec<T>
where
    T: Clone + PartialEq,
{
    type Data = T;
    fn compress(&self) -> Vec<Run<Self::Data>> {
        let mut runs = vec![];
        if self.is_empty() {
            return runs;
        }
        let mut idx = 0;
        loop {
            let first = &self[idx];
            let run_length = self[idx..].iter().take_while(|&item| item == first).count();

            runs.push((first.clone(), run_length));

            idx += run_length;
            if idx > self.len() - 1 {
                break;
            }
        }
        runs
    }
    fn decompress(runs: Vec<Run<Self::Data>>) -> Vec<Self::Data> {
        runs.into_iter()
            .map(|(item, size)| vec![item; size])
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn singleton() {
        let data = "a".chars().collect::<Vec<_>>();
        assert_eq!(Vec::<char>::compress(&data), vec![('a', 1),]);
    }
    #[test]
    fn identity() {
        let data = "aabbccddaabbaa".chars().collect::<Vec<_>>();
        assert_eq!(Vec::<char>::decompress(data.compress()), data);
    }
    #[test]
    fn repeated_singleton() {
        let data = "aaaaaaaaaaaaa".chars().collect::<Vec<_>>();
        assert_eq!(Vec::<char>::compress(&data), vec![('a', 13),]);
    }
    #[test]
    fn empty_runs() {
        let data = "".chars().collect::<Vec<_>>();
        assert!(data.compress().is_empty());
    }
    #[test]
    fn empty_decompress() {
        assert!(Vec::<char>::decompress(vec![]).is_empty());
    }
    #[test]
    fn check_runs1() {
        let data = "aaaabbbbcccc".chars().collect::<Vec<_>>();
        assert_eq!(data.compress(), vec![('a', 4), ('b', 4), ('c', 4)]);
    }
    #[test]
    fn check_runs2() {
        let data = "aabbccddaabbaa".chars().collect::<Vec<_>>();
        assert_eq!(
            data.compress(),
            vec![
                ('a', 2),
                ('b', 2),
                ('c', 2),
                ('d', 2),
                ('a', 2),
                ('b', 2),
                ('a', 2)
            ]
        );
    }
}
