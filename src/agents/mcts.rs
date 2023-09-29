use std::time::Instant;

use super::Agent;
use crate::components::{action::Action, board::Board, enums::WinningStatus, constants::TIME_LIMT};



type ValueType = f64;

const INF_VALUE: ValueType = 10000.0;

struct Node<'a> {
    pub board: Board<'a>,
    pub sum_w:  ValueType,
    pub child_nodes:    Vec<Node<'a>>,
    pub try_count:  u32,
}

impl<'a> Node<'a> {
    pub fn init(board: Board<'a>) -> Node<'a> {
        Node {
            board,
            sum_w:  0.,
            child_nodes:    vec![],
            try_count:  0,
        }
    }

    pub fn expand(& mut self) {
        let actions: Vec<Action> = self.board.legal_actions();
        for action in actions {
            let new_board = self.board.play_onestep(action);
            let add_node: Node = Node::init(new_board);
            self.child_nodes.push(add_node);
        }
    }

    pub fn evaluate(&mut self, expand_threshold: u32) -> ValueType {
        let winning_status = self.board.winning_status();
        if winning_status != WinningStatus::NotFinished {
            let value:ValueType = if winning_status == WinningStatus::Win {
                1.
            } else if winning_status == WinningStatus::Lose {
                0.
            } else {
                0.5
            };
            self.sum_w += value;
            self.try_count += 1;
            value
        } else if self.child_nodes.is_empty() {
            let value = self.get_playout_score();
            self.sum_w += value;
            self.try_count += 1;
            if self.try_count == expand_threshold {
                self.expand();
            }
            value
        } else {
            let index = self.next_child_node_index();
            let node = &mut self.child_nodes[index];
            let value = 1. - node.evaluate(expand_threshold);
            self.sum_w += value;
            self.try_count += 1;
            value
        }
    }

    fn get_playout_score(&self) -> ValueType {
        let winning_status = self.board.winning_status();
        if winning_status == WinningStatus::Win {
            1.
        } else if winning_status == WinningStatus::Lose {
            0.
        } else {
            0.5
        }
    }

    fn next_child_node_index(&self) -> usize {
        let mut t = 0;
        for (i, node) in self.child_nodes.iter().enumerate() {
            if node.try_count == 0 {
                return i;
            }
            t += node.try_count;
        }
        let mut best_value = -INF_VALUE;
        let mut best_index = 0;

        for (i, node) in self.child_nodes.iter().enumerate() {
            let ucb1_value: f64 = 1. - node.sum_w / node.try_count as ValueType
                                + 1. * (2. * (t as ValueType).ln() / node.try_count as ValueType).sqrt();
            if ucb1_value > best_value {
                best_index = i;
                best_value = ucb1_value;
            }
        }
        best_index
    }
}

pub struct MCTS {
    pub expand_threshold:  u32,
    pub rest_time: u128, // micro sec
}

impl Agent for MCTS {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        let now = Instant::now();
        let mut root_node: Node = Node::init(*board);
        root_node.expand();
        while now.elapsed().as_micros() + self.rest_time < TIME_LIMT {
            root_node.evaluate(self.expand_threshold);
        }
        let actions = board.legal_actions();
        let mut res_action = None;

        let mut most_try_count = 0;
        for i in 0..actions.len() {
            let count = root_node.child_nodes[i].try_count;
            if count > most_try_count {
                most_try_count = count;
                res_action = Some(actions[i]);
            }
        }

        res_action
    }
}