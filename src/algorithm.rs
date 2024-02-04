use num::Zero;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;
use std::vec::Vec;

pub fn a_star<NodeType, TFn, NFn, HFn, CostType>(
    start: NodeType,
    target_test: TFn,
    neighbors: NFn,
    heuristic: HFn,
) -> Option<CostType>
where
    NodeType: Clone + Hash + Ord,
    CostType: Ord + PartialOrd + Add + Zero + Clone,
    TFn: Fn(&NodeType) -> bool,
    NFn: Fn(&NodeType) -> Vec<(NodeType, CostType)>,
    HFn: Fn(&NodeType) -> CostType,
{
    let mut queue: BinaryHeap<(Reverse<CostType>, NodeType)> = BinaryHeap::new();
    let mut traversed: HashMap<NodeType, CostType> = HashMap::new();
    queue.push((Reverse(heuristic(&start)), start.clone()));
    traversed.insert(start, CostType::zero());

    while let Some((_, node)) = queue.pop() {
        let total_cost = traversed[&node].clone();
        if target_test(&node) {
            return Some(total_cost.clone());
        }
        for (nei, cost) in neighbors(&node) {
            let next_cost = total_cost.clone() + cost;
            if !traversed.contains_key(&nei) || traversed[&nei] > next_cost {
                queue.push((Reverse(next_cost.clone() + heuristic(&nei)), nei.clone()));
                traversed.insert(nei, next_cost);
            }
        }
    }
    None
}

pub fn dijkstra<N, TFn, NFn, C>(start: N, target_test: TFn, neighbors: NFn) -> Option<C>
where
    N: Clone + Hash + Ord,
    C: Ord + PartialOrd + Add + Zero + Clone,
    TFn: Fn(&N) -> bool,
    NFn: Fn(&N) -> Vec<(N, C)>,
{
    a_star(start, target_test, neighbors, |_| C::zero())
}
