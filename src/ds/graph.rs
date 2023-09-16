use std::cmp::Reverse;
use std::collections::*;

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub to: usize,
    pub next: usize,
    pub w: i64,
}

/// # Example
/// ```
/// use rust_in_competitive_programming::*;
/// let mut gra = Graph::new(10, 4);
/// gra.add(1, 2);
/// gra.add(1, 3);
/// gra.add(3, 4);
/// gra.add(3, 5);
///
/// println!("{:?}", gra.get_edges(1));
/// for i in gra.get_edges(1) {
///     println!("{:?}", i);
/// }
/// ```
pub struct Graph {
    top: usize, 
    head: Vec<usize>, 
    edge: Vec<Star>,
}

impl Graph {
    /// `Graph::new(n, m)`:
    ///     Prelocate n nodes and m edges for graph
    pub fn new(n: usize, m: usize) -> Self {
        return Graph {
            top: 1,
            head: vec![0; n + 1],
            edge: vec![Star { to: 0, next: 0, w: 1 }; m + 1],
        };
    }

    /// `add(u, v)`:
    ///     Add ***DIRECTED*** edge u-\>v to graph
    pub fn add(&mut self, u: usize, v: usize) {
        if self.top >= self.edge.len() {
            panic!("Graph::add: edge overflow");
        }
        self.edge[self.top].to = v;
        self.edge[self.top].next = self.head[u];
        self.head[u] = self.top;
        self.top += 1;
    }

    /// `.add_weight(u, v, w)`:
    ///     Add ***DIRECTED*** edge u-\>v to graph, with weight w
    pub fn add_weight(&mut self, u: usize, v: usize, w: i64) {
        if self.top >= self.edge.len() {
            panic!("Graph::add: edge overflow");
        }
        self.edge[self.top].to = v;
        self.edge[self.top].next = self.head[u];
        self.edge[self.top].w = w;
        self.head[u] = self.top;
        self.top += 1;
    }

    /// `get_edges(u)`:
    ///     Get all edges from node u
    pub fn get_edges(&self, u: usize) -> Vec<&Star> {
        let mut ans = vec![];
        let mut i = self.head[u];
        while i != 0 {
            ans.push(&self.edge[i]);
            i = self.edge[i].next;
        }
        return ans;
    }
}


/// `dijkstra(&gra, s)`:
///     Heap-optimized dijkstra algorithm, O(mlogm)
///     return a vector of shortest distance from node s
pub fn dijkstra(gra: &Graph, s: usize) -> Vec<i64> {
    let mut dis = vec![i64::MAX; gra.head.len()];
    let mut vis = vec![false; gra.head.len()];
    let mut que = BinaryHeap::new();
    dis[s] = 0;
    que.push((Reverse(0), s));
    while let Some((Reverse(d), u)) = que.pop() {
        if vis[u] || dis[u] < d { continue; }
        vis[u] = true;

        for v in gra.get_edges(u) {
            if dis[v.to] > dis[u] + v.w {
                dis[v.to] = dis[u] + v.w;
                que.push((Reverse(dis[v.to]), v.to));
            }
        }
    }
    return dis;
}


// pub fn mst_prime(gra: &Graph) -> i64 {
//
// }

