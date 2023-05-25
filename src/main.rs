
// Given a list of integers, use a vector and return the median (when
// sorted, the value in the middle position) and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

fn main() {
    let mut v = [2,3,55,8,5,6,3,5,4,7,8,9,6,22,5,1,5,8,4,2,5,8,554,654,3,354,5,5,22,5,2,2365,53,2,2,2,5,35,33,35,35,5,5,1,4,7,5];
    v.sort();
    for i in v.iter() {
        println!("{}", i)
    }
}
