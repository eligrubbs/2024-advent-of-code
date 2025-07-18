use std::{collections::HashMap, env, fs::read_to_string};
use regex::Regex;


pub fn day_14_p1_soln() -> i32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let mut bots: Vec<Robot> = parse_input(&raw, 101, 103);
    move_all_for_x_secs(&mut bots, 100);
    get_saftey_factor(&bots)
}

pub fn get_saftey_factor(bots: &Vec<Robot>) -> i32 {
    let mut counts: HashMap<Quadrant, i32> = HashMap::new();
    bots.iter().for_each(|bot| {
        counts.entry(bot.quad()).and_modify(|x| *x+=1).or_insert(1);
    });
    counts.iter().filter(|(&quad, _)| quad != Quadrant::NOT).map(|(_,count)| count).product()
}

pub fn move_all_for_x_secs(bots: &mut Vec<Robot>, secs: u32) {
    bots.iter_mut().for_each(|bot| bot.move_x_seconds(secs as i32));
}

pub fn parse_input(content: &str, width: i32, height: i32) -> Vec<Robot> {
    let robot_reg: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots: Vec<Robot> = content.lines()
        .map(|line| robot_from_str(line, &robot_reg, width, height))
        .collect(); 
    robots
}

fn robot_from_str(input: &str, reg: &Regex, width: i32, height: i32) -> Robot {
    let (_, [x,y,dx,dy]) = reg.captures(input).unwrap().extract();
    Robot { x: str_to_i32(x), y: str_to_i32(y), dx: str_to_i32(dx), dy: str_to_i32(dy), width:width, height:height }
}

fn str_to_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quadrant {
    NW, NE, SW, SE, NOT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot{
    pub x:i32,
    pub y:i32,
    pub dx:i32,
    pub dy:i32,
    pub width:i32,
    pub height:i32,
}


impl Robot{

    pub fn move_x_seconds(&mut self, secs: i32) {
        let x_diff: i32 = (self.dx * secs) % self.width;
        let y_diff: i32 = (self.dy * secs) % self.height;
        self.x = (self.x+x_diff+self.width) % self.width;
        self.y = (self.y+y_diff+self.height) % self.height;
    }

    pub fn quad(&self) -> Quadrant {
        use Quadrant::*;
        if self.x < self.width/2 && self.y < self.height/2 {
            NW
        } else if self.x > self.width/2 && self.y < self.height/2  {
            NE
        } else if self.x < self.width/2 && self.y > self.height/2 {
            SW
        } else if self.x > self.width/2 && self.y > self.height/2  {
            SE
        } else {
            NOT
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let mut bots: Vec<Robot> = parse_input(&input, 11, 7);
        move_all_for_x_secs(&mut bots, 100);
        let score: i32 = get_saftey_factor(&bots);
        assert_eq!(12, score);
    }


    #[test]
    fn test_robot() {
        let mut bot: Robot= Robot{x:2,y:4,dx:2,dy:-3,width:11,height:7};
        bot.move_x_seconds(1);
        assert_eq!(bot, Robot{x:4,y:1,dx:2,dy:-3,width:11,height:7})
    }
}