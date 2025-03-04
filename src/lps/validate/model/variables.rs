use std::collections::HashMap;

use crate::{graphs::get_n, DominoError, Puzzle, Tile};

// Struct representing a Variable with label, tile, and position.
#[derive(Debug, Clone)]
pub struct Variable {
    pub label: String,
    pub tile: (usize, usize),
    pub position: usize,
}

// Struct representing the Variables collection, with HashMaps for lookup by label, tile, and position.
#[derive(Debug, Default, Clone)]
pub struct Variables {
    pub(super) vars: Vec<Variable>,
    pub(super) by_label: HashMap<String, Variable>,
    pub(super) by_tile: HashMap<(usize, usize), Vec<Variable>>,
    pub(super) by_position: HashMap<usize, Vec<Variable>>,
}

impl Variables {
    pub fn new(combinations: Vec<Variable>) -> Self {
        let mut vars = Variables::default();

        for el in combinations {
            vars.insert(el);
        }

        vars
    }

    fn insert(&mut self, variable: Variable) {
        // Helper function to insert or update a map entry.
        fn insert_or_update<K: std::cmp::Eq + std::hash::Hash, V: Clone>(
            map: &mut HashMap<K, Vec<V>>,
            key: K,
            value: V,
        ) {
            map.entry(key).or_insert_with(Vec::new).push(value);
        }
        self.vars.push(variable.clone());
        self.by_label
            .insert(variable.label.clone(), variable.clone());
        insert_or_update(&mut self.by_tile, variable.tile, variable.clone());
        insert_or_update(&mut self.by_position, variable.position, variable);
    }
}

// Main function to create variables based on N.
pub fn variables(puzzle: &Puzzle) -> Result<Variables, DominoError> {
    let n = get_n(puzzle)? as usize;
    let tileset = create_tileset(n)
        .into_iter()
        .enumerate()
        .filter(|(_, tile)| {
            let tile = Tile(tile.0.try_into().unwrap(), tile.1.try_into().unwrap());
            !puzzle
                .iter()
                .any(|&puzzle_tile| puzzle_tile == Some(tile) || puzzle_tile == Some(tile.flip()))
        })
        .collect();
    let mapped_variables = generate_combinations(tileset, n)
        .into_iter()
        .filter(|var| puzzle.get(var.position).unwrap().is_none())
        .collect();
    Ok(Variables::new(mapped_variables))
}

// Function to create a tileset based on N
pub fn create_tileset(n: usize) -> Vec<(usize, usize)> {
    let length: usize = (n + 1).pow(2);
    let mut tileset: Vec<(usize, usize)> = (0..length)
        .map(|i| (i / (n + 1), i % (n + 1)))
        .collect::<Vec<(usize, usize)>>();

    if n % 2 == 1 {
        tileset.retain(|&(i, j)| {
            !(i <= j && j == i + (n + 1) / 2) && !(i > j && i == j + (n + 1) / 2)
        });
    }

    tileset
}

// Function to generate combinations of tiles and positions into Variables.
fn generate_combinations(tileset: Vec<(usize, (usize, usize))>, n: usize) -> Vec<Variable> {
    let sequence_length: usize = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1).pow(2) / 2
    };
    let tileset_length = sequence_length * 2;
    let tileset_digits = ((tileset_length.saturating_sub(1)) as f32).log10().floor() as usize + 1;
    let sequence_digits = ((sequence_length.saturating_sub(1)) as f32).log10().floor() as usize + 1;
    let positions: Vec<usize> = (0..sequence_length).collect::<Vec<usize>>();
    tileset
        .iter()
        .flat_map(|(tile_index, tile)| {
            positions.iter().map(move |&position| {
                let label: String = format!(
                    "x{}{}",
                    format_on_n_digits(*tile_index, tileset_digits),
                    format_on_n_digits(position, sequence_digits)
                );
                let variable = Variable {
                    label,
                    tile: *tile,
                    position,
                };
                variable
            })
        })
        .collect::<Vec<Variable>>()
}

// Helper function to format numbers with padding.
fn format_on_n_digits(number: usize, digits: usize) -> String {
    // format!("{:01}", number) // Modify as needed for digit padding.
    return format!("{:0width$}", number, width = digits);
}
