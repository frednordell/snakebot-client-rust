use crate::{
    client::Player,
    types::{Direction, InboundMessage, Map},
    utils::{ Coordinate, Tile },
};
use log::debug;
use std::ops::Add;
use std::cmp;
use std::collections::HashSet;


#[derive(Debug, Clone)]
pub struct Snake {}

impl Snake {
    pub fn new() -> Snake {
        Snake { }
    }
}
 
impl Tile<'_> {
    fn value(&self) -> i32 {
        match *self {
            Tile::Wall => -200,
            Tile::Food { coordinate: Coordinate } => 500,
            Tile::Obstacle {coordinate: Coordinate } => -150,
            Tile::Empty { coordinate: Coordinate } => 200,
            Tile::SnakeHead { coordinate: Coordinate, snake: SnakeInfo } => -50,
            Tile::SnakeBody { coordinate: Coordinate, snake: SnakeInfo } => -250,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl Player for Snake {
    fn get_next_move(&mut self, map: &Map, player_id: &str) -> Direction {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let snake_info = map.get_snake_by_id(&player_id).unwrap();

        let positions = snake_info.positions.iter()
            .map(|pos| Coordinate::from_position(*pos, map.width))
            .collect::<Vec<_>>();

        let head = positions.iter().find(|&&pos| map.get_tile_at(pos).value() == -50).unwrap();

        let mut scores = DIRECTIONS.iter()
            .filter(|&&dir| map.can_snake_move_in_direction(snake_info, dir))
            .map(|&dir|
                positions.first().unwrap().add(dir.to_movement_delta())
            )
            .map(|coordinate| 
                (Self::score_direction(coordinate, map, *head, &mut visited), coordinate)
            )
            .collect::<Vec<(_, _)>>();

        scores.sort_by(|a, b| b.0.cmp(&a.0));
        debug!("{:?}", scores.first().unwrap_or(&(100, Direction::Up.to_movement_delta())).1.sub(*head).from_movement_delta());
        scores.first().unwrap_or(&(100, Direction::Up.to_movement_delta())).1.sub(*head).from_movement_delta()
    }

    fn score_direction(coordinate: Coordinate, map: &Map, head: Coordinate, visited: &mut HashSet<Coordinate>) -> i32 {
        if coordinate.euclidian_distance_to(head) > 50.0 || !visited.contains(&coordinate) {
            debug!("base Case");
            let adjacent = 
                map.get_tile_at(coordinate.add(Direction::Up.to_movement_delta())).value()
                + map.get_tile_at(coordinate.add(Direction::Down.to_movement_delta())).value()
                + map.get_tile_at(coordinate.add(Direction::Left.to_movement_delta())).value()
                + map.get_tile_at(coordinate.add(Direction::Right.to_movement_delta())).value();
            map.get_tile_at(coordinate).value() + adjacent/4
        } else {
            debug!("branch");
            visited.insert(coordinate);
            if !map.inside_map(coordinate) {
                return i32::min_value();
            }
            let adjacent = 
                map.get_tile_at(coordinate.add(Direction::Up.to_movement_delta())).value()
                + map.get_tile_at(coordinate.add(Direction::Down.to_movement_delta())).value()
                + map.get_tile_at(coordinate.add(Direction::Left.to_movement_delta())).value()
                + map.get_tile_at(coordinate.add(Direction::Right.to_movement_delta())).value();
            let current_tile_value = map.get_tile_at(coordinate).value() + adjacent/4;
            
            let left = Self::score_direction(coordinate.add(Direction::Left.to_movement_delta()), map, head, visited);
            let right = Self::score_direction(coordinate.add(Direction::Right.to_movement_delta()), map, head, visited);
            let up = Self::score_direction(coordinate.add(Direction::Up.to_movement_delta()), map, head, visited);
            let down = Self::score_direction(coordinate.add(Direction::Down.to_movement_delta()), map, head, visited);
            
            current_tile_value + cmp::max(cmp::max(left, up), cmp::max(right, down))
        }
    }

    fn on_message(&mut self, message: &InboundMessage) {
        if let InboundMessage::GameStarting { .. } = message {
            // Reset snake state here
        }
    }
}
