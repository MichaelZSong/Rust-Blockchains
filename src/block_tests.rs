#[cfg(test)]
mod block_tests {
    use crate::block::Block;

    #[test]
    fn test_valid_proof() {
        let mut block = Block::initial(13);
        block.mine(1);
        // println!("{}", block.hash_string());
        assert_eq!(true, block.is_valid_for_proof(11881));

        let mut block1 = Block::next(&block, String::from("testing"));
        block1.mine(1);
        // println!("{}", block1.hash_string());
        assert_eq!(true, block1.is_valid_for_proof(16836));
    }

    #[test]
    fn test_invalid_proof() {
        let mut block = Block::initial(7);
        block.mine(1);
        // println!("{}", block.hash_string());
        assert_eq!(false, block.is_valid_for_proof(89898));
    }

    #[test]
    fn test_zero_difficulty() {
        let mut block = Block::initial(0);
        block.mine(1);
        // println!("{}", block.hash_string());
        assert_eq!(true, block.is_valid_for_proof(0));
    }
}
