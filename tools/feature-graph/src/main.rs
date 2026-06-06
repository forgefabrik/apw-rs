//! feature-graph — dependency graph analysis for the Feature Registry.
//!
//! Subcommands:
//!   show [--milestone Mx]   print features in scope and their dependencies
//!   blocks --milestone Mx   list features in OTHER milestones that block Mx
//!   critical-path [--milestone Mx]   longest dependency chain (bottleneck)
//!   cycles                  Tarjan SCC — surfaces any circular deps

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;
use feature_schema::{Feature, Registry};

#[derive(Parser, Debug)]
#[command(
    name = "feature-graph",
    about = "Dependency-graph analysis for the Feature Registry",
    version
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
    /// Path to features.registry.json (default: ./features.registry.json)
    #[arg(long, global = true, default_value = "features.registry.json")]
    registry: String,
}

#[derive(Parser, Debug)]
enum Cmd {
    /// Show the features in scope and their dependencies.
    Show {
        /// Limit to a single milestone (e.g. M0). If omitted, shows everything.
        #[arg(long)]
        milestone: Option<String>,
    },
    /// List features in OTHER milestones that block the given milestone.
    Blocks {
        /// Target milestone (e.g. M4).
        #[arg(long)]
        milestone: String,
    },
    /// Longest dependency chain — the critical path through the graph.
    CriticalPath {
        /// Limit to a single milestone; the chain is restricted to features in that milestone.
        #[arg(long)]
        milestone: Option<String>,
    },
    /// Detect cycles (Tarjan strongly connected components with size > 1 or self-loops).
    Cycles,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let path = PathBuf::from(&args.registry);
    let reg = match Registry::load(&path).with_context(|| format!("loading {}", path.display())) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e:#}");
            return ExitCode::from(2);
        }
    };
    match args.cmd {
        Cmd::Show { milestone } => {
            show(&reg, milestone.as_deref());
            ExitCode::from(0)
        }
        Cmd::Blocks { milestone } => {
            blocks(&reg, &milestone);
            ExitCode::from(0)
        }
        Cmd::CriticalPath { milestone } => {
            critical_path(&reg, milestone.as_deref());
            ExitCode::from(0)
        }
        Cmd::Cycles => {
            let g = Graph::build(&reg);
            let sccs = g.tarjan_scc();
            let bad: Vec<&Vec<String>> = sccs
                .iter()
                .filter(|c| c.len() > 1 || g.has_self_loop(c[0].as_str()))
                .collect();
            if bad.is_empty() {
                println!("no cycles detected across {} features", reg.features.len());
                ExitCode::from(0)
            } else {
                println!("{} cycle(s) detected:", bad.len());
                for c in &bad {
                    println!("  {}", c.join(" -> "));
                }
                ExitCode::from(1)
            }
        }
    }
}

/// Adjacency list keyed by feature id.
struct Graph {
    /// Forward edges: id -> set of ids it depends on.
    adj: BTreeMap<String, BTreeSet<String>>,
}

impl Graph {
    fn build(reg: &Registry) -> Self {
        let mut adj: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for f in &reg.features {
            adj.entry(f.id.clone()).or_default();
            for d in &f.depends_on {
                adj.entry(f.id.clone()).or_default().insert(d.clone());
                // ensure the dependency also has a node
                adj.entry(d.clone()).or_default();
            }
        }
        Self { adj }
    }

    /// Reverse edges: id -> set of ids that depend on it.
    fn reverse(&self) -> BTreeMap<String, BTreeSet<String>> {
        let mut r: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (from, tos) in &self.adj {
            r.entry(from.clone()).or_default();
            for to in tos {
                r.entry(to.clone()).or_default().insert(from.clone());
            }
        }
        r
    }

    fn has_self_loop(&self, id: &str) -> bool {
        self.adj.get(id).is_some_and(|s| s.contains(id))
    }

