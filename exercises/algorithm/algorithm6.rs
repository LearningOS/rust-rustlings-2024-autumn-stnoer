/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/


use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 如果当前节点v已经访问过，直接返回
        if visited.contains(&v) {
            return;
        }

        // 标记当前节点v为已访问
        visited.insert(v);

        // 记录访问顺序
        visit_order.push(v);

        // 递归访问邻接节点
        for &neighbor in &self.adj[v] {
            self.dfs_util(neighbor, visited, visit_order);
        }
    }

    // 执行DFS遍历，返回访问节点的顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new(); // 记录访问过的节点
        let mut visit_order = Vec::new(); // 记录访问顺序
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

}