/// Solves for the sequence of moves required to move all discs from peg 1 to
/// peg 3, using peg 2 as an intermediary.
pub fn hanoi(num_discs: u32) -> Vec<(u8, u8)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut middle = Vec::new();
    let mut solution = Vec::new();
    for i in 1..num_discs {
        left.push(i);
    }
    move(num_discs, left, right, middle, solution)
}

fn move(n: u32, source: &Vec<u32>, target: &Vec<u32>, aux: &Vec<u32>, moves: &Vec<u32>) {
    move(n - 1, source, aux, target)
    // # move the nth disk from source to target
    target.append(source.pop())
    // # Display our progress
    // println!("{:?}, {:?}, {:?} {}, {}", sour, B, C, '##############', sep = '\n')

    // # move the n - 1 disks that we left on auxiliary onto target
    move(n - 1, aux, target, source)
}