    /// Tarjan strongly connected components. Returns SCCs in discovery order.
    fn tarjan_scc(&self) -> Vec<Vec<String>> {
        // fully owned String keys to avoid lifetime gymnastics across recursion
        fn strongconnect(
            v: String,
            adj: &BTreeMap<String, BTreeSet<String>>,
            indices: &mut HashMap<String, usize>,
            lowlinks: &mut HashMap<String, usize>,
            on_stack: &mut HashSet<String>,
            stack: &mut Vec<String>,
            sccs: &mut Vec<Vec<String>>,
            counter: &mut usize,
        ) {
            indices.insert(v.clone(), *counter);
            lowlinks.insert(v.clone(), *counter);
            *counter += 1;
            stack.push(v.clone());
            on_stack.insert(v.clone());
            if let Some(neigh) = adj.get(&v) {
                for w in neigh {
                    if !indices.contains_key(w) {
                        strongconnect(
                            w.clone(),
                            adj,
                            indices,
                            lowlinks,
                            on_stack,
                            stack,
                            sccs,
                            counter,
                        );
                        let w_low = lowlinks[w];
                        let v_low = lowlinks[&v];
                        if w_low < v_low {
                            lowlinks.insert(v.clone(), w_low);
                        }
                    } else if on_stack.contains(w) {
                        let w_idx = indices[w];
                        let v_low = lowlinks[&v];
                        if w_idx < v_low {
                            lowlinks.insert(v.clone(), w_idx);
                        }
                    }
                }
            }
            if lowlinks[&v] == indices[&v] {
                let mut scc = Vec::new();
                loop {
                    let w = stack.pop().expect("stack non-empty");
                    on_stack.remove(&w);
                    scc.push(w.clone());
                    if w == v {
                        break;
                    }
                }
                sccs.push(scc);
            }
        }

        let mut indices: HashMap<String, usize> = HashMap::new();
        let mut lowlinks: HashMap<String, usize> = HashMap::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        let mut stack: Vec<String> = Vec::new();
        let mut sccs: Vec<Vec<String>> = Vec::new();
        let mut counter: usize = 0;

        for v in self.adj.keys() {
            if !indices.contains_key(v) {
                strongconnect(
                    v.clone(),
                    &self.adj,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut stack,
                    &mut sccs,
                    &mut counter,
                );
            }
        }
        sccs
    }

    /// Longest dependency chain by depth. Returns a Vec<String> of feature ids
    /// from leaf (no deps) to root (depends on the most).
    fn longest_path(&self) -> Vec<String> {
        // memoize depth per node
        fn depth(
            node: &str,
            adj: &BTreeMap<String, BTreeSet<String>>,
            cache: &mut HashMap<String, usize>,
            path: &mut Vec<String>,
        ) -> usize {
            if let Some(&d) = cache.get(node) {
                return d;
            }
            path.push(node.to_string());
            let mut best: usize = 0;
            if let Some(neigh) = adj.get(node) {
                for n in neigh {
                    let d = depth(n, adj, cache, path) + 1;
                    if d > best {
                        best = d;
                    }
                }
            }
            path.pop();
            cache.insert(node.to_string(), best);
            best
        }
        let mut cache: HashMap<String, usize> = HashMap::new();
        let mut path: Vec<String> = Vec::new();
        // pick the node with the highest depth; reconstruct path
        let mut best_node: Option<String> = None;
        let mut best_depth: usize = 0;
        for k in self.adj.keys() {
            let d = depth(k, &self.adj, &mut cache, &mut path);
            if d > best_depth {
                best_depth = d;
                best_node = Some(k.clone());
            }
        }
        // reconstruct the path
        let mut out: Vec<String> = Vec::new();
        if let Some(start) = best_node {
            let mut current = start;
            loop {
                out.push(current.clone());
                let next = self.adj.get(&current).and_then(|s| {
                    s.iter()
                        .max_by_key(|n| cache.get(*n).copied().unwrap_or(0))
                        .cloned()
                });
                match next {
                    Some(n) if cache.get(&n).copied().unwrap_or(0) > 0 => current = n,
                    _ => break,
                }
            }
        }
        out
    }
}

fn index_by_id(reg: &Registry) -> BTreeMap<&str, &Feature> {
    reg.features.iter().map(|f| (f.id.as_str(), f)).collect()
}

