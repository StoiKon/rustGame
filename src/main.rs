use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::ptr::NonNull;
use std::time::Duration;
use std::fs;
use std::thread;


const PLAYER_MOVEMENT_SPEED: i32 = 20;

mod map;
mod Entities;

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    playerEntity: Entities::Entity 
}
struct Animation{
    name:String,
    duration:i32,
    x:i32,
    y:i32
}

fn render(
    
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
    map: &mut map::Map,
    tiles: &[Texture],
    entities:&mut Vec<Entities::Entity>,
    entities_tex:&[Texture],
    animations:&mut Vec<Animation>,
    animationTex:&[Texture]
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position =  Point::new(950, 500);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
    
    let mut x=0;
    let mut y=0;
    let size =100;
    for c in map.layout.chars(){

        if c=='g'{
            canvas.copy(&(tiles[0]),Rect::new(0,0,200,200) , Rect::new(x*100 +950 - player.position.x,y *100 +500 -player.position.y ,size,size))?;
        }

        if c=='u'{
            canvas.copy(&(tiles[1]),Rect::new(0,0,200,200) , Rect::new(x*100 + 950 - player.position.x,y*100 +500 -player.position.y ,size,size))?;
        }

        if c=='d'{
            canvas.copy(&(tiles[2]),Rect::new(0,0,200,200) ,Rect::new(x*100 + 950 - player.position.x,y*100 +500 -player.position.y ,size,size) )? ;
        }

        if c=='f'{
            canvas.copy(&(tiles[3]),Rect::new(0,0,200,200) ,Rect::new(x*100 + 950 - player.position.x,y*100 +500 -player.position.y ,size,size) )?;
        }

        if c=='w'{
            canvas.copy(&(tiles[4]),Rect::new(0,0,200,200) ,Rect::new(x*100 + 950 - player.position.x,y*100 +500 -player.position.y ,size,size) )?;
        }
        
        x+=1;
        if c=='\n'{
            y+=1;
            x=0;
        }
        //println!("{} {}",x,y);
    }
    for en in entities{
        //en.toString();
        if en.hp>0{
            if en.name=="barbarian"{
                canvas.copy(&(entities_tex[0]),Rect::new(0,0,200,200) , Rect::new(en.x +950 -player.position.x,en.y+500-player.position.y,size,size));
            }
            if en.name=="barbarian2"{
                canvas.copy(&(entities_tex[1]),Rect::new(0,0,200,200) , Rect::new(en.x +950 -player.position.x,en.y+500-player.position.y,size,size));
            }
            if en.name=="barbarianK"{
                canvas.copy(&(entities_tex[2]),Rect::new(0,0,200,200) , Rect::new(en.x +950 -player.position.x,en.y+500-player.position.y,size,size));
            }
            if en.name=="hoplite"{
                canvas.copy(&(entities_tex[9]),Rect::new(0,0,200,200) , Rect::new(en.x +950 -player.position.x,en.y+500-player.position.y,size,size));
            }
        }
    }
    canvas.copy(texture, player.sprite, screen_rect)?;
    
    
    if !animations.is_empty() && animations[0].duration<=0{
        animations.remove(0);
    }
    for animation in animations{
        if animation.name == "slash" && animation.duration>0{
            canvas.copy(&(animationTex[0]), Rect::new(0,0,300,300), Rect::new(animation.x +950 -player.position.x,animation.y+500-player.position.y,size,size));
            animation.duration=animation.duration-1;
        }
    }
    
    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.fill_rect(Rect::new(40,50,player.playerEntity.hp as u32,10));
    canvas.present();


    Ok(())
}

