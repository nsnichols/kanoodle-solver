use colored::Colorize;

mod shapes;
mod board;

fn main() {
    let mut board = board::Board::new();

    let shape_e = shapes::shape_e();
    let shape_g = shapes::shape_g();
    let shape_j = shapes::shape_j();
    let shape_i = shapes::shape_i();
    let shape_a = shapes::shape_a();
    let shape_c = shapes::shape_c();
    let shape_d = shapes::shape_d();
    let shape_l = shapes::shape_l();
    let shape_h = shapes::shape_h();
    let shape_b = shapes::shape_b();
    let shape_f = shapes::shape_f();
    let shape_k = shapes::shape_k();

    board = board.add_shape(&shape_e);
    board = board.add_shape(&shape_g);
    board = board.add_shape(&shape_j);
    board = board.add_shape(&shape_i);
    board = board.add_shape(&shape_a);
    board = board.add_shape(&shape_c);
    board = board.add_shape(&shape_d);
    board = board.add_shape(&shape_l);
    board = board.add_shape(&shape_h);
    board = board.add_shape(&shape_b);
    board = board.add_shape(&shape_f);
    board = board.add_shape(&shape_k);

    println!("{}", board);
}


