/// Solves for the sequence of moves required to move all discs from peg 1 to
/// peg 3, using peg 2 as an intermediary.

enum Position {
    Left,
    Right,
    Middle
}

struct Tower {
    position: Position,
    disks: Vec<u32>
}

pub fn hanoi(num_discs: u32) -> Vec<(u8, u8)> {
    // Edge Cases
    let mut solution : Vec<(u8, u8)> = Vec::new();
    if num_discs == 0 {
        return solution;
    }
    if num_discs == 1 {
        solution.push((1, 3));
        return solution;
    }
    let mut left = Tower { position: Position::Left, disks: Vec::new() };
    let mut right = Tower { position: Position::Right, disks: Vec::new() };
    let mut middle = Tower { position: Position::Middle, disks: Vec::new() };
    for i in 1..num_discs {
        left.disks.push(i);
    }
    move_disk(num_discs, &mut left, &mut right, &mut middle, &mut solution);
    solution
}

fn move_disk(n: u32, source: &mut Tower, target: &mut Tower, aux: &mut Tower, moves: &mut Vec<(u8, u8)>) {
    if n > 0 {
        let mut _move = (1u8, 3u8);
        match source.position {
            Position::Left => _move.0 = 1,
            Position::Middle => _move.0 = 2,
            Position::Right => _move.0 = 3,
        }
        match target.position {
            Position::Left => _move.1 = 1,
            Position::Middle => _move.1 = 2,
            Position::Right => _move.1 = 3,
        }
        move_disk(n - 1, source, aux, target, moves);
        // # move the nth disk from source to target
        if let Some(x) = source.disks.pop() {
            target.disks.push(x);
        }
        moves.push(_move);
        // # Display our progress
        // println!("{:?}, {:?}, {:?} {}, {}", sour, B, C, '##############', sep = '\n')

        // # move the n - 1 disks that we left on auxiliary onto target
        move_disk(n - 1, aux, target, source, moves);
    }
}