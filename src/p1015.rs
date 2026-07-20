use std::io;

pub fn main() {
    let mut linha1 = String::new();
    let mut linha2 = String::new();

    io::stdin().read_line(&mut linha1).unwrap();
    io::stdin().read_line(&mut linha2).unwrap();

    let p1: Vec<f32> = linha1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let p2: Vec<f32> = linha2
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let x1 = p1[0];
    let y1 = p1[1];
    let x2 = p2[0];
    let y2 = p2[1];

    let distancia = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    println!("{:.4}", distancia);
}