fn show(reg: &Registry, milestone: Option<&str>) {
    let g = Graph::build(reg);
    let idx = index_by_id(reg);
    let by_id = &g.adj;
    let scope: Vec<&Feature> = reg
        .features
        .iter()
        .filter(|f| milestone.is_none() || f.milestone == milestone.unwrap())
        .collect();
    println!(
        "scope: {} feature(s){}",
        scope.len(),
        milestone
            .map(|m| format!(" (milestone = {m})"))
            .unwrap_or_default()
    );
    let mut sorted = scope.clone();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for f in sorted {
        let deps = by_id
            .get(f.id.as_str())
            .map(|s| s.iter().cloned().collect::<Vec<_>>().join(", "))
            .unwrap_or_default();
        let milestone_tag = f.milestone.as_str();
        let status_tag = f.status.as_str();
        if deps.is_empty() {
            println!("  [{}|{}]  {}", milestone_tag, status_tag, f.id);
        } else {
            println!("  [{}|{}]  {} -> {}", milestone_tag, status_tag, f.id, deps);
        }
        // resolve each dep to its milestone for quick at-a-glance grouping
        for d in f.depends_on.iter() {
            if let Some(d) = idx.get(d.as_str()) {
                println!("        └─ {} [{}|{}]", d.id, d.milestone, d.status);
            } else {
                println!("        └─ {} [dangling]", d);
            }
        }
    }
}

fn blocks(reg: &Registry, milestone: &str) {
    let g = Graph::build(reg);
    let rev = g.reverse();
    let idx = index_by_id(reg);
    let targets: BTreeSet<&str> = reg
        .features
        .iter()
        .filter(|f| f.milestone == milestone)
        .map(|f| f.id.as_str())
        .collect();
    // walk the reverse graph from the target set, collecting every ancestor not
    // already in the target set itself.
    let mut blockers: BTreeSet<String> = BTreeSet::new();
    let mut frontier: Vec<String> = targets.iter().map(|s| s.to_string()).collect();
    let mut visited: HashSet<String> = targets.iter().map(|s| s.to_string()).collect();
    while let Some(cur) = frontier.pop() {
        if let Some(parents) = rev.get(&cur) {
            for p in parents {
                if visited.insert(p.clone()) {
                    blockers.insert(p.clone());
                    frontier.push(p.clone());
                }
            }
        }
    }
    let other: Vec<&Feature> = blockers
        .iter()
        .filter_map(|id| idx.get(id.as_str()).copied())
        .filter(|f| f.milestone != milestone)
        .collect();
    if other.is_empty() {
        println!("milestone {milestone} has no blockers in other milestones");
        return;
    }
    let mut by_ms: BTreeMap<&str, Vec<&Feature>> = BTreeMap::new();
    for f in &other {
        by_ms.entry(f.milestone.as_str()).or_default().push(f);
    }
    println!(
        "milestone {} is blocked by {} feature(s) in other milestones:",
        milestone,
        other.len()
    );
    for (ms, feats) in by_ms {
        println!("  from {ms}:");
        let mut s = feats.clone();
        s.sort_by(|a, b| a.id.cmp(&b.id));
        for f in s {
            println!("    - {} [{}] {}", f.id, f.status, f.name);
        }
    }
}

fn critical_path(reg: &Registry, milestone: Option<&str>) {
    // Restrict graph to features in the milestone (or whole graph if None).
    let g_all = Graph::build(reg);
    let allowed: BTreeSet<String> = match milestone {
        Some(m) => reg
            .features
            .iter()
            .filter(|f| f.milestone == m)
            .map(|f| f.id.clone())
            .collect(),
        None => g_all.adj.keys().cloned().collect(),
    };
    let mut sub: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for id in &allowed {
        sub.entry(id.clone()).or_default();
        if let Some(deps) = g_all.adj.get(id) {
            for d in deps {
                if allowed.contains(d) {
                    let entry = sub.entry(id.clone()).or_default();
                    entry.insert(d.clone());
                }
            }
        }
    }
    let g = Graph { adj: sub };
    let path = g.longest_path();
    let idx = index_by_id(reg);
    let scope_label = milestone
        .map(|m| format!("milestone {m}"))
        .unwrap_or_else(|| "the whole registry".to_string());
    if path.is_empty() {
        println!("no path found in {scope_label}");
        return;
    }
    println!(
        "critical path in {scope_label}: {} feature(s), {} hop(s)",
        path.len(),
        path.len().saturating_sub(1)
    );
    for (i, id) in path.iter().enumerate() {
        let name = idx.get(id.as_str()).map(|f| f.name.as_str()).unwrap_or("?");
        let ms = idx
            .get(id.as_str())
            .map(|f| f.milestone.as_str())
            .unwrap_or("?");
        let st = idx
            .get(id.as_str())
            .map(|f| f.status.as_str())
            .unwrap_or("?");
        if i + 1 < path.len() {
            println!("  {i:>2}. [{ms}|{st}] {id} — {name} ->");
        } else {
            println!("  {i:>2}. [{ms}|{st}] {id} — {name}");
        }
    }
}
