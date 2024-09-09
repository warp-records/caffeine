
use std::hash::Hash;
use crate::hash_map::HashMap;
use crate::q::PriorityQ;

pub trait GraphNode<T> {
    fn get_neighbors(&self) -> Vec<(&Self, usize)>;
}

//Not sure if I spelled this right lol
fn dijkstra<T: Hash + Eq + PartialOrd + GraphNode<T>>(start: &T, goal: &T) {
    let mut pq = PriorityQ::<&T>::new();
    //HashMap<Elem, (prev nghbr with least dist, dist from prev nghbr)>
    //consider using struct to improve readability
    let mut node_meta = HashMap::<&T, (Option<&T>, usize)>::new();
    //visited means all of it's neighbors have been queued, not whether or not
    //it's distance values have been added
    let visited = HashMap::<&T, bool>::new();
    pq.push(start, 0);
    node_meta.insert(start, (None, 0));

    while !pq.is_empty() {
        //cum as in cumulative, not ejaculate
        let (curr_elem, cum_dist) = pq.pop().unwrap();
        if curr_elem == goal {
            break;
        }

        for (neighbor, nghbr_dist) in curr_elem.get_neighbors() {
            //note: possible optimization would be caching hash or giving a hash hint
            //since we're hashing the same element twice
            //is_none_or is available with nightly rust, but I don't think it's worth the switch
            if node_meta.get(&neighbor).is_none() || cum_dist+nghbr_dist < node_meta.get(&neighbor).unwrap().1 {
                node_meta.insert(neighbor, (Some(curr_elem), cum_dist+nghbr_dist));
            }

            if visited.get(&neighbor).is_none() {
                pq.push(&neighbor, cum_dist+nghbr_dist);
            }
        }
    }

}
