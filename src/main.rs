pub mod graph;

use graph::{Graph, N, SUDOKU_DIM};
use std::env;
use std::fs::File;
use std::io::{Read, Write};

const GRAPH_FILE: &'static str = "graph.txt";

fn main() {
    let cwd = env::current_dir()
        .expect("Error retrieving current working directory!")
        .to_str()
        .expect("Generic error!")
        .to_owned();
    let mut fp_sudoku =
        File::open(cwd.clone() + "/" + "sudoku.txt").expect("Unable to open the file!");
    let mut fp_graph =
        File::create(cwd.clone() + "/" + GRAPH_FILE).expect("Unable to open the file!");
    sudoku_to_graph(&mut fp_sudoku, &mut fp_graph);
    fp_graph = File::open(cwd + "/" + GRAPH_FILE).expect("Unable to open the file!");
    let mut g = Graph::new(&mut fp_graph);
    g.solve_sudoku();
    g.print_solution();
}

fn sudoku_to_graph(sudoku_file: &mut File, graph_file: &mut File) {
    let n_vertices = SUDOKU_DIM * SUDOKU_DIM;
    let sudoku = read_sudoku(sudoku_file);
    let err_msg = "Error writing on the file!";
    writeln!(graph_file, "{}", n_vertices).expect(err_msg);
    for i in 0..n_vertices {
        writeln!(
            graph_file,
            "{} {}",
            i,
            sudoku[i / SUDOKU_DIM][i % SUDOKU_DIM]
        )
        .expect(err_msg);
    }
    let mut i = 0;
    let mut j;
    let mut sub;
    let mut vertex_number;
    while i < SUDOKU_DIM {
        j = 0;
        while j < SUDOKU_DIM {
            vertex_number = i * SUDOKU_DIM + j;
            writeln!(
                graph_file,
                "{} {}",
                vertex_number,
                vertex_number + SUDOKU_DIM + 1
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number,
                vertex_number + 2 * SUDOKU_DIM + 2
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + SUDOKU_DIM + 1,
                vertex_number + 2 * SUDOKU_DIM + 2
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 1,
                vertex_number + 2 + SUDOKU_DIM
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + SUDOKU_DIM,
                vertex_number + 2 * SUDOKU_DIM + 1
            )
            .expect(err_msg);

            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 2,
                vertex_number + SUDOKU_DIM + 1
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 2,
                vertex_number + 2 * SUDOKU_DIM
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + SUDOKU_DIM + 1,
                vertex_number + 2 * SUDOKU_DIM
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 1,
                vertex_number + SUDOKU_DIM
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + SUDOKU_DIM + 2,
                vertex_number + 2 * SUDOKU_DIM + 1
            )
            .expect(err_msg);

            writeln!(
                graph_file,
                "{} {}",
                vertex_number,
                vertex_number + SUDOKU_DIM + 2
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number,
                vertex_number + 2 * SUDOKU_DIM + 1
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 1,
                vertex_number + 2 * SUDOKU_DIM
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 1,
                vertex_number + 2 * SUDOKU_DIM + 2
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 2,
                vertex_number + 2 * SUDOKU_DIM + 1
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 2,
                vertex_number + SUDOKU_DIM
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + SUDOKU_DIM,
                vertex_number + 2 * SUDOKU_DIM + 2
            )
            .expect(err_msg);
            writeln!(
                graph_file,
                "{} {}",
                vertex_number + 2 * SUDOKU_DIM,
                vertex_number + SUDOKU_DIM + 2
            )
            .expect(err_msg);
            j += N;
        }
        i += N;
    }

    i = 0;
    while i < SUDOKU_DIM {
        sub = 0;
        while sub < SUDOKU_DIM {
            vertex_number = i * SUDOKU_DIM + sub;
            j = 1;
            while j < SUDOKU_DIM - sub {
                writeln!(graph_file, "{} {}", vertex_number, vertex_number + j).expect(err_msg);
                j += 1;
            }
            vertex_number = i + sub * SUDOKU_DIM;
            j = SUDOKU_DIM;
            while j < SUDOKU_DIM * SUDOKU_DIM - (sub * SUDOKU_DIM) {
                writeln!(graph_file, "{} {}", vertex_number, vertex_number + j).expect(err_msg);
                j += SUDOKU_DIM;
            }
            sub += 1;
        }
        i += 1;
    }
}

fn read_sudoku(sudoku_file: &mut File) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut buf: String = String::new();
    let mut counter: usize = 0;
    sudoku_file
        .read_to_string(&mut buf)
        .expect("Unable to read the file!");
    for line in buf.split_terminator('\n') {
        result.push(Vec::new());
        for str_value in line.split_whitespace() {
            result[counter].push(match str_value.parse::<u8>() {
                Ok(v) => v,
                Err(_) => panic!("Wrong format in input file!"),
            });
        }
        counter += 1;
    }
    result
}
