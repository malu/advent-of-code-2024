fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");

    let total1 = part1::run(&input);
    println!("Total (part 1): {total1}");

    let total2 = part2::run(&input);
    println!("Total (part 2): {total2}");
}

mod part1 {
    #[derive(Copy, Clone)]
    enum Block {
        File { id: usize },
        Free,
    }

    pub fn run(input: &str) -> usize {
        let mut map: Vec<_> = input
            .chars()
            .enumerate()
            .flat_map(|(i, len)| {
                let Some(len) = len.to_digit(10) else {
                    return Vec::new();
                };

                if i % 2 == 0 {
                    vec![Block::File { id: i / 2 }; len as usize]
                } else {
                    vec![Block::Free; len as usize]
                }
            })
            .collect();

        let mut free_idx = 0;
        for i in (0..map.len()).rev() {
            if let Block::File { .. } = map[i] {
                match next_free(&map, free_idx) {
                    Some(n) if n < i => free_idx = n,
                    _ => break,
                }
                map.swap(i, free_idx);
            }
        }

        let mut total = 0;
        for (i, b) in map.iter().enumerate() {
            if let Block::File { id } = b {
                total += *id * i;
            }
        }

        total
    }

    fn next_free(map: &[Block], start: usize) -> Option<usize> {
        map.iter()
            .enumerate()
            .skip(start)
            .find_map(|(i, b)| matches!(b, Block::Free).then_some(i))
    }
}

mod part2 {
    #[derive(Copy, Clone)]
    enum Block {
        File { id: usize, len: usize, start: usize },
        Free { len: usize, start: usize },
    }

    pub fn run(input: &str) -> usize {
        let mut offset = 0;
        let mut map: Vec<_> = input
            .chars()
            .enumerate()
            .flat_map(|(i, len)| {
                let len = len.to_digit(10)?;

                let res = if i % 2 == 0 {
                    Some(Block::File {
                        id: i / 2,
                        len: len as usize,
                        start: offset,
                    })
                } else {
                    Some(Block::Free {
                        len: len as usize,
                        start: offset,
                    })
                };

                offset += len as usize;
                res
            })
            .collect();

        let mut free = Free::new();

        let mut total = 0;
        let mut i = map.len() - 1;
        while i > 0 {
            if let Block::File { len, id, start } = map[i] {
                match free.next(&map[0..i], len) {
                    Some(f) => {
                        if let Block::Free {
                            len: free_len,
                            start,
                            ..
                        } = &mut map[f]
                        {
                            for i in 0..len {
                                total += id * (*start + i);
                            }
                            *free_len -= len;
                            *start += len;
                        }
                    }
                    _ => {
                        for i in 0..len {
                            total += id * (start + i);
                        }
                    }
                }
            }

            i -= 1;
        }

        total
    }

    struct Free {
        next: [usize; 10],
    }

    impl Free {
        fn new() -> Self {
            Self { next: [0; 10] }
        }

        fn next(&mut self, map: &[Block], min_len: usize) -> Option<usize> {
            let res = map
                .iter()
                .enumerate()
                .skip(self.next[min_len])
                .find_map(|(i, b)| match b {
                    Block::Free { len, .. } if *len >= min_len => Some(i),
                    _ => None,
                });

            if let Some(i) = res {
                self.next[min_len..].iter_mut().for_each(|n| *n = i - 1);
            } else {
                self.next[min_len..]
                    .iter_mut()
                    .for_each(|n| *n = map.len() - 1);
            }

            res
        }
    }
}
