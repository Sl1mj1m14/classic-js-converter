use std::collections::HashMap;

use thiserror::Error;

use mc_classic_js::{ChangedBlocks, Data, JSLevel, Settings};
use mc_classic::Level;

#[derive(Error, Debug)]

pub enum ConversionError {
    #[error("Unable to Recognize World Size")]
    AmbiguousWorldSize()
}

pub fn classic_to_js (classic: Level, seed: i64, opt: u8) -> Result<Data,ConversionError> {
    let mut tile_map: Vec<u8> = Vec::new();

    if classic.blocks.is_some() { tile_map = classic.blocks.unwrap().clone() }

    //Replace these values with defaults from a config file
    println!("Reading world size");
    let mut x: i32 = if classic.width.is_some() { classic.width.unwrap() } else { 256 };
    let mut y: i32 = if classic.depth.is_some() { classic.depth.unwrap() } else { 64 };
    let mut z: i32 = if classic.height.is_some() { classic.height.unwrap() } else { 256 };

    //Handler for errors regarding world size
    if x*y*z != tile_map.len() as i32 && tile_map.len() > 0 {
        println!("World size is broken, attempting to fix!");
        let len: i32 = tile_map.len() as i32;
        if ((((len/y) as f64).sqrt()*10.0) as i32)%10 == 0 {
            x = ((len/y) as f64).sqrt() as i32;
            z = ((len/y) as f64).sqrt() as i32;
        } else if ((((len/64) as f64).sqrt()*10.0) as i32)%10 == 0 {
            y = 64;
            x = ((len/y) as f64).sqrt() as i32;
            z = ((len/y) as f64).sqrt() as i32;
        } else if len%y == 0 {
            let w: i32 = len/y;
            if w%x == 0 { z = w/x }
            else if w%z == 0 { x = w/z }
            else if w%64 == 0 { x = 64; z = w/x; }
            else if w%128 == 0 { x = 128; z = w/x; }
            else if w%256 == 0 { x = 256; z = w/x; }
            else if w%512 == 0 { x = 512; z = w/x; }
            else if w%1024 == 0 { x = 1024; z = w/x; }
            else { return Err(ConversionError::AmbiguousWorldSize()) }
        } else if len%64 == 0 { 
            y = 64;
            let w: i32 = len/y;
            if w%x == 0 { z = w/x }
            else if w%z == 0 { x = w/z }
            else if w%64 == 0 { x = 64; z = w/x; }
            else if w%128 == 0 { x = 128; z = w/x; }
            else if w%256 == 0 { x = 256; z = w/x; }
            else if w%512 == 0 { x = 512; z = w/x; }
            else if w%1024 == 0 { x = 1024; z = w/x; }
            else { return Err(ConversionError::AmbiguousWorldSize()) }
        } else {
            return Err(ConversionError::AmbiguousWorldSize())
        }
    }

    println!("Checking world height");
    //Containing the level to Y64
    if y > 64 && tile_map.len() > 0 {
        println!("Warning! Height is Restricted at 64 for Javascript Levels");
        println!("Shearing off {} levels from the world", y-64);
        for _ in 0..((y-64)*x*z) { _ = tile_map.pop() }
    }

    if y < 64 && tile_map.len() > 0 {
        println!("World Height is Restricted at 64 for Javascript Levels");
        println!("Adding {} levels of air to top of the world", 64-y);
        for _ in 0..((64-y)*x*z) { tile_map.push(0) }
    }

    y = 64;

    println!("Converting tile ids");
    //Converting tile ids to javascript tile ids
    let mut i: usize = 0;
    for tile in tile_map.clone() {
        match tile {
            1 => tile_map[i] = 2, //Stone
            2 => tile_map[i] = 1, //Grass Block
            3 => tile_map[i] = 3, //Dirt
            4 => tile_map[i] = 9, //Cobblestone
            5 => tile_map[i] = 4, //Planks
            6 => tile_map[i] = 8, //Sapling
            7 => tile_map[i] = 10, //Bedrock
            8 => tile_map[i] = 7, //Flowing Water
            9 => tile_map[i] = 7, //Stationary Water
            10 => tile_map[i] = 17, //Flowing Lava
            11 => tile_map[i] = 17, //Stationary Lava
            12 => tile_map[i] = 11, //Sand
            13 => tile_map[i] = 12, //Gravel
            14 => tile_map[i] = 18, //Gold Ore
            15 => tile_map[i] = 19, //Iron Ore
            16 => tile_map[i] = 20, //Coal Ore
            17 => tile_map[i] = 13, //Logs
            18 => tile_map[i] = 14, //Leaves
            19 => tile_map[i] = 22, //Sponge
            20 => tile_map[i] = 23, //Glass
            21 => tile_map[i] = 24, //Red Cloth
            22 => tile_map[i] = 25, //Orange Cloth
            23 => tile_map[i] = 26, //Yellow Cloth
            24 => tile_map[i] = 27, //Chartreuse Cloth
            25 => tile_map[i] = 28, //Green Cloth
            26 => tile_map[i] = 29, //Spring Green Cloth
            27 => tile_map[i] = 30, //Cyan Cloth
            28 => tile_map[i] = 31, //Capri Cloth
            29 => tile_map[i] = 32, //Ultramarine Cloth
            30 => tile_map[i] = 34, //Purple Cloth
            31 => tile_map[i] = 33, //Violet Cloth
            32 => tile_map[i] = 35, //Magenta Cloth
            33 => tile_map[i] = 36, //Rose Cloth
            34 => tile_map[i] = 37, //Dark Gray Cloth
            35 => tile_map[i] = 38, //Light Gray Cloth
            36 => tile_map[i] = 39, //White Cloth
            37 => tile_map[i] = 6, //Dandelion
            38 => tile_map[i] = 5, //Rose
            39 => tile_map[i] = 16, //Brown Mushroom
            40 => tile_map[i] = 15, //Red Mushroom
            41 => tile_map[i] = 21, //Block of Gold
            42 => tile_map[i] = 0, //Block of Iron
            43 => tile_map[i] = 0, //Double Slab
            44 => tile_map[i] = 0, //Slab
            45 => tile_map[i] = 0, //Bricks
            46 => tile_map[i] = 0, //TNT
            47 => tile_map[i] = 0, //Bookshelf
            48 => tile_map[i] = 0, //Mossy Cobblestone
            49 => tile_map[i] = 0, //Obsidian
            _ => tile_map[i] = 0,
        }
        i += 1;
    }

    //Converting the tile map to an array of changed blocks
    let mut changed_blocks: HashMap<String, mc_classic_js::ChangedBlocks> = HashMap::new();

    let world_size: i32 = if x >= z {x} else {z};
    let seed1: i64 = if seed > 0 {seed} else {1}; //Replace 1 with a default seed from a config file
    let tile_map1: Vec<u8> = mc_classic_js::get_tile_map(world_size, seed1);

    println!("Converting tiles to changed blocks");
    if tile_map.len() > 0 {
        for i in 0..y {
            for j in 0..z {
                for k in 0..x {
                    if (tile_map[((i*z*x) + (j*x) + k) as usize] != tile_map1[((i*z*x) + (j*x) + k) as usize]) {
                        let key: String = String::from(format!(r#""p{}_{}_{}":"#,k,i,j));
                        changed_blocks.insert(key, ChangedBlocks {a: 1, bt: tile_map[((i*z*x) + (j*x) + k) as usize]});
                    }
                }
            }
        }
    }

    println!("Writing javascript world");
    //Creating JSLevel object

    let level: JSLevel = JSLevel {
        worldSeed: seed1,
        changedBlocks: changed_blocks,
        worldSize: world_size,
        version: 1
    };

    //Grabbing Username
    println!("Grabbing username");
    let mut settings: Settings = Settings::default();
    if classic.creator.is_some() {settings.username = classic.creator.unwrap()}

    return Ok(Data {js_level: level, settings: settings})
}