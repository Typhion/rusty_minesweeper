use std::io;
use rand::Rng;

mod field;

use field::field_module::Tile;
use crate::field::field_module::TileType;

fn fill_field(x: u8, y: u8, bomb_amount: u8) -> Vec<Vec<Tile>> {
    let mut y_counter = 0;
    let mut bomb_counter = 0;
    let mut field: Vec<Vec<Tile>> = vec![];
    while y_counter < y {
        let mut x_counter = 0;
        let mut x_field: Vec<Tile> = vec![];
        while x_counter < x {
            let mut tile_type = TileType::Empty;
            let random: u8 = rand::random();
            if random < u8::MAX / 4 && bomb_counter < bomb_amount {
                tile_type = TileType::Bomb;
                bomb_counter += 1;
            }
            x_field.push(Tile::new(tile_type, format!("{}{}", (x_counter + 65) as char, y_counter + 1)));
            x_counter += 1;
        }
        field.push(x_field);
        y_counter += 1;
    }
    return field;
}

fn main() {
    loop {
        println!("How large do you want your field?");
        let mut field_size = String::new();
        io::stdin().read_line(&mut field_size)
            .expect("Didn't receive input");
        if field_size == "x" {
            break;
        }
        println!("How many bombs?");
        let mut bomb_amount = String::new();
        io::stdin().read_line(&mut bomb_amount)
            .expect("Didn't receive input");
        if bomb_amount == "x" {
            break;
        }
        let field = fill_field(
            field_size.trim().parse().expect("Input not an integer"),
            field_size.trim().parse().expect("Input not an integer"),
            bomb_amount.trim().parse().expect("Input not an integer"),
        );

        print!("  ");
        for tile in &field[0] {
            print!("{} ", tile.key.chars().nth(0).unwrap())
        }

        println!();
        for tiles in &field {
            print!("{} ", tiles.get(0).unwrap().key.chars().nth(1).unwrap());
            for tile in tiles {
                print!("{} ", tile.to_string());
            }
            println!()
        }
    }
}
