use std::collections::HashMap;


pub enum Mark {
    Flagged,
    NotFlagged,
}

pub enum Status {
    Open,
    Closed(Mark),
}

#[derive(PartialEq)]
pub enum Vsebina {
    Stevilo(u8),
    Mina,
}

pub struct Tile {
    vsebina: Vsebina,
    status: Status,
    // mesto: (u16, u16)
}

pub struct Mreza {
    velikost: (u16, u16),
    tiles: HashMap<(u16, u16), Tile>,
}



impl Tile {
    pub fn vsebina(&self) -> &Vsebina {
        &self.vsebina
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
    
}

use crate::strukture::Vsebina::Mina;

impl Mreza {
    pub fn tile(&self, i: u16, j: u16) -> Option<&Tile> {
        self.tiles.get(&(i, j))
    }
    pub fn add_tile(&mut self, tile: Tile, i: u16, j: u16) -> Option<Tile> {
        self.tiles.insert((i, j), tile)
    }
    pub fn mines(&self) -> Vec<(u16, u16)> {
        let mut mine_vec = vec![];
        for ((i, j), tile) in self.tiles.iter() {
            if tile.vsebina == Mina {
                mine_vec.push((*i, *j));
            }
        }
        return mine_vec;
    }
    pub fn sosedje(&self, i: u16, j: u16) -> Vec<(u16, u16)> {
        let mut mozni = vec![(i - 1, j - 1), (i, j - 1), (i + 1, j - 1),
                         (i - 1, j), (i + 1, j),
                         (i - 1, j + 1), (i, j + 1), (i + 1, j + 1)];
        let keys: Vec<&(u16, u16)>  = self.tiles.keys().collect();
        mozni.retain(|n| keys.contains(&n));
        return mozni;
    }
    pub fn pripisi_stevilo(&self, i: u16, j: u16) -> u8 {
        2 //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn poizkus() {
        let test_mreza = Mreza {
            velikost: (123, 456),
            tiles: HashMap::new()
        };
       
    }
}


    // fn preverjaj(&self) -> bool {
    //     match self.vsebina {

    //     }
    // }