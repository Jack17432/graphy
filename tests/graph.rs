#[cfg(test)]
mod nodes {
    use graphy::nodes::Node;

    #[test]
    fn create_node_new() {
        let node1 = Node::new(5, vec![]);

        assert_eq!(node1.data, 5);
        assert_eq!(node1.children.len(), 0);
    }

    #[test]
    fn create_node_data_string_new() {
        let node1 = Node::new(String::from("Hello world!"), vec![]);

        assert_eq!(node1.data, "Hello world!");
        assert_eq!(node1.children.len(), 0);
    }

    #[test]
    fn create_node_recursivly_new() {
        let node1 = Node::new(5, vec![]);
        let node2 = Node::new(node1, vec![]);

        assert_eq!(node2.data.data, 5);
        assert_eq!(node2.children.len(), 0);
    }

    #[test]
    fn create_node_from() {
        let node1 = Node::from(5);

        assert_eq!(node1.data, 5);
        assert_eq!(node1.children.len(), 0);
    }

    #[test]
    fn create_node_data_string_from() {
        let node1 = Node::from(String::from("Hello world!"));

        assert_eq!(node1.data, "Hello world!");
        assert_eq!(node1.children.len(), 0);
    }

    #[test]
    fn create_node_recursivly_from() {
        let node1 = Node::from(5);
        let node2 = Node::from(node1);

        assert_eq!(node2.data.data, 5);
        assert_eq!(node2.children.len(), 0);
    }

    #[test]
    fn get_data_from_children() {
        use std::rc::Rc;

        let node1 = Node::from(5);
        let node2 = Node::from(10);
        let node3 = Node::new(15, vec![Rc::clone(&node1), Rc::clone(&node2)]);

        assert_eq!(node3.children[0].data, node1.data);
        assert_eq!(node3.children[1].data, node2.data);
        assert_eq!(node3.children.len(), 2);
    }
}