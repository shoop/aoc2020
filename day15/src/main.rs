use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::vec::Vec;

fn star_one(starting: &Vec<isize>, until: isize) -> isize {
    let mut memory: HashMap<isize, isize> = HashMap::new();
    let mut time: isize = 1;
    let mut s = starting.iter();
    let mut to_say: isize = *s.next().unwrap();
    let mut cur = to_say;

    while time <= until {
        cur = to_say;
        time += 1;

        match s.next() {
            Some(val) => {
                // Starting sequence, just say it next time
                to_say = *val;
            },
            None => {
                // Check memory of last spoken to determine next thing to say
                to_say = match memory.entry(cur) {
                    Vacant(_) => {
                        // Not yet spoken, say 0 next
                        0
                    },
                    Occupied(entry) => {
                        time - entry.get()
                    },
                };
            }
        }

        memory.insert(cur, time);
    }

    cur
}

fn main() {
    let starting: Vec<isize> = vec![0,13,16,17,1,10,6];

    let ans = star_one(&starting, 2020);
    println!("Star one: {}", ans);

    let ans = star_one(&starting, 30000000);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_star_one() {
        let test_data: Vec<isize> = vec![0,3,6];
        let ans = super::star_one(&test_data, 2020);
        assert_eq!(ans, 436);
    }

    #[test]
    fn test_star_two() {
        let test_data: Vec<isize> = vec![0,3,6];
        let ans = super::star_one(&test_data, 30000000);
        assert_eq!(ans, 175594);
    }
}
