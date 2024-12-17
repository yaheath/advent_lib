use num::Zero;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;
use std::vec::Vec;

pub fn a_star_ex<NodeType, TFn, NFn, HFn, CostType>(
    start: NodeType,
    target_test: TFn,
    neighbors: NFn,
    heuristic: HFn,
    exhaustive: bool,
) -> Option<(CostType, HashMap<NodeType, (CostType, HashSet<NodeType>)>)>
where
    NodeType: Clone + Hash + Ord,
    CostType: Ord + PartialOrd + Add + Zero + Clone,
    TFn: Fn(&NodeType) -> bool,
    NFn: Fn(&NodeType) -> Vec<(NodeType, CostType)>,
    HFn: Fn(&NodeType) -> CostType,
{
    let mut queue: BinaryHeap<(Reverse<CostType>, NodeType)> = BinaryHeap::new();
    let mut traversed: HashMap<NodeType, CostType> = HashMap::new();
    let mut prev: HashMap<NodeType, (CostType, HashSet<NodeType>)> = HashMap::new();
    queue.push((Reverse(heuristic(&start)), start.clone()));
    traversed.insert(start, CostType::zero());
    let mut lowest_cost: Option<CostType> = None;

    while let Some((_, node)) = queue.pop() {
        let total_cost = traversed[&node].clone();
        if target_test(&node) {
            if !exhaustive {
                return Some((total_cost.clone(), prev));
            }
            if lowest_cost.is_none() {
                lowest_cost = Some(total_cost.clone());
            }
        }
        for (nei, cost) in neighbors(&node) {
            let next_cost = total_cost.clone() + cost;
            if let Some(lc) = &lowest_cost {
                if next_cost > *lc {
                    continue;
                }
            }
            if !traversed.contains_key(&nei) || traversed[&nei] > next_cost {
                prev.entry(nei.clone())
                    .and_modify(|(c, set)| {
                        if *c == next_cost {
                            set.insert(node.clone());
                        } else if *c > next_cost {
                            set.clear();
                            set.insert(node.clone());
                        }
                    })
                    .or_insert((next_cost.clone(), HashSet::from_iter([node.clone()])));
                queue.push((Reverse(next_cost.clone() + heuristic(&nei)), nei.clone()));
                traversed.insert(nei, next_cost);
            } else if exhaustive && traversed[&nei] == next_cost {
                prev.entry(nei.clone())
                    .and_modify(|(c, set)| {
                        if *c == next_cost {
                            set.insert(node.clone());
                        } else if *c > next_cost {
                            set.clear();
                            set.insert(node.clone());
                        }
                    })
                    .or_insert((next_cost, HashSet::from_iter([node.clone()])));
            }
        }
    }
    lowest_cost.map(|c| (c, prev))
}

pub fn a_star<NodeType, TFn, NFn, HFn, CostType>(
    start: NodeType,
    target_test: TFn,
    neighbors: NFn,
    heuristic: HFn,
) -> Option<CostType>
where
    NodeType: Clone + Copy + Hash + Ord,
    CostType: Ord + PartialOrd + Add + Zero + Clone,
    TFn: Fn(NodeType) -> bool,
    NFn: Fn(NodeType) -> Vec<(NodeType, CostType)>,
    HFn: Fn(NodeType) -> CostType,
{
    a_star_ex(
        start,
        |node| target_test(*node),
        |node| neighbors(*node),
        |node| heuristic(*node),
        false,
    )
    .map(|r| r.0)
}

pub fn dijkstra<N, TFn, NFn, C>(start: N, target_test: TFn, neighbors: NFn) -> Option<C>
where
    N: Clone + Copy + Hash + Ord,
    C: Ord + PartialOrd + Add + Zero + Clone,
    TFn: Fn(N) -> bool,
    NFn: Fn(N) -> Vec<(N, C)>,
{
    a_star_ex(
        start,
        |node| target_test(*node),
        |node| neighbors(*node),
        |_| C::zero(),
        false,
    )
    .map(|r| r.0)
}

pub fn dijkstra_ex<N, TFn, NFn, C>(
    start: N,
    target_test: TFn,
    neighbors: NFn,
    exhaustive: bool,
) -> Option<(C, HashMap<N, (C, HashSet<N>)>)>
where
    N: Clone + Hash + Ord,
    C: Ord + PartialOrd + Add + Zero + Clone,
    TFn: Fn(&N) -> bool,
    NFn: Fn(&N) -> Vec<(N, C)>,
{
    a_star_ex(start, target_test, neighbors, |_| C::zero(), exhaustive)
}