// Update player a fixed amount based on their speed.
// WARNING: Calling this function too often or at a variable speed will cause the player's speed
// to be unpredictable!
fn update_player(player: &mut Player,map:&String,ents:&mut Vec<Entities::Entity>,animations:&mut Vec<Animation>) {
    let lines: Vec<&str> = map.split('\n').collect();
    
    
    let mut y = player.position.y/100 ;
    let mut x:i32 = player.position.x/100;
    //println!("{} {}",x,y);
    let mut chars : Vec<_>= lines[y as usize].chars().collect();
    let mut tile = chars[x as usize];
    //println!("{}",tile);
    use self::Direction::*;
    match player.direction {
        Left => {
            
            x =(player.position.x - player.speed)/100 ;
            tile=chars[x as usize];
            if(tile != 'w'){
                player.position = player.position.offset(-player.speed, 0);
            }
        },
        Right => {
             x =(player.position.x + player.speed)/100 ;
            
             if(tile != 'w'){
                player.position = player.position.offset(player.speed, 0);
            }
        },
        Up => {
            
            y =(player.position.y - player.speed)/100 ;
            chars= lines[y as usize].chars().collect();
            tile=chars[x as usize];
            if(tile != 'w'){
            player.position = player.position.offset(0, -player.speed);
            }
        },
        Down => {
            
            y =(player.position.y + player.speed)/100 ;
            chars= lines[y as usize].chars().collect();
            tile=chars[x as usize];
            if(tile != 'w'){
            player.position = player.position.offset(0, player.speed);
            }
        },
        
    }
    for ent in ents{
        let distance =  f64::sqrt((player.position.x - ent.x).pow(2) as f64 + (player.position.y - ent.y).pow(2) as f64);
        if distance <60.0{
            if(ent.hp>0){
            ent.getHit(player.playerEntity.dmg);
            
            animations.push(Animation{name:"slash".to_string(),x:ent.x,y:ent.y,duration:2})
            }
        }
    }

}
fn updateAI(entities:&mut Vec<Entities::Entity>,player: &mut Player,map:&String,animations:&mut Vec<Animation>){
    let speed =4;
    for ent in entities{
        let distance =  f64::sqrt((player.position.x - ent.x).pow(2) as f64 + (player.position.y - ent.y).pow(2) as f64);
        if distance <600.0{
            if player.position.x < ent.x{
                ent.x=ent.x-speed;
            }
            if player.position.x > ent.x{
                ent.x=ent.x+speed;
            }
            if player.position.y < ent.y{
                ent.y=ent.y-speed;
            }
            if player.position.y > ent.y{
                ent.y=ent.y+speed;
            }
            if distance <40.0{
                if(player.playerEntity.hp>0){
                    if(player.playerEntity.hp>0){
                player.playerEntity.getHit(ent.dmg);
                //thread::sleep(Duration::new(1, 0));
               
                //animations.push(Animation{name:"slash".to_string(),x:player.position.x,y:player.position.y,duration:2})
                }
            }
            }
        }
    }
}
fn main() -> Result<(), String> {
   
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    //Window creation
    let window = video_subsystem.window("game tutorial", 1900, 1080)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    //canvas creation
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    //textures
    let texture_creator = canvas.texture_creator();
    //player
    let texture = texture_creator.load_texture("assets/entities/hero3.png")?;
    
    //tiels
    let mut tiles:[Texture;5]=[
        texture_creator.load_texture("assets/tiles/grass.jpeg")?,
        texture_creator.load_texture("assets/tiles/stairs.jpeg")?,
        texture_creator.load_texture("assets/tiles/stdoor.jpeg")?,
        texture_creator.load_texture("assets/tiles/stonef.jpeg")?,
        texture_creator.load_texture("assets/tiles/stonew.jpeg")?,
    ];
    
    println!("Textures loaded");
    //player creation
    let mut player = Player {
        position: Point::new(200, 200),
        sprite: Rect::new(0, 0, 100, 100),
        speed: 0,
        direction: Direction::Right,
        playerEntity:Entities::Entity::new("hero".to_string(),1000 ,50,100,0,0)
    };
    println!("player created");
    let mut currentMap:map::Map = map::Map::loadMap("l1".to_owned(),"levels/level1".to_owned());
    currentMap.toString();
    println!("map created");
    ///ENTITIES HERE
    let mut entities:Vec<Entities::Entity>;
    entities=loadEntities(currentMap.name.clone());
    let mut entity_textures:[Texture;10]=[
        texture_creator.load_texture("assets/entities/barbarian.png")?,
        texture_creator.load_texture("assets/entities/barbarian2.png")?,
        texture_creator.load_texture("assets/entities/barbarianK.png")?,
        texture_creator.load_texture("assets/entities/Darkmage1.png")?,
        texture_creator.load_texture("assets/entities/Darkmage2.png")?,
        texture_creator.load_texture("assets/entities/hero1.png")?,
        texture_creator.load_texture("assets/entities/hero2.png")?,
        texture_creator.load_texture("assets/entities/hero2b.png")?,
        texture_creator.load_texture("assets/entities/hero3.png")?,
        texture_creator.load_texture("assets/entities/hoplite.png")?
        
    ];
    let mut animations:Vec<Animation>= Vec::new();
    let mut animationTex:[Texture;1]=[
        texture_creator.load_texture("assets/animations/slash.png")?,
    ];
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    //println!("Hello");
                    let mapL =currentMap.layout.clone();
                    let lines: Vec<&str> = mapL.split('\n').collect();
                    let y = player.position.y/100 ;
                    let x:i32 = player.position.x/100;
                    let chars : Vec<_>= lines[y as usize].chars().collect();
                    let tile = chars[x as usize];
                    //println!("{}",tile);
                    if(tile == 'd'){
                        //println!("inside if");
                        if(currentMap.name.eq("l1")){
                            currentMap= map::Map::loadMap("l2".to_owned(),"levels/level2".to_owned());
                            entities=loadEntities(currentMap.name.clone());
                        }
                        else if(currentMap.name.eq("l2")){
                            currentMap= map::Map::loadMap("l3".to_owned(),"levels/level3".to_owned());
                            entities=loadEntities(currentMap.name.clone());
                        }
                    }
                    if(tile == 'u'){
                        //println!("inside if");
                        if(currentMap.name.eq("l2")){
                            currentMap= map::Map::loadMap("l1".to_owned(),"levels/level1".to_owned());
                            entities=loadEntities(currentMap.name.clone());
                        }
                        else if(currentMap.name.eq("l3")){
                            currentMap= map::Map::loadMap("l2".to_owned(),"levels/level2".to_owned());
                            entities=loadEntities(currentMap.name.clone());
                        }
                    }
                    
                }
                ,
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = 0;
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        update_player(&mut player,&currentMap.layout,&mut entities,&mut animations);
        updateAI(&mut entities, &mut player, &currentMap.layout,&mut animations);

        // Render
        render(&mut canvas, Color::RGB(0, 0, 0), &texture, &player,&mut currentMap,&tiles,&mut entities,&entity_textures,&mut animations,&mut animationTex)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}

