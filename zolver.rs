use std::collections::HashSet;


type Shape = Vec<(i32, i32)>;
type State = Vec<bool>;

type Index = Vec<Vec<i32>>;

fn main() {
    // let initial_state = vec![false, true, false];
    // let shape: Shape = vec![(0, 1), (1, 2)];
    // "The Fork"
    // let initial_state = vec![false, false, false, false, false, false, false, false, false] ;
    // let shape: Shape = vec![(0, 1), (1, 2), (2, 4), (4, 3), (4, 5), (4, 7), (3, 6), (6, 7), (7, 8), (5, 8)];
    // "The Whale"
    let initial_state = vec![true, true, true, false, true, true, true, true, true, true, true, true, true, true];
    let shape: Shape = vec![(0, 1), (1, 4), (4, 6), (6, 5), (5, 2), (2, 0), (3, 0), (3, 1), (3, 4), (3, 6), (3, 5), (3, 2),  (6, 7), (7, 8),  (8, 12), (12, 13),  (5, 9), (9, 10), (10, 11), (11, 8),  (9, 6), (6, 10), (10, 7), (7, 11)];
    let wanted_state:Vec<_> = initial_state.iter().map(|_| true).collect();

    if !check_input(&initial_state, &shape) {
        println!("Invalid input");
        return;
    }
    let count = initial_state.len();
    let indexed_shape = indexed_shape(&shape, count as i32);
    println!("Initial State: {:?}", initial_state);
    println!("Wanted State: {:?}", wanted_state);
    
    let bases : Vec<State> = (0..count as i32).map(|i| make_move(&indexed_shape, &wanted_state.clone(), i)).collect();
    println!("Bases: {:?}", bases);
    let case_count = i32::pow(2,bases.len() as u32);
    println!("Brute Force! Cases: {}", case_count);
    for i in 0..case_count {
        let mut state = initial_state.clone();
        for j in 0..bases.len() {
            if (i >> j) & 1 == 1 {
                state = make_move(&indexed_shape, &state, j as i32);
            }
        }
        if state == wanted_state {
            println!("solution:");
            state = initial_state.clone();
            println!("starting with: {:?}", state);
            for j in 0..bases.len() {
                if (i >> j) & 1 == 1 {
                    state = make_move(&indexed_shape, &state, j as i32);
                    println!("press {} gives you {:?}", j, state);
                }
            }
        }
    }
}

fn normalize_entry(entry: (i32, i32)) -> (i32, i32) {
    return if entry.0<entry.1 { entry } else { (entry.1,entry.0) };
}

fn check_input(state: &State, shape: &Shape) -> bool {
    return check_shape(&shape, (state.len()-1) as i32);
}

fn check_shape(shape: &Shape, max: i32) -> bool {
    for &(a, b) in shape.iter() {
        if a < 0 || a > max {
            println!("index '{}' is out of bounds", b);
            return false;
        }
        if b < 0 || b > max {
            println!("index '{}' is out of bounds", b);
            return false;
        }
        if a == b {
            println!("({}, {}) pointing to itself", a, b);
            return false;
        }
    }

    let mut h = HashSet::new();
    for &entry in shape.iter() {
        let entry = normalize_entry(entry);
        if h.contains(&entry) {
            println!("repeated edge: ({}, {})", entry.0, entry.1);
            return false;
        }
        h.insert(entry);
    }
    return true;
}

fn make_move(indexed_shape: &Index, state: &State, index: i32) -> State {
    let ix = index as usize;
    let mut result = state.clone();
    result[ix] = !result[ix];
    for i in &indexed_shape[ix] {
        let ii = i.clone() as usize; // ew
        result[ii] = !result[ii];
    }
    return result;
}

fn indexed_shape(shape: &Shape, len: i32) -> Index {
    let mut map : Index  = Vec::with_capacity(len as usize);
    // map.push([])
    for _ in 0..len {
        map.push(vec![]);
    }
    for &(a, b) in shape.iter() {
        map[a as usize].push(b);
        map[b as usize].push(a);
    }
    return map
}