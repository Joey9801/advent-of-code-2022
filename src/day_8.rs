use std::collections::HashSet;

pub struct TreeMap {
    size: (u8, u8),
    heights: Vec<i8>,
}

impl TreeMap {
    fn row(&self, row_idx: u8) -> impl DoubleEndedIterator<Item=(i8, (u8, u8))> + '_ {
        self.heights
            .iter()
            .cloned()
            .skip(row_idx as usize * self.size.0 as usize)
            .take(self.size.1 as usize)
            .enumerate()
            .map(move |(x, h)| (h, (x as u8, row_idx)))
    }

    fn col(&self, col_idx: u8) -> impl DoubleEndedIterator<Item=(i8, (u8, u8))> + '_ {
        self.heights
            .iter()
            .cloned()
            .skip(col_idx as usize)
            .step_by(self.size.0 as usize)
            .take(self.size.1 as usize)
            .enumerate()
            .map(move |(y, h)| (h, (col_idx, y as u8)))
    }
    
    fn get(&self, x: u8, y: u8) -> i8 {
        self.heights[x as usize + y as usize * self.size.0 as usize]
    }
}

impl AsRef<TreeMap> for TreeMap {
    fn as_ref(&self) -> &TreeMap {
        self
    }
}

pub fn parse(input: &str) -> TreeMap {
    let size_x = input.lines().next().unwrap().len() as u8;
    let size_y = input.lines().count() as u8;
    
    let heights = input
        .chars()
        .filter_map(|x| x.to_digit(10).map(|x| x as i8))
        .collect();

    TreeMap {
        size: (size_x, size_y),
        heights,
    }
}

pub fn solve_part_1(input: &TreeMap) -> usize {
    fn find_peaks(visible: &mut HashSet<(u8, u8)>, x: impl Iterator<Item=(i8, (u8, u8))>) {
        let mut max = -1;
        
        for (tree, pos) in x {
            if tree > max {
                visible.insert(pos);
                max = tree;
            }
            
            if tree == 9 {
                break
            }
        }
    }
    
    let mut peaks = HashSet::with_capacity(input.size.0 as usize * input.size.1 as usize);

    for row in 0..input.size.1 {
        find_peaks(&mut peaks, input.row(row));
        find_peaks(&mut peaks, input.row(row).rev());
    }
    for col in 0..input.size.0 {
        find_peaks(&mut peaks, input.col(col));
        find_peaks(&mut peaks, input.col(col).rev());
    }

    peaks.len()
}

pub fn solve_part_2(input: &TreeMap) -> usize {
    fn score(map: &TreeMap, pos: (u8, u8)) -> usize {
        let vectors = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
        ];
        
        let mut distances = [0; 4];
        let pos_height = map.get(pos.0, pos.1);
        for (i, vec) in vectors.iter().enumerate() {
            let mut cursor = (pos.0 as i8, pos.1 as i8);
            cursor = (cursor.0 + vec.0, cursor.1 + vec.1);
            while cursor.0 >= 0 && cursor.0 < map.size.0 as i8 && cursor.1 >= 0 && cursor.1 < map.size.1 as i8 {
                distances[i] += 1;

                if map.get(cursor.0 as u8, cursor.1 as u8) >= pos_height {
                    break
                }

                cursor = (cursor.0 + vec.0, cursor.1 + vec.1);
            }
        }

        distances.iter().product()
    }
    
    (0..input.size.0)
        .flat_map(|x| (0..input.size.1).map(move |y| (x, y)))
        .map(|pos| score(input, pos))
        .max()
        .unwrap()
}