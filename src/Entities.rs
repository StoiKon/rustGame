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

pub struct Entity{
    pub hp:i32,
    pub dmg:i32,
    pub armour:i32,
    pub name:String,
    pub x:i32,
    pub y:i32
}
impl Entity{
    pub fn new(n:String,h:i32,dmg:i32,armour:i32,x:i32,y:i32) ->Entity{
        return Entity{hp:h,dmg:dmg,armour:armour,name:n,x:x,y:y};
    }
    pub fn toString(&self){
        println!("hp {} dmg {} armour {} name {} x {} y {}",self.hp,self.dmg,self.armour,self.name,self.x,self.y);
    }
    pub fn getHit(&mut self,amount:i32){
        if self.hp >0{
        self.hp= self.hp - (nonNegative(amount -(self.armour)));
        }else{
            self.hp=0;
        }
    }

}
fn nonNegative(x:i32) -> i32{
    if x<0{
        return 0;
    }
    return x;
}
