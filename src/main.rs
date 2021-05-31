use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use velcro::hash_map;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Fill {
    Empty,
    Half,
    Solid,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Shape {
    Circle,
    Diamond,
    Squiggle,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Number {
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Orange,
    Green,
    Purple,
}

struct Card {
    fill: Fill,
    shape: Shape,
    number: Number,
    color: Color,
}

impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{:?} {:?} {:?} {:?}",
            self.color, self.fill, self.number, self.shape
        )
    }
}

fn same_or_different<T: PartialEq>(arr: &[T]) -> bool {
    arr.iter().all_equal() // || arr.iter().all_unique()
}

fn is_match(arr: &[Card]) -> bool {
    let colors: Vec<_> = arr.iter().map(|x| &x.color).collect();
    let shapes: Vec<_> = arr.iter().map(|x| &x.shape).collect();
    let numbers: Vec<_> = arr.iter().map(|x| &x.number).collect();
    let fills: Vec<_> = arr.iter().map(|x| &x.fill).collect();
    same_or_different(&colors)
        && same_or_different(&shapes)
        && same_or_different(&numbers)
        && same_or_different(&fills)
}

fn main() {
    println!("{:?}", Shape::Squiggle);
    println!("{:?}", Color::Orange);
}
