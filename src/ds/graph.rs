use crate::*;
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
    ///     ***NOTE***: node index starts from 1
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
        if u == 0 || v == 0 {
            panic!("Graph::add: node index starts from 1");
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
        if u == 0 || v == 0 {
            panic!("Graph::add: node index starts from 1");
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
        if u == 0 || u >= self.head.len() {
            panic!("Graph::get_edges: cannot index node({})", u);
        }
        let mut ans = vec![];
        let mut i = self.head[u];
        while i != 0 {
            ans.push(&self.edge[i]);
            i = self.edge[i].next;
        }
        return ans;
    }

    pub fn is_connected(&self) -> bool {
        let mut set = DisjointSet::new(self.head.len());
        for i in 1..self.head.len() {
            for edge in self.get_edges(i) {
                set.merge(i, edge.to);
            }
        }

        for i in 2..self.head.len() {
            if set.same(1, i) == false {
                return false;
            }
        }
        return true
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

/// `kruskal(&gra)`:
///     Kruskal algorithm(DisjointSet + Heap), O(mlogm)
///     return the minimum spanning tree's weight
///     if the graph is not connected, return None
pub fn kruskal(gra: &Graph) -> Option<i64> {
    if gra.is_connected() == false { return None; }

    let mut ds = DisjointSet::new(gra.head.len());
    let mut que = BinaryHeap::new();
    for i in 1..gra.head.len() {
        for edge in gra.get_edges(i) {
            que.push((Reverse(edge.w), i, edge.to));
        }
    }
    let mut ans = 0;
    while let Some((Reverse(w), u, v)) = que.pop() {
        if ds.same(u, v) == false {
            ds.merge(u, v);
            ans += w;
        }
    }
    return Some(ans);
}

/// `prim_raw(&gra)`:
///    Prim algorithm(Brute Force), O(n^2 + m)
///    return the minimum spanning tree's weight
///    if the graph is not connected, return None
pub fn prim_raw(gra: &Graph) -> Option<i64> {
    if gra.is_connected() == false { return None; }

    let mut dis = vec![i64::MAX; gra.head.len()];
    let mut vis = vec![false; gra.head.len()];
    dis[1] = 0;
    let mut ans = 0;
    for _ in 1..gra.head.len() {
        let mut u = 0;
        for i in 1..gra.head.len() {
            if vis[i] == false && (u == 0 || dis[i] < dis[u]) {
                u = i;
            }
        }
        vis[u] = true;
        ans += dis[u];
        for v in gra.get_edges(u) {
            if vis[v.to] == false && dis[v.to] > v.w {
                dis[v.to] = v.w;
            }
        }
    }
    return Some(ans);
}

/// `prim_heap(&gra)`:
///   Prim algorithm(Heap Optimized), O(mlogm)
///   return the minimum spanning tree's weight
///   if the graph is not connected, return None
pub fn prim_heap(gra: &Graph) -> Option<i64> {
    if gra.is_connected() == false { return None; }

    let mut dis = vec![i64::MAX; gra.head.len()];
    let mut vis = vec![false; gra.head.len()];
    let mut que = BinaryHeap::new();
    dis[1] = 0;
    que.push((Reverse(0), 1));
    let mut ans = 0;
    while let Some((Reverse(d), u)) = que.pop() {
        if vis[u] || dis[u] < d { continue; }
        vis[u] = true;
        ans += d;

        for v in gra.get_edges(u) {
            if dis[v.to] > v.w {
                dis[v.to] = v.w;
                que.push((Reverse(dis[v.to]), v.to));
            }
        }
    }
    return Some(ans);
}
