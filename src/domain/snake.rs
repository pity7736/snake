use super::{direction::Direction, position::Position};


pub struct Snake {
    body: Vec<Position>,
    previous_tail_position: Position
}

impl Snake {

    pub fn new(head_position: Position) -> Self {
        let mut body: Vec<Position> = Vec::with_capacity(15);
        body.push(head_position);
        Self {body, previous_tail_position: head_position}
    }

    pub fn move_(&mut self, direction: Direction) {
        let body = self.body.clone();
        self.previous_tail_position = self.tail();
        self.body[0] = self.body[0].move_(direction);
        for i in 1..self.body.len() {
            self.body[i] = body[i - 1];
        }
    }

    pub fn eat(&mut self) {
        self.body.push(self.previous_tail_position);
    }

    pub fn head(&self) -> Position {
        return self.body[0];
    }

    pub fn body(&self) -> Vec<Position> {
        return self.body[1..].to_vec();
    }

    pub fn tail(&self) -> Position {
        return self.body[self.body.len() - 1];
    }

    pub fn has_crashed(&self) -> bool {
        for i in 1..self.body.len() {
            if self.body[0] == self.body[i] {
                return true;
            }
        }
        return false;
    }
}
