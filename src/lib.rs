use std::{fs, cmp::PartialEq, char};


pub fn run(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let codepad = Vec2d::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
    

    let mut final_code = String::new();//Final 4-ints code
    let s = fs::read_to_string(filename)?;//File content


    let instruction_collection = lines_iterates(s);//Cuts the files in instructions lines

    for instruct in &instruction_collection {
        let char: char;
        //final_code.push(char);
    }


    Ok(final_code)
}

fn lines_iterates(s: String) -> Vec<String>{
    let mut results: Vec<String> = vec![];
    for i in s.split("\r\n") {
        results.push(i.to_owned());
    }
    results //lines
} 


struct Vec2d<T>{
    vec: Vec<T>,
    rows: usize,
    cols: usize,
}

struct Vec2dIndex {
    row: usize,
    col: usize,
}

impl<T: PartialEq + Copy> Vec2d<T> {
    fn new(vec: Vec<T>, rows: usize, cols: usize) -> Self {
        assert!(once(&vec));//Asserts the code pad has different numbers (to avoid get_index from returning pos of just one specimen of the predicate) 
        assert!(rows*cols == vec.len());
        Self{vec, rows, cols}
    }

    fn index(&self, row_index: usize, col_index: usize) -> &T {
        let i = self.cols * row_index;
        &(self.vec[i + col_index])
    }

    fn safe_index(&self, row_index: usize, col_index: usize) -> Option<T> {
        let i = self.cols * row_index;
        safety(self, self.vec.get(i + col_index))
    }

    fn get_index(&self, elt: T) -> Vec2dIndex {//Returns the position of the elt in the Vec2d (0-starting values)
        let (mut row, mut col) = (0, 0);
        for i in self.vec.iter() {
            if *i == elt {
                break;
            }
            col+=1;
            if (col+1) > self.cols{
                col = 0;
                row+=1;
            }
        }
        Vec2dIndex {row, col}
    }
}

//fn follow_instructs(instruction_line: &str, codepad: Vec2d<i32>) -> char {
//    let mut current: i32;
//    let instructions = instruction_line.chars();
//    for i in instructions {
//        match i {
//            'D' => ,
//            'U' => ,
//            'L' => ,
//            'R' => ,
//        }
//    }
//   'c'
//}

fn safety<T: Copy + PartialEq>(vec2d: &Vec2d<T>, elt: Option<&T>) -> Option<T> {
    match elt {
        Some(smthg)/*If `.get()` returns a value (index is valid then)*/ => {
            let position = vec2d.get_index(*smthg);
            if position.col+1 > vec2d.cols || position.row+1 > vec2d.rows {//If it goes beyond the row/col limit
                return None;
            }
            return Some(*smthg)//The element 
        },
        _ => return None,//If the Option<> is None return None (so in the `safe_index()` usecase, if `.get(index)` returns None)
    }
}

fn once<T: PartialEq + Copy>(vec: &Vec<T>) -> bool {
    let mut list: Vec<T> = vec![];
    for i in vec{
        if list.contains(i) {
            return false;
        }
        list.push(*i)
    }
    true
}



#[cfg(test)]
mod tests {
    use crate::{Vec2d, safety};

    #[test]
    //#[ignore = "reason"]
    fn test() {
        let vec = Vec2d::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(vec.safe_index(0, 3).unwrap(), 4);
    }
}