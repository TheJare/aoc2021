use crate::utils::read_file;
use anyhow::{Context, Result};
use itertools::Itertools;

// https://adventofcode.com/2021/day/4

pub fn read_input(args: &crate::File) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = read_file(&args.file)?;
    let mut tokens = file.split_ascii_whitespace();
    let moves = tokens.next();
    let moves = moves
        .map(|line| line.split(",").flat_map(|v| v.parse::<i32>()).collect_vec())
        .context("File does not contain a set of moves")?;
    let board_entries = tokens.flat_map(|v| v.parse::<i32>()).collect_vec();
    Ok((moves, board_entries))
}

fn is_board_line_complete(board: &[i32], offset: usize, delta: usize) -> bool {
    (0..5).all(|i| board[offset + i * delta] < 0)
}

fn is_board_bingo(board: &[i32]) -> bool {
    (0..5).any(|i| is_board_line_complete(board, i * 5, 1) || is_board_line_complete(board, i, 5))
}

fn board_score(board: &[i32]) -> i32 {
    board.iter().filter(|&&t| t >= 0).sum()
}

#[allow(dead_code)]
fn print_board(board: &[i32]) {
    board.iter().chunks(5).into_iter().for_each(|t| {
        t.for_each(|&f| print!("{:3}", f));
        println!("");
    });
    println!("");
}

fn apply_draw(boards: &mut Vec<&mut [i32]>, draw: i32) {
    for b in boards.iter_mut() {
        for c in b.iter_mut() {
            if draw == *c {
                *c = -1;
            }
        }
    }
}

fn find_completed_board(boards: &Vec<&mut [i32]>) -> Option<i32> {
    boards
        .iter()
        .find_map(|board| is_board_bingo(board).then(|| board_score(&board)))
}

pub fn day4(args: &crate::File) -> Result<()> {
    let (moves, mut board_entries) = read_input(&args)?;
    let mut boards = board_entries.chunks_mut(25).collect_vec();

    let mut first_winner: Option<i32> = None;
    let result = moves
        .iter()
        .find_map(|&draw| {
            apply_draw(&mut boards, draw);
            let winner = find_completed_board(&boards).map(|score| score * draw);
            winner.and_then(|score| {
                first_winner = first_winner.or(winner);
                boards.retain(|b| !is_board_bingo(b));
                boards.is_empty().then(|| (first_winner.unwrap(), score))
            })
        })
        .context("Moves exhausted and some board was not complete")?;

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
