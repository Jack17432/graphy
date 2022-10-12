#[cfg(test)]
mod hackenbush_test {
    use graphy::gamer_theory::games::{Hackenbush};

    #[test]
    fn get_edges() {
        let game1 = Hackenbush::new();

        assert_eq!(game1.get_root_edges(), &vec![]);
    }
}