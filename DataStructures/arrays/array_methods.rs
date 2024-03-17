// contain, sort explore here
fn main() {
    // contain
    does_contain();
    // sort
    sort_array();
}

fn does_contain() {
    let array_of_chars = ['a', 'b', 'c', 'd'];
    let contains_a = array_of_chars.contains(&'a');
    println!("{}", contains_a);

}

fn sort_array() {

    // unsorted number tuples
    let mut points = [(4, 0), (1, 3), (0, 0), (1, 2)];
    points.sort();
    println!("{:?}", points);
}
