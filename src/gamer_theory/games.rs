use std::collections::HashMap;

struct Hackenbush {
    edges: HashMap<usize, Vec<usize>>

    // dict of int: node
    // where node has a list of edges that it is conneceted to
    // when a edge is removed it dose a dfs to try and find a root node
    // if it does not then it will do a dfs and remove all the nodes from depth to node that is oposit of the removed edge
    // this way we can garentie that each node has a edge conecting it to the roots
    // the node will just be a vec1 containg a tuple that is a int for the node it is connect to and the player who owns that edge.
}