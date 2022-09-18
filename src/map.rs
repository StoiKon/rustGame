use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

use std::env;
use std::fs;


pub struct Map{
    pub name:String,
    pub layout:String
    
}
impl Map{
    pub fn loadMap(na:String,path:String) -> Map {
        Map {name: na , layout: fs::read_to_string(path).expect("file input error")}
    }
    pub fn toString(&self){
        println!("{}",self.name);
        println!("{}",self.layout);
    }
   
        
}
pub fn test(){
    
    let level1:Map = Map::loadMap("level1".to_owned(),"levels/level1".to_owned());
    level1.toString();
}