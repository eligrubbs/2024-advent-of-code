use crate::parser::parse_day_5_input;


fn get_inorder_not_inorder<'a>(rules: &Vec<(i32,i32)>, pages: &'a Vec<Vec<i32>>) -> (Vec<&'a Vec<i32>>, Vec<&'a Vec<i32>>){
    //find all in order
    let mut inorder_pages: Vec<&Vec<i32>> = vec![];
    let mut outorder_pages: Vec<&Vec<i32>> = vec![];

    for i in 0..pages.len() {
        let p: &Vec<i32> = pages.get(i).unwrap();

        let mut add_it:bool = true;
        for l in 0..(p.len()-1){
            for r in l..p.len() {
                if !rule_lte(&rules, p.get(l).unwrap(), p.get(r).unwrap()) {
                    add_it = false;
                }
            }
        }

        if add_it { inorder_pages.push(p); } 
        else { outorder_pages.push(p); }
    }

    (inorder_pages, outorder_pages)
}

fn rule_lte(rules: &Vec<(i32,i32)>, lhs: &i32, rhs: &i32) -> bool {
    for (rl, rr) in rules {
        if *lhs == *rr && *rhs == *rl {
            // x <= y is false if there is a rule y|x, else true
            return false;
        }
    }
    true
}


pub fn day_5_p1_soln() -> i32 {
    let (rules, pages) = parse_day_5_input();

    //find all in order
    let (inorder_pages, _) = get_inorder_not_inorder(&rules, &pages);

    //get middle element
    inorder_pages.iter().map(|page| page.get(page.len()/2).unwrap()).sum()
}


pub fn day_5_p2_soln() -> i32 {
    let (rules, pages) = parse_day_5_input();

    //find all out order
    let (_, mut outorder_pages) = get_inorder_not_inorder(&rules, &pages);

    outorder_pages.iter_mut().map(|x| reorder_page(&rules, x))
                             .map(|page| page.get(page.len()/2).unwrap().clone()).sum()
}

fn reorder_page(rules: &Vec<(i32,i32)>, p: &Vec<i32>) -> Vec<i32> {
    let mut l: usize = 0;
    let mut p_f: Vec<i32> = p.clone();

    while l < (p.len()-1){
        let mut no_change: bool = false;

        for r in (l+1)..p_f.len() {
            if !rule_lte(&rules, p_f.get(l).unwrap(), p_f.get(r).unwrap()) {
                //swap
                p_f.swap(l,r);
                // don't increment l
                no_change = true;
                // restart this right-side sweep
                break;
            }
        }
        l = if no_change {l} else {l+1};
    }
    p_f
}