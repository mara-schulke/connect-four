pub struct Field {
    tiles: Vec<Vec<u8>>
}

impl Field {
    pub fn new(cols : u8, rows : u8) -> Self {
        println!("called field constructor");

        let mut Tiles : Vec<Vec<u8>> = vec![];
        println!( "{:?}", Tiles );

        for y in 0..rows {
            Tiles.push(vec![]);

            for x in 0..cols {
                Tiles[y as usize].push(0); 
            }
        }

        println!( "{:?}", Tiles );

        Field {
            tiles: Tiles    
        }
    } 
}