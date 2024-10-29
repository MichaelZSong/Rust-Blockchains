use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        let prev_hash = Hash::default();
        let generation = 0;
        let difficulty = difficulty;
        let data = String::new();
        let proof = None;

        Block {
            prev_hash,
            generation,
            difficulty,
            data,
            proof,
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        let prev_hash = previous.hash();
        let generation = previous.generation + 1;
        let difficulty = previous.difficulty;
        let data = data;
        let proof = None;

        Block {
            prev_hash,
            generation,
            difficulty,
            data,
            proof,
        }
    }

    // Return the hash string this block would have if we set the proof to `proof`
    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        let mut hash_string = String::new();

        write!(&mut hash_string, "{:02x}:", self.prev_hash).unwrap();  // Formats an array of bytes
        write!(&mut hash_string, "{}:", self.generation).unwrap();
        write!(&mut hash_string, "{}:", self.difficulty).unwrap();
        write!(&mut hash_string, "{}:", self.data).unwrap();
        write!(&mut hash_string, "{}", proof).unwrap();

        hash_string
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    // Return the block's SHA256 hash value as it would be if we set the proof to `proof`
    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // hash string with sha2 crate
        let mut hasher = Sha256::new();
        hasher.update(self.hash_string_for_proof(proof));

        hasher.finalize()
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    // Returns true if the block produces a hash that ends with .difficulty number of zero bits
    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        let hash = self.hash_for_proof(proof);  // Calculate hash value
        let n_bytes = self.difficulty / 8;
        let n_bits = self.difficulty % 8;

        // Check each of the last n_bytes bytes are 0u8
        for i in (hash.len() - n_bytes as usize)..hash.len() {
            if hash[i] != 0u8 {
                return false;
            }
        }

        // Check that the next byte from the end is divisible by 1 << n_bits
        if n_bits > 0 {
            let last_byte = hash[hash.len() - n_bytes as usize - 1];

            // Check if the last byte of the hash has the required number of zero bits
            if last_byte & ((1 << n_bits) - 1) != 0 {
                return false;
            }
        }

        true
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // With `workers` threads, check proof values in the given range, breaking up
	    // into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.
        
        let range_size = end - start + 1; // Check proof values for this block from start to end (inclusive)
        let chunk_size;
        let chunk_remain;
        let mut queue = WorkQueue::new(workers);

        // Prevent attempt to subtract with overflow
        if range_size < chunks {
            chunk_size = chunks;
            chunk_remain = 0;
        } else {
            chunk_size = range_size / chunks;
            chunk_remain = range_size % chunks;
        }

        // Dividing the work into chunks approximately-equal parts
        for i in 0..chunks {
            let start_proof = start + i * chunk_size;
            let chunk_end = start_proof + chunk_size - 1;
            let end_proof;
            
            if i == chunks - 1 {
                end_proof = chunk_end + chunk_remain;  // If last chunk, include remaining blocks
            } else {
                end_proof = chunk_end;
            }

            let task = MiningTask::new(sync::Arc::new(self.clone()), start_proof, end_proof);
            queue.enqueue(task).unwrap();
        }

        for proof in queue.iter() {
            // Stop checking proof values after a valid proof is found
            if self.is_valid_for_proof(proof) {
                queue.shutdown();
                return proof;  // Return valid proof
            }
        }

        0  // If no valid proof
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    start_proof: u64,
    end_proof: u64,
}

impl MiningTask {
    fn new(block: sync::Arc<Block>, start_proof: u64, end_proof: u64) -> MiningTask {
        MiningTask {
            block,
            start_proof,
            end_proof,
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        for proof in self.start_proof..=self.end_proof {
            if self.block.is_valid_for_proof(proof) {
                return Some(proof);  // If p is a valid proof
            }
        }

        None  // No valid proof was found
    }
}
