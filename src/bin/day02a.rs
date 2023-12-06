use std::{
    io::stdin,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            draws: Vec::new(),
        }
    }

    fn _from(id: u32, draws: Vec<Draw>) -> Self {
        Self { id, draws }
    }

    fn exceeds(&self, limit: Draw) -> bool {
        self.draws.iter().any(|d| d.exceeds(limit))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn zero() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn rgb(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn exceeds(&self, limit: Draw) -> bool {
        self.red > limit.red || self.green > limit.green || self.blue > limit.blue
    }
}

impl AddAssign for Draw {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Add for Draw {
    type Output = Draw;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out += rhs;
        out
    }
}

pub fn main() {
    let limit = Draw::rgb(12, 13, 14);
    let sum = stdin().lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (s, game) = parser::game(&line).unwrap();
        assert_eq!(s, "");
        if game.exceeds(limit) {
            acc
        } else {
            acc + game.id
        }
    });
    println!("{}", sum);
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        combinator::map_res,
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };

    use crate::{Draw, Game};

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn from_dec(s: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(s, 10)
    }

    fn num(s: &str) -> IResult<&str, u32> {
        map_res(take_while(is_digit), from_dec)(s)
    }

    #[test]
    fn test_num() {
        assert_eq!(num("10"), Ok(("", 10)));
        assert_eq!(num("128"), Ok(("", 128)));
    }

    fn color(s: &str) -> IResult<&str, &str> {
        alt((tag("red"), tag("blue"), tag("green")))(s)
    }

    fn draw_term(s: &str) -> IResult<&str, Draw> {
        let (s, (count, color)) = separated_pair(num, tag(" "), color)(s)?;
        let term = match color {
            "red" => Draw::rgb(count, 0, 0),
            "green" => Draw::rgb(0, count, 0),
            "blue" => Draw::rgb(0, 0, count),
            _ => unreachable!(),
        };
        Ok((s, term))
    }

    #[test]
    fn test_draw_term() {
        assert_eq!(draw_term("4 red"), Ok(("", Draw::rgb(4, 0, 0))));
        assert_eq!(draw_term("8 green"), Ok(("", Draw::rgb(0, 8, 0))));
        assert_eq!(draw_term("5 blue"), Ok(("", Draw::rgb(0, 0, 5))));
    }

    fn draw(s: &str) -> IResult<&str, Draw> {
        let (s, terms) = separated_list0(tag(", "), draw_term)(s)?;
        let draw = terms.iter().fold(Draw::zero(), |acc, term| acc + *term);
        Ok((s, draw))
    }

    #[test]
    fn test_draw() {
        assert_eq!(draw("4 red, 8 green, 5 blue"), Ok(("", Draw::rgb(4, 8, 5))));
    }

    fn draws(s: &str) -> IResult<&str, Vec<Draw>> {
        separated_list0(tag("; "), draw)(s)
    }

    #[test]
    fn test_draws() {
        assert_eq!(
            draws("4 red; 8 green, 5 blue"),
            Ok(("", vec![Draw::rgb(4, 0, 0), Draw::rgb(0, 8, 5)]))
        );
    }

    fn game_header(s: &str) -> IResult<&str, Game> {
        let (s, _) = tag("Game ")(s)?;
        let (s, id) = num(s)?;
        let (s, _) = tag(": ")(s)?;
        let game = Game::new(id);
        Ok((s, game))
    }

    #[test]
    fn test_game_header() {
        assert_eq!(game_header("Game 3: "), Ok(("", Game::new(3))));
    }

    pub fn game(s: &str) -> IResult<&str, Game> {
        let (s, mut game) = game_header(s)?;
        let (s, draws) = draws(s)?;
        game.draws = draws;
        Ok((s, game))
    }

    #[test]
    fn test_game() {
        assert_eq!(
            game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            Ok((
                "",
                Game::_from(
                    3,
                    vec![Draw::rgb(20, 8, 6), Draw::rgb(4, 13, 5), Draw::rgb(1, 5, 0)]
                )
            ))
        );
    }
}
