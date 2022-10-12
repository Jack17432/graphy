use graphy::searchy;
use std::collections::HashMap;

#[test]
fn depth_first_search() {
    {
        use graphy::gamer_theory::games::Hackenbush;
        let graph1_edges = HashMap::from([
            (0, vec![(1, 0), (2, 0)]),
            (1, vec![(2, 0)]),
            (2, vec![(3, 0)]),
        ]);
        let graph1 = Hackenbush::from(graph1_edges);
        assert!(searchy::depth_first_search(&graph1, &3));
    }
}