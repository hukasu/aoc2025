use std::collections::HashMap;

pub struct ServerRackConnections<'a> {
    server_map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> ServerRackConnections<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            server_map: input
                .lines()
                .map(|line| {
                    let Some((machine, outputs)) = line.split_once(": ") else {
                        unreachable!(
                            "Line must contain a machine and its outputs separated by ': '."
                        );
                    };
                    (machine, outputs.split(" ").collect())
                })
                .collect(),
        }
    }

    pub fn connections_to_from(&self, start: &'a str, target: &'a str) -> usize {
        let mut memoizer = HashMap::new();
        Self::connections_dynamic_programming(start, target, &self.server_map, &mut memoizer)
    }

    pub fn paths_through(
        &self,
        start: &'a str,
        target: &'a str,
        passing_through: &[&str],
    ) -> usize {
        let mut memoizer = HashMap::new();
        Self::paths_through_dynamic_programming(
            "".to_owned(),
            start,
            target,
            passing_through,
            &self.server_map,
            &mut memoizer,
        )
    }

    fn connections_dynamic_programming(
        current_machine: &'a str,
        target: &'a str,
        server_map: &HashMap<&'a str, Vec<&'a str>>,
        memoizer: &mut HashMap<&'a str, usize>,
    ) -> usize {
        match memoizer.get(current_machine) {
            Some(paths) => *paths,
            None => {
                let Some(connections) = server_map.get(current_machine) else {
                    unreachable!("All machines must be part of the server map.");
                };
                let paths = connections
                    .iter()
                    .map(|connection| {
                        if *connection == target {
                            1
                        } else {
                            Self::connections_dynamic_programming(
                                connection, target, server_map, memoizer,
                            )
                        }
                    })
                    .sum();
                memoizer.insert(current_machine, paths);
                paths
            }
        }
    }

    fn paths_through_dynamic_programming(
        current_path: String,
        current_machine: &'a str,
        target: &'a str,
        passing_through: &[&str],
        server_map: &HashMap<&'a str, Vec<&'a str>>,
        memoizer: &mut HashMap<(&'a str, String), usize>,
    ) -> usize {
        match memoizer.get(&(current_machine, passing_through.join(""))) {
            Some(paths) => *paths,
            None => {
                let Some(connections) = server_map.get(current_machine) else {
                    unreachable!("All machines must be part of the server map.");
                };

                let mut passing_through = passing_through.to_vec();
                if let Some(position) = passing_through
                    .iter()
                    .position(|machine| current_machine == *machine)
                {
                    passing_through.remove(position);
                }

                let paths = connections
                    .iter()
                    .map(|connection| {
                        if *connection == target {
                            passing_through.is_empty() as usize
                        } else {
                            let new_path = format!("{current_path}{current_machine}");
                            Self::paths_through_dynamic_programming(
                                new_path,
                                connection,
                                target,
                                &passing_through,
                                server_map,
                                memoizer,
                            )
                        }
                    })
                    .sum();
                memoizer.insert((current_machine, passing_through.join("")), paths);
                paths
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const INPUT2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_connections() {
        assert_eq!(
            ServerRackConnections::parse(INPUT).connections_to_from("you", "out"),
            5
        );
    }

    #[test]
    fn test_paths_through() {
        assert_eq!(
            ServerRackConnections::parse(INPUT2).paths_through("svr", "out", &["dac", "fft"]),
            2
        );
    }
}
