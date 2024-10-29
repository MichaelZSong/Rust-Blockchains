use a3::block::Block;

fn main() {
    // Nothing is required here, but it may be useful for testing.
    
    /* Check initial() and next() */
    // let mut b0 = Block::initial(19);
    // b0.mine(1);
    // // b0.set_proof(56231);
    // println!("{}", b0.hash_string());

    // let mut b1 = Block::next(&b0, String::from("message"));
    // b1.mine(1);
    // // b1.set_proof(2159);
    // println!("{}", b1.hash_string());

    /* Check is_valid_for_proof() */
    // let mut b0 = Block::initial(16);
    // println!("{:?}", b0.is_valid_for_proof(0));  // false
    // println!("{:?}", b0.is_valid_for_proof(56231));  // true
    // println!("{:?}", b0.is_valid_for_proof(1407891));  // false
    // b0.set_proof(87745);
    // let mut b1 = Block::next(&b0, String::from("hash example 1234"));
    // b1.set_proof(1407891);
    // println!("{}", b1.hash_string());

    /* Check valid hashes being mined */
    let mut b0 = Block::initial(20);
    b0.mine(1);
    println!("{}", b0.hash_string());
    println!("{:02x}", b0.hash());
    let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
    b1.mine(1);
    println!("{}", b1.hash_string());
    println!("{:02x}", b1.hash());
    let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    b2.mine(1);
    println!("{}", b2.hash_string());
    println!("{:02x}", b2.hash());
}
