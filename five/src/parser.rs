use nom::{character::complete::{self, space1, digit1}, bytes::complete::{tag}, sequence::{tuple, preceded, delimited}, multi::{many1, separated_list1}, branch::alt};
#[allow(unused_imports)]
use nom::{IResult};

pub fn full_parse(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
  let (input, crate_stacks) = crate_stacks(input)?;
  let (input, _) = tag(" \n")(input)?;
  let (input, moves) = crate_moves(input)?;

 
  Ok((input, (crate_stacks, moves)))
}

#[derive(Debug)]
pub struct Move {
  pub count: u32,
  pub from: u32,
  pub to: u32,
}

fn crate_move(input: &str) -> IResult<&str, Move> {
  let (input, (count, from, to)) = tuple((
    preceded(tag("move "), complete::u32),
    preceded(tag(" from "), complete::u32),
    preceded(tag(" to "), complete::u32),
  ))(input)?;

  return Ok((input, Move { count, from, to}))
}

fn crate_moves(i: &str) -> IResult<&str, Vec<Move>> {
  many1(preceded(
    tag("\n"),
    crate_move,
  ))(i)
}

fn crate_marker(i: &str) -> IResult<&str, Option<&str>> {
  let (input, maybecrate) = alt((
    delimited(
      tag("["),
      complete::alpha1,
      tag("]"),
    ),
    tag("   "),
  ))(i)?;

  let crate_option = match maybecrate {
    "   " => None,
    value => Some(value),
  };

  Ok((input, crate_option))
}

fn crate_layer(input: &str) -> IResult<&str, Vec<Option<&str>>> {
  let (input, layer) = separated_list1(
    tag(" "),
    crate_marker,
  )(input)?;
  Ok((input, layer))
}

fn crate_stacks(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
  let (input, layers) = separated_list1(tag("\n"), crate_layer)(input)?;
  let (input, _) = tag("\n ")(input)?;
  let (input, _) = separated_list1(space1, digit1)(input)?;

  let transposed = transpose_crate_stacks(layers);
  Ok((input, transposed))
}

fn transpose_crate_stacks(horizontal_stacks: Vec<Vec<Option<&str>>>) -> Vec<Vec<&str>> {
  let mut vertical_stacks: Vec<Vec<&str>> = vec![];
  for _ in 0..horizontal_stacks[0].len() {
    vertical_stacks.push(vec![]);
  };

  for vec in horizontal_stacks.iter().rev() {
    for (i, c) in vec.iter().enumerate() {
      match c {
        Some(label) => vertical_stacks[i].push(label),
        None => {},
      };
    };
  };

  vertical_stacks
}
