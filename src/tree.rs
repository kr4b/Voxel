use nalgebra::{distance, Point3, Vector3};

const MIN_DIST: f32 = 2.5;
const MAX_DIST: f32 = 15.0;
const BRANCH_COLOR: u16 = 0b1_01010_00111_00011;
const LEAF_COLOR: u16 = 0b1_01001_10100_00101;
const LEAF_SIZE: usize = 5;

struct Branch {
    pos: Point3<f32>,
    dir: Vector3<f32>,
    next_dirs: Vec<Vector3<f32>>,
    leaf: bool,
}

impl Branch {
    fn new(from: Point3<f32>, dir: Vector3<f32>) -> Self {
        Self {
            pos: from + dir,
            dir,
            next_dirs: Vec::new(),
            leaf: true,
        }
    }

    fn next(&mut self) -> Self {
        self.leaf = false;
        let len = self.next_dirs.len() as f32 + 1.0;
        let next_dir = self.next_dirs.drain(..).fold(self.dir, |acc, v| acc + v) / len;
        Self::new(self.pos, next_dir.normalize())
    }

    fn next_option(&mut self) -> Option<Self> {
        if self.next_dirs.is_empty() {
            return None;
        }

        Some(self.next())
    }

    fn add(&mut self, dir: Vector3<f32>) {
        self.next_dirs.push(dir);
    }
}

pub struct Tree {
    leaves: Vec<Point3<f32>>,
    branches: Vec<Branch>,
    width: f32,
    done: bool,
}

impl Tree {
    pub fn new(
        center: Point3<f32>,
        radius: f32,
        leaves: usize,
        start: Point3<f32>,
        data: &mut Vec<u16>,
        size: usize,
    ) -> Self {
        let mut leaves = Vec::with_capacity(leaves);
        for _ in 0..leaves.capacity() {
            let dir = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
            );
            let dir = dir / dir.dot(&dir).sqrt();
            let len = rand::random::<f32>();
            let pos = center + dir * radius * len;

            leaves.push(pos);
        }

        let mut branches = vec![Branch::new(start, Vector3::new(0.0, 1.0, 0.0))];
        let mut done = false;
        while !done {
            let branch = branches.last().unwrap();
            for leaf in &leaves {
                let dist = distance(&branch.pos, leaf);
                if dist < MAX_DIST {
                    done = true;
                    break;
                }
            }

            let branch = branches.last_mut().unwrap().next();
            branches.push(branch);
        }

        for branch in &branches {
            Self::create_branch_thickness(branch.pos, branch.dir, 6, data, size);
        }

        Tree {
            leaves,
            branches,
            width: 5.0,
            done: false,
        }
    }

    fn create_branch(pos: Point3<f32>, data: &mut Vec<u16>, size: usize) {
        data[pos.z.round() as usize * size * size
            + pos.y.round() as usize * size
            + pos.x.round() as usize] = BRANCH_COLOR;
    }

    fn create_branch_thickness(
        pos: Point3<f32>,
        dir: Vector3<f32>,
        thickness: usize,
        data: &mut Vec<u16>,
        size: usize,
    ) {
        for i in 0..thickness + 1 - (thickness & 1) {
            for j in 0..thickness + 1 - (thickness & 1) {
                if (thickness & 1) == 0 {
                    let c = (i == 0) as usize
                        + (j == 0) as usize
                        + (i == thickness - (thickness & 1)) as usize
                        + (j == thickness - (thickness & 1)) as usize;

                    if c >= 2 {
                        continue;
                    }
                }

                let start = (thickness as f32 / 2.0).floor();
                let mut offset = Vector3::zeros();
                if dir.x.abs() < dir.y.abs() || dir.x.abs() < dir.z.abs() {
                    if dir.y.abs() < dir.z.abs() {
                        offset += Vector3::x() * (i as f32 - start);
                        offset += Vector3::y() * (j as f32 - start);
                    } else {
                        offset += Vector3::x() * (i as f32 - start);
                        offset += Vector3::z() * (j as f32 - start);
                    }
                } else {
                    offset += Vector3::y() * (i as f32 - start);
                    offset += Vector3::z() * (j as f32 - start);
                }

                Self::create_branch(pos + offset, data, size);
            }
        }
    }

    fn check_weight(data: &mut Vec<u16>, size: usize, x: usize, y: usize, z: usize) -> usize {
        let mut weight = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    let x = x as i32 + i;
                    let y = y as i32 + j;
                    let z = z as i32 + k;
                    if x < 0
                        || y < 0
                        || z < 0
                        || x >= size as i32
                        || y >= size as i32
                        || z >= size as i32
                        || (i == 0 && j == 0 && k == 0)
                    {
                        continue;
                    }

                    let current =
                        3 - i.abs() as usize - j.abs() as usize - k.abs() as usize;
                    if data[z as usize * size * size + y as usize * size + x as usize] > 0 {
                        weight += current;
                    }
                }
            }
        }

        weight
    }

    pub fn create_leaves(&mut self, data: &mut Vec<u16>, size: usize) {
        for branch in &self.branches {
            let x = branch.pos.x.round() as usize;
            let y = branch.pos.y.round() as usize;
            let z = branch.pos.z.round() as usize;
            if !branch.leaf || Self::check_weight(data, size, x, y, z) > 3 {
                continue;
            }

            for x in 0..LEAF_SIZE {
                for y in 0..LEAF_SIZE {
                    for z in 0..LEAF_SIZE {
                        let c = (x == 0) as usize
                            + (y == 0) as usize
                            + (z == 0) as usize
                            + (x == LEAF_SIZE - 1) as usize
                            + (y == LEAF_SIZE - 1) as usize
                            + (z == LEAF_SIZE - 1) as usize;

                        if c >= 2 {
                            continue;
                        }

                        data[((branch.pos.z.round() - 2.0) as usize + z) * size * size
                            + ((branch.pos.y.round() - 2.0) as usize + y) * size
                            + (branch.pos.x.round() - 2.0) as usize
                            + x] = LEAF_COLOR;
                    }
                }
            }
        }
    }

    pub fn grow(&mut self, data: &mut Vec<u16>, size: usize) {
        if self.done {
            return;
        }

        if self.width < 1.0 {
            self.create_leaves(data, size);
            self.done = true;
        }

        let branches = &self.branches;
        self.leaves.retain(|leaf| {
            branches.iter().fold(std::f32::MAX, |acc, branch| {
                distance(&branch.pos, leaf).min(acc)
            }) > MIN_DIST
        });

        for leaf in &self.leaves {
            let branch = self
                .branches
                .iter_mut()
                .fold((std::f32::MAX, None), |acc, branch| {
                    let dist = distance(&branch.pos, leaf);
                    if dist < acc.0 {
                        (dist, Some(branch))
                    } else {
                        acc
                    }
                })
                .1
                .unwrap();

            let dir = (leaf - branch.pos).normalize();
            branch.add(dir);
        }

        let new_branches: Vec<_> = self
            .branches
            .iter_mut()
            .filter_map(|x| x.next_option())
            .collect();

        for branch in &new_branches {
            Self::create_branch_thickness(branch.pos, branch.dir, self.width as usize, data, size);
        }

        self.branches.extend(new_branches);

        self.width -= 0.08;
    }
}
