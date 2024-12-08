mod day01;
mod day02;

fn main() {
    let in_str: String = day01::read_input();
    day01::solve_p1(&in_str);
    day01::solve_p2(&in_str);
}
