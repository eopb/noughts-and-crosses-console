#![feature(tool_lints)]
#![warn(clippy::pedantic)]
#![allow(clippy::single_match_else)]
extern crate ai_graph;
extern crate noughts_and_crosses_lib;
use ai_graph::Gene;
use noughts_and_crosses_lib::{GameBoard, Players, Winner};
use std::cmp::Ordering::Equal;

#[derive(Debug, Clone)]
struct GeneStorage {
    gene: Gene,
    score: f64,
}

fn main() {
    let mut scores = Vec::new();
    for _x in 0..1000 {
        let gene_tested = Gene::new_random_gene();
        let mut score_values = Vec::new();
        for _x in 0..10000 {
            let mut game_board = GameBoard::empty_board();

            loop {
                game_board = match game_board.random_placement(Players::Cross) {
                    Some(game_board) => game_board,
                    None => {
                        score_values.push(1);
                        break;
                    }
                };
                match game_board.has_someone_won() {
                    Winner::Nought => {
                        panic!("error 1");
                    }
                    Winner::Cross => {
                        score_values.push(0);
                        break;
                    }
                    Winner::None => (),
                };
                // game_board.draw_game_board(&GameMode::Spectate);
                game_board = match game_board.place_largest_empty(
                    &gene_tested.clone().output(&[1, 0, 1, 1, 1, 0, 0, 1, 1]),
                    Players::Nought,
                ) {
                    Some(game_board) => game_board,
                    None => {
                        score_values.push(1);
                        break;
                    }
                };
                // game_board.draw_game_board(&GameMode::Spectate);
                match game_board.has_someone_won() {
                    Winner::Nought => {
                        score_values.push(2);
                        break;
                    }
                    Winner::Cross => {
                        panic!("error 1");
                    }
                    Winner::None => (),
                };
            }
        }
        #[allow(clippy::cast_precision_loss)]
        scores.push(GeneStorage {
            gene: gene_tested.clone(),
            score: score_values.iter().cloned().map(f64::from).sum::<f64>() as f64
                / score_values.len() as f64,
        });
    }
    // println!("{:#?}", scores);
    let mut score_val_temp = Vec::new();
    for score in &scores {
        score_val_temp.push(score.score)
    }
    // println!("{:#?}", score_val_temp);
    score_val_temp.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    // println!("{:#?}", score_val_temp);
    println!("Big value{:#?}", score_val_temp[score_val_temp.len() - 1]);
    println!("Hello, world!");
}
