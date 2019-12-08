mod computer;

use std::io::{self, Read};

use anyhow::{anyhow, Result};
use async_std::{sync::channel, task};
use futures::future::join_all;
use itertools::Itertools;

use computer::Computer;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

fn parse_program(program: &str) -> Result<Vec<i32>> {
    Ok(program
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|val| Ok(val.trim().parse()?))
        .collect::<Result<Vec<i32>>>()?)
}

fn part1(input: &str) -> Result<i32> {
    Ok(task::block_on(max_thruster_signal(parse_program(input)?))?)
}

async fn get_thruster_signal(program: Vec<i32>, settings: &[i32]) -> Result<i32> {
    let (start_sender, start_receiver) = channel(10);
    let mut last_sender = start_sender.clone();
    let mut last_receiver = start_receiver;

    for phase in settings {
        last_sender.send(*phase).await;
        let mut computer = Computer::from_mem(program.clone());

        let (sender, receiver) = channel(10);

        computer.connect_input(last_receiver.clone());
        computer.connect_output(sender.clone());
        last_sender = sender;
        last_receiver = receiver;

        computer.spawn().await;
    }
    start_sender.send(0).await;
    Ok(last_receiver
        .recv()
        .await
        .ok_or(anyhow!("Could not get output"))?)
}

async fn get_thruster_signal_feedback(program: Vec<i32>, settings: &[i32]) -> Result<i32> {
    let (start_sender, start_receiver) = channel(10);
    let mut last_sender = start_sender.clone();
    let mut last_receiver = start_receiver.clone();

    let mut computers = Vec::new();

    for phase in settings {
        last_sender.send(*phase).await;
        let mut computer = Computer::from_mem(program.clone());

        let (sender, receiver) = channel(10);

        computer.connect_input(last_receiver.clone());
        computer.connect_output(sender.clone());
        last_sender = sender;
        last_receiver = receiver;
        computers.push(computer);
    }

    computers
        .last_mut()
        .unwrap()
        .connect_output(start_sender.clone());

    let tasks = join_all(computers.into_iter().map(|computer| computer.spawn())).await;

    start_sender.send(0).await;

    join_all(tasks).await;

    Ok(start_receiver
        .recv()
        .await
        .ok_or(anyhow!("Could not get output"))?)
}

async fn max_thruster_signal(program: Vec<i32>) -> Result<i32> {
    let values = join_all((0..5).permutations(5).map(|settings| {
        let program = program.clone();
        task::spawn(async move { get_thruster_signal(program, &settings).await })
    }))
    .await;

    Ok(values
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max()
        .unwrap())
}

async fn max_thruster_signal_feedback(program: Vec<i32>) -> Result<i32> {
    let values = join_all((5..=9).permutations(5).map(|settings| {
        let program = program.clone();
        task::spawn(async move { get_thruster_signal_feedback(program, &settings).await })
    }))
    .await;

    Ok(values
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max()
        .unwrap())
}

fn part2(input: &str) -> Result<i32> {
    Ok(task::block_on(max_thruster_signal_feedback(
        parse_program(input)?,
    ))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_get_thruster_signal() -> Result<()> {
        assert_eq!(
            get_thruster_signal(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                &[4, 3, 2, 1, 0]
            )
            .await?,
            43210
        );
        assert_eq!(
            get_thruster_signal(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                &[0, 1, 2, 3, 4]
            )
            .await?,
            54321
        );
        assert_eq!(
            get_thruster_signal(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                &[1, 0, 4, 3, 2]
            )
            .await?,
            65210
        );
        Ok(())
    }

    #[async_std::test]
    async fn test_get_thruster_signal_feedback() -> Result<()> {
        assert_eq!(
            get_thruster_signal_feedback(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                &[9, 8, 7, 6, 5]
            )
            .await?,
            139629729
        );
        assert_eq!(
            get_thruster_signal_feedback(
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                &[9, 7, 8, 5, 6]
            )
            .await?,
            18216
        );
        Ok(())
    }

    #[async_std::test]
    async fn test_max_thruster_signal() -> Result<()> {
        assert_eq!(
            max_thruster_signal(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ])
            .await?,
            43210
        );
        assert_eq!(
            max_thruster_signal(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ])
            .await?,
            54321
        );
        assert_eq!(
            max_thruster_signal(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ])
            .await?,
            65210
        );

        Ok(())
    }

    #[async_std::test]
    async fn test_max_thruster_signal_feedback() -> Result<()> {
        assert_eq!(
            max_thruster_signal_feedback(vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ])
            .await?,
            139629729
        );
        assert_eq!(
            max_thruster_signal_feedback(vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ])
            .await?,
            18216
        );

        Ok(())
    }
}
