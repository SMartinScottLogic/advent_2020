use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;
use log::info;
use regex::Regex;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let r = Regex::new(r"^Tile (?<tile>\d+):$").unwrap();
    let mut solution = Solution::default();
    let mut cur_tile = 0;
    for line in reader.lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(c) = r.captures(&line) {
            cur_tile = c.name("tile").unwrap().as_str().parse().unwrap();
            continue;
        }
        solution.add_tile_row(cur_tile, line);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    answer_part1: i64,
    answer_part2: i64,
    tiles: HashMap<i64, Vec<String>>,
}

impl Solution {
    pub fn add_tile_row(&mut self, cur_tile: i64, line: String) {
        assert_ne!(cur_tile, 0);
        self.tiles.entry(cur_tile).or_default().push(line);
    }

    pub fn analyse(&mut self) {
        let mut tile_edges: HashMap<String, Vec<i64>> = HashMap::new();

        for (tile_id, tile) in &self.tiles {
            let [top, bottom, left, right] = get_edges(tile);
            tile_edges.entry(top).or_default().push(*tile_id);
            tile_edges.entry(bottom).or_default().push(*tile_id);
            tile_edges.entry(left).or_default().push(*tile_id);
            tile_edges.entry(right).or_default().push(*tile_id);
        }
        info!("tile_edges: {:?}", tile_edges);
        let edges =
            tile_edges
                .iter()
                .fold(HashMap::<i64, usize>::new(), |mut acc, (_edge, ids)| {
                    if ids.len() == 1 {
                        *acc.entry(ids[0]).or_default() += 1;
                    }
                    acc
                });
        info!("edges? {:?}", edges);
        let corners: Vec<_> = edges
            .iter()
            .filter_map(|(id, c)| match c {
                &2 => Some(id),
                _ => None,
            })
            .collect();
        info!("corners {:?}", corners);
        assert_eq!(4, corners.len());
        self.answer_part1 = corners.iter().fold(1, |acc, v| acc * **v);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        Some(self.answer_part1)
    }

    pub fn answer_part2(&self) -> Option<i64> {
        None
    }
}

fn get_edges(tile: &[String]) -> [String; 4] {
    // Top
    let top = [
        tile.first().unwrap().to_owned(),
        tile.first().unwrap().chars().rev().collect(),
    ];
    // Bottom
    let bottom = [
        tile.last().unwrap().to_owned(),
        tile.last().unwrap().chars().rev().collect(),
    ];
    // Left
    let left: String = tile.iter().map(|s| s.chars().next().unwrap()).collect();
    let left = [left.to_owned(), left.chars().rev().collect()];
    // Right
    let right: String = tile
        .iter()
        .map(|s| s.chars().next_back().unwrap())
        .collect();
    let right = [right.to_owned(), right.chars().rev().collect()];

    let top = top.iter().k_smallest(1).next().unwrap().to_owned();
    let bottom = bottom.iter().k_smallest(1).next().unwrap().to_owned();
    let left = left.iter().k_smallest(1).next().unwrap().to_owned();
    let right = right.iter().k_smallest(1).next().unwrap().to_owned();

    [top, bottom, left, right]
}
