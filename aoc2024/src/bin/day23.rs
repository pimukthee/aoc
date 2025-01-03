use std::collections::HashSet;

fn get_index(u: (u8, u8)) -> usize {
    (u.0 - b'a') as usize * 26 + (u.1 - b'a') as usize
}

fn find_clique(
    adj_list: &[Vec<(u8, u8)>],
    edges: &HashSet<((u8, u8), (u8, u8))>,
    u: (u8, u8),
    i: usize,
    mut max_clique: Vec<(u8, u8)>,
    clique: &mut Vec<(u8, u8)>,
) -> Vec<(u8, u8)> {
    let adjs = &adj_list[get_index(u)];
    if i == adjs.len() {
        return if clique.len() > max_clique.len() {
            clique.clone()
        } else {
            max_clique
        };
    }

    max_clique = find_clique(adj_list, edges, u, i + 1, max_clique, clique);
    for node in clique.iter() {
        if !edges.contains(&(*node, adjs[i])) {
            return max_clique
        }
    }
    clique.push(adjs[i]);
    max_clique = find_clique(adj_list, edges, u, i + 1, max_clique, clique);
    clique.retain(|&node| node != adjs[i]);

    max_clique
}

fn part2(adj_list: &[Vec<(u8, u8)>], edges: &HashSet<((u8, u8), (u8, u8))>) -> String {
    let mut max_clique = vec![];
    for pre in b'a'..b'z' + 1 {
        for suf in b'a'..b'z' + 1 {
            if adj_list[get_index((pre, suf))].is_empty() {
                continue;
            }
            let clique = find_clique(adj_list, edges, (pre, suf), 0, vec![], &mut vec![(pre, suf)]);
            if clique.len() <= max_clique.len() {
                continue;
            }
            max_clique = clique;
        }
    }
    max_clique.sort_unstable();
    max_clique.into_iter().map(|(pre, suf)| format!("{}{}", pre as char, suf as char)).collect::<Vec<String>>().join(",")
}

fn part1(adj_list: &[Vec<(u8, u8)>], edges: &HashSet<((u8, u8), (u8, u8))>) -> usize {
    let mut ans = 0;
    for prefix in b'a'..b'z' + 1 {
        for suffix in b'a'..b'z' + 1 {
            let u = (prefix, suffix);
            for adj in &adj_list[get_index(u)] {
                ans += adj_list[get_index(*adj)]
                    .iter()
                    .filter(|&&node| {
                        edges.contains(&(u, node))
                            && (prefix == b't' || adj.0 == b't' || node.0 == b't')
                    })
                    .count();
            }
        }
    }

    ans / 6
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut graph: Vec<Vec<(u8, u8)>> = vec![vec![]; 26 * 26];
    let mut edges: HashSet<((u8, u8), (u8, u8))> = HashSet::new();

    while let Some(Ok(line)) = lines.next() {
        let (u, v) = line.split_once('-').unwrap();
        let u = u.as_bytes();
        let v = v.as_bytes();
        graph[get_index((u[0], u[1]))].push((v[0], v[1]));
        graph[get_index((v[0], v[1]))].push((u[0], u[1]));
        edges.insert(((u[0], u[1]), (v[0], v[1])));
        edges.insert(((v[0], v[1]), (u[0], u[1])));
    }

    println!("{}", part1(&graph, &edges));
    println!("{}", part2(&graph, &edges));
}
