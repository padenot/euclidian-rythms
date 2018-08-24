extern crate smallvec;

use smallvec::SmallVec;

pub fn euclidian_rythm(p: &mut [u8], pulses: usize) -> Result<(), &'static str> {
    let steps = p.len();
    let mut pattern = SmallVec::from_slice(p);
    pattern.clear();

    if pulses > steps {
        return Err("more pulses than steps.");
    }

    let mut divisor = steps - pulses;

    let mut level = 0;
    let mut counts = SmallVec::<[usize; 64]>::new();
    let mut remainders = SmallVec::<[usize; 64]>::new();

    remainders.push(pulses);

    // Run the euclid algorithm, store all the intermediate results
    loop {
        counts.push(divisor / remainders[level]);
        let r = remainders[level];
        remainders.push(divisor % r);

        divisor = remainders[level];
        level += 1;

        if remainders[level] <= 1 {
            break;
        }
    }
    counts.push(divisor);

    // Build the pattern
    fn build(
        counts: &[usize],
        pattern: &mut SmallVec<[u8; 64]>,
        remainders: &[usize],
        level: isize,
    ) {
        if level == -1 {
            pattern.push(0);
        } else if level == -2 {
            pattern.push(1);
        } else {
            for _ in 0..counts[level as usize] {
                build(counts, pattern, remainders, level - 1);
            }
            if remainders[level as usize] != 0 {
                build(counts, pattern, remainders, level - 2);
            }
        }
    }

    build(
        counts.as_slice(),
        &mut pattern,
        remainders.as_slice(),
        level as isize,
    );

    // Put a 1 on the first step
    let index_first_one = pattern.iter().position(|&x| x == 1).unwrap();
    pattern.rotate_left(index_first_one);

    p.copy_from_slice(&pattern);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use euclidian_rythm;
    #[test]
    fn it_works() {
        let mut pattern = [0 as u8; 8];
        let pulses = 4;
        euclidian_rythm(&mut pattern, pulses).unwrap();
        println!("{:?}", pattern);

        let mut pattern = [0 as u8; 9];
        let pulses = 3;
        euclidian_rythm(&mut pattern, pulses).unwrap();
        println!("{:?}", pattern);

        let mut pattern = [0 as u8; 12];
        let pulses = 7;
        euclidian_rythm(&mut pattern, pulses).unwrap();
        println!("{:?}", pattern);

        let mut pattern = [0 as u8; 13];
        let pulses = 5;
        euclidian_rythm(&mut pattern, pulses).unwrap();
        println!("{:?}", pattern);

        let mut pattern = [0 as u8; 31];
        let pulses = 7;
        euclidian_rythm(&mut pattern, pulses).unwrap();
        println!("{:?}", pattern);
    }
}
