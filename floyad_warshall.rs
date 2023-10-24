use std::usize::MAX;

const V: usize = 4;

fn floyd_warshall(graph: &mut Vec<Vec<usize>>) {
    for k in 0..V {
        for i in 0..V {
            for j in 0..V {
                if graph[i][k] != MAX && graph[k][j] != MAX && graph[i][k] + graph[k][j] < graph[i][j] {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }
}

fn print_solution(graph: &Vec<Vec<usize>>) {
    for i in 0..V {
        for j in 0..V {
            if graph[i][j] == MAX {
                print!("INF\t");
            } else {
                print!("{}\t", graph[i][j]);
            }
        }
        println!();
    }
}

fn main() {
    let mut graph: Vec<Vec<usize>> = vec![
        vec![0, 5, MAX, 10],
        vec![MAX, 0, 3, MAX],
        vec![MAX, MAX, 0, 1],
        vec![MAX, MAX, MAX, 0],
    ];

    floyd_warshall(&mut graph);

    println!("Shortest distances between every pair of vertices:");
    print_solution(&graph);
}
