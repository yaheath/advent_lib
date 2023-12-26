use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::vec::Vec;

pub fn a_star<NodeType, TFn, NFn, HFn>(
    start: NodeType,
    target_test: TFn,
    neighbors: NFn,
    heuristic: HFn) -> Option<usize>
where NodeType: Copy + Hash + Ord,
      TFn: Fn(NodeType) -> bool,
      NFn: Fn(NodeType) -> Vec<(NodeType, usize)>,
      HFn: Fn(NodeType) -> usize,
{
    let mut queue: BinaryHeap<(Reverse<usize>, NodeType)> = BinaryHeap::new();
    let mut traversed: HashMap<NodeType, usize> = HashMap::new();
    queue.push((Reverse(heuristic(start)), start));
    traversed.insert(start, 0);

    while let Some((_, node)) = queue.pop() {
        let total_cost = traversed[&node];
        if target_test(node) {
            return Some(total_cost);
        }
        for (nei, cost) in neighbors(node) {
            let next_cost = total_cost + cost;
            if !traversed.contains_key(&nei) || traversed[&nei] > next_cost {
                queue.push((Reverse(next_cost + heuristic(nei)), nei));
                traversed.insert(nei, next_cost);
            }
        }
    }
    None
}

pub fn dijkstra<NodeType, TFn, NFn>(start: NodeType, target_test: TFn, neighbors: NFn) -> Option<usize>
where NodeType: Copy + Hash + Ord,
      TFn: Fn(NodeType) -> bool,
      NFn: Fn(NodeType) -> Vec<(NodeType, usize)>,
{
    a_star(start, target_test, neighbors, |_| 0)
}
