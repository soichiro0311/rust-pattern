mod gameObject;

#[cfg(test)]
mod tests {
    use crate::gameObject::*;
    #[test]
    fn test_rock_win_scissors() {
        let rock = Rock::new();
        let scissors = Scissors::new();
        assert_eq!(true, rock.beats(scissors));
    }

    #[test]
    fn test_scissors_lose_rock() {
        let rock = Rock::new();
        let scissors = Scissors::new();
        assert_eq!(false, scissors.beats(rock));
    }

    #[test]
    fn test_paper_win_rock() {
        let rock = Rock::new();
        let paper = Paper::new();
        assert_eq!(true, paper.beats(rock));
    }
}
