use std::collections::HashMap;
use super::super::nodes::Children;

#[derive(Debug)]
pub struct Hackenbush {
    /// {start_Node: (end_Node, player)}
    /// No owning player if player == 0
    /// blue player if player == 1
    /// red player if player == 2
    edges: HashMap<usize, Vec<(usize, u8)>>
}

impl Hackenbush {
    pub fn new() -> Self {
        Self {
            edges: HashMap::from([
                (0, vec![]),
            ])
        }
    }

    pub fn from(edges: HashMap<usize, Vec<(usize, u8)>>) -> Self {
        assert!(edges.get(&0).is_some(), "must contain a 'root' node == 1");

        let mut game = Self::new();
        game.edges = edges;
        game
    }

    pub fn get_root_edges(&self) -> &Vec<(usize, u8)> {
        &self.edges[&0]
    }
}

impl Children for Hackenbush {
    fn children(&self, node: usize) -> Option<&Vec<(usize, u8)>> {
        self.edges.get(&node)
    }
}

#[cfg(test)]
mod hackenbush_unit {
    use super::Hackenbush;
    use std::collections::HashMap;

    #[test]
    fn new_hackenbush() {
        let game1 = Hackenbush::new();
        let game1_edges = HashMap::from([
            (0, vec![]),
        ]);

        assert_eq!(game1.edges, game1_edges)
    }
}