use std::{collections::{HashMap, HashSet}, env, fs::read_to_string, hash::Hash};


pub fn day_12_p1_soln() -> u32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let map: HashMap<Coord, Plot> = parse_input(&raw);
    calc_cost_all_fences(&map)
}


pub fn parse_input(content: &str) -> HashMap<Coord, Plot> {
    let mut result: HashMap<Coord, Plot> = HashMap::new();
    for (r,line) in content.lines().enumerate() {
        for (c, chr) in line.char_indices() {
            let plot: Plot = make_plot(chr, make_coord(r, c));
            result.insert(make_coord(r, c), plot);
        }
    }
    result
}


pub fn calc_cost_all_fences(map: &HashMap<Coord, Plot>) -> u32 {
    let all_regions: HashMap<char, Vec<Region>> = find_all_regions(map);
    let mut total_cost = 0;
    for (_, regions) in all_regions {
        for region in regions {
            let cost: usize = region.plots.len() * region.perimeter();
            total_cost += cost;
        }
    }
    total_cost as u32
}


fn find_all_regions(map: &HashMap<Coord, Plot>) -> HashMap<char, Vec<Region>>{
    let mut all_regions: HashMap<char, Vec<Region>> = HashMap::new();
    for (_, new_plot) in map {
        if let Some(regions_of_same_typ) = all_regions.get(&new_plot.typ) {
            if !plot_in_these_regions(new_plot, regions_of_same_typ) {
                // this dude part of new region of a known type
                let new_region: Region = fill_this_region(map, new_plot);
                all_regions.entry(new_plot.typ).or_insert(vec![]).push(new_region);
            }
        } else { // this dude part of new region of new type
            let new_region: Region = fill_this_region(map, new_plot);
            all_regions.entry(new_plot.typ).or_insert(vec![]).push(new_region);
        }
        
    }
    all_regions
}

fn fill_this_region(map: &HashMap<Coord, Plot>, plot: &Plot) -> Region {
    let mut region: Region = Region{plots: HashSet::from([plot.clone()])};
    let mut neighbors_to_add: HashSet<Plot> = HashSet::from(find_neighs_in_region(map, plot));
    while neighbors_to_add.len() != 0 {
        // remove a neighbor to add to region
        let neigh: Plot = neighbors_to_add.iter().next().unwrap().clone();
        neighbors_to_add.remove(&neigh);
        // add it to region
        region.plots.insert(neigh);
        // use that to find more in the region
        for new_neigh in find_neighs_in_region(map, &neigh) {
            if !region.plots.contains(&new_neigh) {
                neighbors_to_add.insert(new_neigh);
            }
        }
    }
    region
}

fn find_neighs_in_region(map: &HashMap<Coord, Plot>, plot: &Plot) -> HashSet<Plot> {
    let mut result: HashSet<Plot> = HashSet::new();
    let (r,c) = (plot.pos.r, plot.pos.c);
    if r != 0 {// North -- prevent underflow of usize
        if let Some(&n) = map.get(&make_coord(r-1, c)){
            if n.typ == plot.typ { result.insert(n); }
        }
    }
    if let Some(&n) = map.get(&make_coord(r, c+1)){ // East
        if n.typ == plot.typ { result.insert(n); }
    }
    if let Some(&n) = map.get(&make_coord(r+1, c)){ // South
        if n.typ == plot.typ { result.insert(n); }
    }
    if c != 0 {// West -- prevent underflow of usize
        if let Some(&n) = map.get(&make_coord(r, c-1)){
            if n.typ == plot.typ { result.insert(n); }
        }
    }
    result
}

fn num_neighs_in_region(reg: &Region, plot: &Plot) -> usize {
    let mut result: HashSet<Plot> = HashSet::new();
    let typ: char = plot.typ;
    let (r,c) = (plot.pos.r, plot.pos.c);
    if r != 0 {// North -- prevent underflow of usize
        if let Some(&n) = reg.plots.get(&make_plot(typ, make_coord(r-1, c))){
            if n.typ == plot.typ { result.insert(n); }
        }
    }
    if let Some(&n) = reg.plots.get(&make_plot(typ, make_coord(r, c+1))){ // East
        if n.typ == plot.typ { result.insert(n); }
    }
    if let Some(&n) = reg.plots.get(&make_plot(typ, make_coord(r+1, c))){ // South
        if n.typ == plot.typ { result.insert(n); }
    }
    if c != 0 {// West -- prevent underflow of usize
        if let Some(&n) = reg.plots.get(&make_plot(typ, make_coord(r, c-1))){
            if n.typ == plot.typ { result.insert(n); }
        }
    }
    assert!(result.len() <= 4);
    result.len()
}

fn plot_in_these_regions(plot: &Plot, regions: &Vec<Region>) -> bool {
    for region in regions {
        if region.plots.contains(plot) {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub plots: HashSet<Plot>,
}

impl Region {
    fn perimeter(&self) -> usize {
        let mut perimeter: usize = 0;
        for plot in self.plots.clone() {
            let num_neighs: usize = num_neighs_in_region(self, &plot);
            perimeter += 4-num_neighs;
        }
        perimeter
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Plot {
    pub pos: Coord,
    pub typ: char,
}

pub fn make_plot(typ: char, pos: Coord) -> Plot {
    Plot{typ, pos}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub r: usize,
    pub c: usize,
}

pub fn make_coord(r: usize, c: usize) -> Coord {
    Coord{r,c}
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec() {
        let input: &str = "AAAA\nBBCD\nBBCC\nEEEC";
        let map: HashMap<Coord, Plot> = parse_input(&input);
        let cost: u32 = calc_cost_all_fences(&map);
        assert_eq!(cost, 140);

        let input: &str = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
        let map: HashMap<Coord, Plot> = parse_input(&input);
        let cost: u32 = calc_cost_all_fences(&map);
        assert_eq!(cost, 772);

        let input: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let map: HashMap<Coord, Plot> = parse_input(&input);
        let cost: u32 = calc_cost_all_fences(&map);
        assert_eq!(cost, 1930);
    }
}