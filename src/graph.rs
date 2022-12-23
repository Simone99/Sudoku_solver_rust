use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

pub const N: usize = 3;
pub const SUDOKU_DIM: usize = N * N;

struct Content {
    vertex_number: usize,
    color: usize,
    available_colors: Vec<bool>,
    removed: bool,
}

pub struct Graph {
    adj_list: Vec<Vec<Rc<RefCell<Content>>>>,
    vertices: Vec<Rc<RefCell<Content>>>,
    v: u8,
    e: u16,
}

fn str_to_tuple(str_tuple: &str) -> Option<(usize, usize)> {
    let mut iter = str_tuple.split_whitespace();
    let a = match iter.next()?.trim().parse::<usize>() {
        Ok(v) => v,
        Err(_) => panic!("Wrong edge list file format!"),
    };
    let b = match iter.next()?.trim().parse::<usize>() {
        Ok(v) => v,
        Err(_) => panic!("Wrong edge list file format!"),
    };
    Some((a, b))
}

impl Graph {
    pub fn new(fp: &mut File) -> Graph {
        let mut tmp = Graph {
            adj_list: Vec::new(),
            vertices: Vec::new(),
            v: 0,
            e: 0,
        };
        let err_msg = "Error in graph file iterator!";
        let mut buf = String::new();
        fp.read_to_string(&mut buf)
            .expect("Unable to read the file!");
        let mut graph_file_iter = buf.split_terminator('\n');
        let v = match graph_file_iter.next().expect(err_msg).trim().parse::<u8>() {
            Ok(v) => v,
            Err(_) => panic!("Wrong edge list file format!"),
        };
        tmp.v = v;
        for _ in 0..v {
            let (a, b) = str_to_tuple(graph_file_iter.next().expect(err_msg))
                .expect("Error in the format of the file!");
            tmp.adj_list.push(Vec::new());
            tmp.vertices.push(Rc::new(RefCell::new(Content {
                vertex_number: a,
                color: b,
                available_colors: Vec::from([true; SUDOKU_DIM]),
                removed: false,
            })));
        }
        loop {
            let tmp_str_value = match graph_file_iter.next() {
                Some(v) => v,
                None => break tmp,
            };
            let (a, b) = str_to_tuple(tmp_str_value).expect("Error in the format of the file!");
            tmp.adj_list[a].push(Rc::clone(&tmp.vertices[b]));
            tmp.adj_list[b].push(Rc::clone(&tmp.vertices[a]));
            tmp.e = tmp.e + 1;
        }
    }

    fn precoloringext_to_listcoloring(&mut self) {
        for vertex in self.vertices.iter() {
            let mut vertex_borrowed = vertex.borrow_mut();
            if vertex_borrowed.color == 0 {
                for neighbor_v in self.adj_list[vertex_borrowed.vertex_number].iter() {
                    let neighbot_borrowed = neighbor_v.borrow();
                    if neighbot_borrowed.color != 0 {
                        vertex_borrowed.available_colors[neighbot_borrowed.color - 1] = false;
                    }
                }
            } else {
                vertex_borrowed.removed = true;
            }
        }
    }

    fn list_coloring_recursive(&mut self, vertex_number: usize, found: &mut bool) {
        if vertex_number >= 81 {
            *found = true;
            return;
        }
        if !self.vertices[vertex_number].borrow().removed {
            let tmp_vec = self.vertices[vertex_number]
                .borrow()
                .available_colors
                .clone();
            for (color_index, is_color_available) in tmp_vec.iter().enumerate() {
                if *found {
                    break;
                }
                if *is_color_available {
                    self.vertices[vertex_number].borrow_mut().color = color_index + 1;
                    let mut vertices_to_restore: Vec<usize> = Vec::new();
                    for neighbor_v in self.adj_list[vertex_number].iter() {
                        if !neighbor_v.borrow().removed
                            && neighbor_v.borrow().available_colors[color_index]
                        {
                            neighbor_v.borrow_mut().available_colors[color_index] = false;
                            vertices_to_restore.push(neighbor_v.borrow().vertex_number);
                        }
                    }
                    self.list_coloring_recursive(vertex_number + 1, found);
                    if !*found {
                        self.vertices[vertex_number].borrow_mut().color = 0;
                        for vertex_to_restore in vertices_to_restore {
                            self.vertices[vertex_to_restore]
                                .borrow_mut()
                                .available_colors[color_index] = true;
                        }
                    }
                }
            }
        } else {
            self.list_coloring_recursive(vertex_number + 1, found);
        }
    }

    fn list_coloring(&mut self) {
        let mut found: bool = false;
        self.list_coloring_recursive(0, &mut found);
    }

    pub fn print_solution(&self) {
        let mut tmp;
        for i in 0..SUDOKU_DIM {
            for j in 0..(SUDOKU_DIM - 1) {
                tmp = self.vertices[i * SUDOKU_DIM + j].borrow().color;
                print!("{tmp}");
                print!(" ")
            }
            tmp = self.vertices[i * SUDOKU_DIM + SUDOKU_DIM - 1]
                .borrow()
                .color;
            print!("{tmp}");
            println!()
        }
    }

    pub fn solve_sudoku(&mut self) {
        self.precoloringext_to_listcoloring();
        self.list_coloring();
    }
}
