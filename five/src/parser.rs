use nom::{character::complete::{self, alpha1}, bytes::complete::tag, sequence::{tuple, preceded, delimited}, multi::{many1, separated_list1}, branch::alt};
#[allow(unused_imports)]
use nom::{IResult};


pub struct Move {
  count: u32,
  from: u32,
  to: u32,
}

fn crate_move(input: &str) -> IResult<&str, Move> {
  // let (input, _) = tag("move ")(input)?;
  // let (input, count) = complete::u32(input)?;

  let (input, (count, from, to)) = tuple((
    preceded(tag("move "), complete::u32),
    preceded(tag(" from "), complete::u32),
    preceded(tag(" to "), complete::u32),
  ))(input)?;

  return Ok((input, Move { count, from, to}))
}


pub fn crate_moves(i: &str) -> IResult<&str, Vec<Move>> {
  many1(crate_move)(i)
}

pub fn crate(i: &str) -> IResult

fn crate_layer(input: &str) -> IResult<&str, Vec<Option<&str>>> {
  separated_list1(
    tag(" "),
    alt((
      delimited(
        tag("["),
        alpha1,
        tag("]")
      ),
      tag("   "),
    )),
  )(input)
}

pub fn crate_stacks(input: &str) -> IResult<&str, Vec<Vec<&str>>> {

}
