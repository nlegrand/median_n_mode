
// Given a list of integers, use a vector and return the median (when
// sorted, the value in the middle position) and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

struct Median {
    pos: usize,
    value: i32,
}

fn median(v: Vec<i32>) -> Median {
    let length = v.len();
    let modulo = length % 2;
    if modulo != 0 { //even
        let median_pos = (length + 1) / 2 -1;
        return Median {
            pos: median_pos,
            value: v[median_pos],
        }
    }
    else { //odd
        let median_pos = length / 2 -1;
        return Median {
            pos: median_pos,
            value: v[median_pos],
        }
    }
}

fn main() {
    let mut v = vec![2i32,3,55,8,5,6,3,5,4,7,-1];
    v.sort();
    let mut pos = 0;
    for i in v.iter() {
        println!("{} -> {}", pos, i);
        pos = pos + 1
    }
    println!("v is {} long", v.len());
    let med = median(v);
    println!("Median is in pos {} with value {}", med.pos, med.value)
}
