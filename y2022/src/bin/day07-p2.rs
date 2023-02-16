use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day07.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut current_dir_idx = 0;
    let mut f_system = FileaaaSystem::new(Fileaaa::new(String::from("/"), 0, true));

    for line in lines {
        let line = line.unwrap();

        let mut parts = line.split(" ");

        match parts.next() {
            Some("$") => match parts.next() {
                Some("cd") => match parts.next() {
                    Some("..") => {
                        current_dir_idx = f_system.get(current_dir_idx).p_idx;
                    }
                    Some(dir_name) => {
                        let id = f_system
                            .get(current_dir_idx)
                            .children
                            .iter()
                            .find(|&&id| f_system.get(id).name == dir_name);
                        if id.is_some() {
                            current_dir_idx = *id.unwrap();
                        } else {
                            current_dir_idx = 0;
                        }
                    }
                    _ => {}
                },
                Some("ls") => {} // do nothing
                _ => {}          // do nothing
            },
            Some("dir") => {
                let dir_name = parts.next().unwrap();
                let dir_name = String::from(dir_name);
                let dir = Fileaaa::new(dir_name, 0, true);

                f_system.add_as_child(current_dir_idx, dir);
            } // dir
            Some(size) => {
                let size = size.parse::<usize>().unwrap();
                let name = String::from(parts.next().unwrap());
                let f = Fileaaa::new(name, size, false);

                f_system.add_as_child(current_dir_idx, f);
            } // file
            _ => {}
        }
    }

    let mut current_min_dir_size = usize::MAX;
    let root_dir_size = f_system.size_of(0);
    for f in &f_system.f_arr {
        if f.is_dir {
            let size = f_system.size_of(f.idx);
            if size + (70000000 - root_dir_size) >= 30000000 {
                if size < current_min_dir_size {
                    current_min_dir_size = size;
                }
            }
        }
    }
    println!("min size : {:?}", current_min_dir_size);
}

struct Fileaaa {
    idx: usize,
    p_idx: usize,
    name: String,
    size: usize,
    is_dir: bool,
    children: Vec<usize>,
}

impl Fileaaa {
    fn new(name: String, size: usize, is_dir: bool) -> Self {
        Fileaaa {
            idx: 0,
            p_idx: 0,
            name,
            size,
            is_dir,
            children: vec![],
        }
    }

    fn append_child(&mut self, child: &mut Fileaaa) {
        self.children.push(child.idx);
        child.p_idx = self.idx;
    }
}

struct FileaaaSystem {
    f_arr: Vec<Fileaaa>,
}

impl FileaaaSystem {
    fn new(f: Fileaaa) -> Self {
        FileaaaSystem { f_arr: vec![f] }
    }

    fn add_as_child(&mut self, p_idx: usize, mut f: Fileaaa) -> usize {
        f.idx = self.f_arr.len();

        let result = f.idx;
        if self.f_arr.len() > 0 {
            self.f_arr[p_idx].append_child(&mut f);
        }
        self.f_arr.push(f);

        result
    }

    fn get(&self, idx: usize) -> &Fileaaa {
        &self.f_arr[idx]
    }

    fn size_of(&self, idx: usize) -> usize {
        let mut result = 0;
        let f = self.get(idx);
        if f.is_dir {
            for child_idx in &f.children {
                result += self.size_of(*child_idx);
            }
        } else {
            result += f.size;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fileaaa, FileaaaSystem};

    #[test]
    fn dir_system_test() {
        let root = Fileaaa::new(String::from("/"), 0, true);
        let mut system = FileaaaSystem::new(root);

        let child1 = Fileaaa::new(String::from("1"), 0, true);
        let child1_1 = Fileaaa::new(String::from("1-1"), 200, false);

        let child2 = Fileaaa::new(String::from("2"), 0, true);

        let child3 = Fileaaa::new(String::from("3"), 500, false);

        let child1_idx = system.add_as_child(0, child1);
        let child1_1_idx = system.add_as_child(child1_idx, child1_1);

        let child2_idx = system.add_as_child(0, child2);
        let child3_idx = system.add_as_child(0, child3);

        assert_eq!(system.get(child1_idx).p_idx, 0);
        assert_eq!(system.get(child1_1_idx).p_idx, 1);
        assert_eq!(system.get(child2_idx).p_idx, 0);
        assert_eq!(system.get(child3_idx).p_idx, 0);

        assert_eq!(system.size_of(0), 700);
        assert_eq!(system.size_of(child1_idx), 200);
        assert_eq!(system.size_of(child1_1_idx), 200);

        assert_eq!(system.size_of(child2_idx), 0);
        assert_eq!(system.size_of(child3_idx), 500);
    }
}