fn loadEntities(map: String) -> Vec<Entities::Entity>{
    let mut entities:Vec<Entities::Entity>= Vec::new();
    if map == "l1"{
        let mut text:String=loadEnFile("levels/level1En".to_string());
        println!("{}",text);
        let lines : Vec<_> = text.split("\n").collect();
        for i in lines{
            let params:Vec<_> = i.split(" ").collect(); 
            println!("{}",i);
            entities.push(Entities::Entity::new(params[0].to_string(), params[1].parse::<i32>().unwrap(), params[2].parse::<i32>().unwrap(), params[3].parse::<i32>().unwrap(),params[4].parse::<i32>().unwrap(),params[5].parse::<i32>().unwrap()));
        }
    }
        if map == "l2"{
            let mut text:String=loadEnFile("levels/level2En".to_string());
            println!("{}",text);
            let lines : Vec<_> = text.split("\n").collect();
            for i in lines{
                let params:Vec<_> = i.split(" ").collect(); 
                println!("{}",i);
                entities.push(Entities::Entity::new(params[0].to_string(), params[1].parse::<i32>().unwrap(), params[2].parse::<i32>().unwrap(), params[3].parse::<i32>().unwrap(),params[4].parse::<i32>().unwrap(),params[5].parse::<i32>().unwrap()));
            }
    } 

    if map == "l3"{
        let mut text:String=loadEnFile("levels/level3En".to_string());
        println!("{}",text);
        let lines : Vec<_> = text.split("\n").collect();
        for i in lines{
            let params:Vec<_> = i.split(" ").collect(); 
            println!("{}",i);
            entities.push(Entities::Entity::new(params[0].to_string(), params[1].parse::<i32>().unwrap(), params[2].parse::<i32>().unwrap(), params[3].parse::<i32>().unwrap(),params[4].parse::<i32>().unwrap(),params[5].parse::<i32>().unwrap()));
        }
} 
    return entities;
}
fn loadEnFile(path:String) -> String{
    return fs::read_to_string(path).expect("en file not found");
}