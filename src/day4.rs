use crate::utils::read_file;
use anyhow::{Context, Result};
use itertools::Itertools;

// https://adventofcode.com/2021/day/4

pub fn read_input(args: &crate::File) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = read_file(&args.file)?;
    let mut nums = file.split_ascii_whitespace();
    let moves = nums.next();
    let moves = moves
        .map(|line| line.split(",").flat_map(|v| v.parse::<i32>()).collect_vec())
        .context("File does not contain a set of moves")?;
    let boards = nums.flat_map(|v| v.parse::<i32>()).collect_vec();
    Ok((moves, boards))
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
    for board in boards.iter() {
        if is_board_bingo(board) {
            return Some(board_score(&board));
        }
    }
    None
}

pub fn day4(args: &crate::File) -> Result<()> {
    let (moves, mut boards) = read_input(&args)?;
    let mut boards = boards.chunks_mut(25).collect_vec();

    let mut moves_iter = moves.iter();
    let mut first_winner: Option<i32> = None;
    let result = moves_iter
        .find_map(|&draw| {
            apply_draw(&mut boards, draw);
            let winner = find_completed_board(&boards).map(|score| score * draw);
            winner.and_then(|score| {
                if first_winner.is_none() {
                    first_winner = winner;
                }
                boards.retain(|b| !is_board_bingo(b));
                if boards.is_empty() {
                    Some((first_winner.unwrap(), score))
                } else {
                    None
                }
            })
        })
        .context("Moves exhausted and some board was not complete")?;

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
