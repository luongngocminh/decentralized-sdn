use std::cmp::Ordering;

use bluesea_identity::NodeId;
use serde::{Deserialize, Serialize};

pub const BANDWIDTH_LIMIT: u32 = 10000; //10Mbps
const HOP_PLUS_RTT: u16 = 10; //10ms each hops

/// Concatenate two hops array, with condition that the last hop of `a` is the first hop of `b`, if not return None
pub fn concat_hops(a: &Vec<NodeId>, b: &Vec<NodeId>) -> Option<Vec<NodeId>> {
    if a.is_empty() {
        return Some(b.clone());
    }
    if b.is_empty() {
        return Some(a.clone());
    }
    if a.last().unwrap() == b.first().unwrap() {
        let mut ret = a.clone();
        ret.extend_from_slice(&b[1..]);
        return Some(ret);
    }
    None
}

/// Path to destination, all nodes in reverse path
/// Example with local connection : A -> A => hops: [A],
/// Example with direct connection : A -> B => hops: [B, A],
/// Example with indirect connection : A -> B -> C => hops: [C, B, B],
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metric {
    pub latency: u16,      //in milliseconds
    pub hops: Vec<NodeId>, //in hops, from 1 (direct)
    pub bandwidth: u32,    //in kbps
                           // pub lost: f32,
                           // pub jitter: u16,
}

impl Metric {
    pub fn new(latency: u16, hops: Vec<NodeId>, bandwidth: u32) -> Self {
        Metric {
            latency,
            hops,
            bandwidth,
        }
    }

    pub fn contain_in_hops(&self, node_id: NodeId) -> bool {
        self.hops.contains(&node_id)
    }

    pub fn add(&self, other: &Self) -> Option<Self> {
        Some(Metric {
            latency: self.latency + other.latency,
            hops: concat_hops(&self.hops, &other.hops)?,
            bandwidth: std::cmp::min(self.bandwidth, other.bandwidth),
        })
    }
}

impl PartialOrd for Metric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.bandwidth >= BANDWIDTH_LIMIT && other.bandwidth >= BANDWIDTH_LIMIT)
            || (self.bandwidth < BANDWIDTH_LIMIT && other.bandwidth < BANDWIDTH_LIMIT)
        {
            let res = match (self.latency + (self.hops.len() as u16 * HOP_PLUS_RTT))
                .cmp(&(other.latency + (other.hops.len() as u16 * HOP_PLUS_RTT)))
            {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match self.hops.len().cmp(&other.hops.len()) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => match self.bandwidth.cmp(&other.bandwidth) {
                        Ordering::Less => Ordering::Greater,
                        Ordering::Greater => Ordering::Less,
                        Ordering::Equal => Ordering::Equal,
                    },
                },
            };
            Some(res)
        } else if self.bandwidth >= BANDWIDTH_LIMIT {
            Some(Ordering::Less)
        } else if other.bandwidth >= BANDWIDTH_LIMIT {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialEq<Self> for Metric {
    fn eq(&self, other: &Self) -> bool {
        self.latency == other.latency && self.hops == other.hops
    }
}

#[cfg(test)]
mod tests {
    use crate::table::metric::{concat_hops, Metric};

    #[test]
    fn eq() {
        let m1 = Metric::new(1, vec![1], 10000);
        let m2 = Metric::new(1, vec![1], 10000);

        assert_eq!(m1, m2);
    }

    #[test]
    fn compare() {
        let m1 = Metric::new(1, vec![1], 10000);
        let m2 = Metric::new(2, vec![2], 10000);
        let m3 = Metric::new(2, vec![3, 4], 10000);

        assert!(m1 < m2);
        assert!(m2 > m1);

        assert!(m1 < m3);
        assert!(m2 < m3);
        assert!(m3 > m2);
    }

    #[test]
    fn compare_bandwidth_limit() {
        let m1 = Metric::new(1, vec![1], 9000);
        let m2 = Metric::new(2, vec![2], 10000);
        let m3 = Metric::new(2, vec![3], 9000);
        assert!(m2 < m1);
        assert!(m1 < m3);
    }

    #[test]
    fn add() {
        let m1 = Metric::new(1, vec![1, 2], 10000);
        let m2 = Metric::new(2, vec![2, 3], 20000);
        let m3 = Metric::new(2, vec![3, 4], 20000);

        assert_eq!(m1.add(&m2), Some(Metric::new(3, vec![1, 2, 3], 10000)));
        assert_eq!(m1.add(&m3), None);
    }

    #[test]
    fn concat_test() {
        assert_eq!(concat_hops(&vec![1, 2], &vec![2, 3]), Some(vec![1, 2, 3]));
        assert_eq!(concat_hops(&vec![1, 2], &vec![3, 4]), None);
        assert_eq!(concat_hops(&vec![1, 2], &vec![]), Some(vec![1, 2]));
        assert_eq!(concat_hops(&vec![], &vec![1, 2]), Some(vec![1, 2]));
    }

    #[test]
    fn hops_has_affect_latancy() {
        let m1 = Metric::new(1, vec![1, 2], 10000);
        let m2 = Metric::new(2, vec![2], 10000);

        assert!(m1 > m2);
    }
}