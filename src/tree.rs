use nalgebra::{distance, Point3, Vector3};

const MIN_DIST: f32 = 2.5;
const MAX_DIST: f32 = 15.0;
const BRANCH_COLOR: u16 = 0b1_01010_00111_00011;
const LEAF_COLOR: u16 = 0b1_01001_10100_00101;
const LEAF_SIZE: usize = 5;

struct Branch {
    from: Point3<f32>,
    to: Point3<f32>,
    dir: Vector3<f32>,
    next_dirs: Vec<Vector3<f32>>,
    leaf: bool,
}

impl Branch {
    fn new(from: Point3<f32>, dir: Vector3<f32>) -> Self {
        Self {
            from,
            to: from + dir,
            dir,
            next_dirs: Vec::new(),
            leaf: true,
        }
    }

    fn next(&mut self) -> Self {
        self.leaf = false;
        let len = self.next_dirs.len() as f32 + 1.0;
        let next_dir = self.next_dirs.drain(..).fold(self.dir, |acc, v| acc + v) / len;
        Self::new(self.to, next_dir)
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
    change_count: usize,
    prev_len: usize,
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
                let dist = distance(&branch.to, leaf);
                if dist < MAX_DIST {
                    done = true;
                    break;
                }
            }

            let branch = branches.last_mut().unwrap().next();
            branches.push(branch);
        }

        for branch in &branches {
            Self::create_branch_thickness(branch.from, branch.to, 6, data, size);
        }

        Tree {
            leaves,
            branches,
            width: 5.0,
            change_count: 0,
            prev_len: 0,
        }
    }

    fn create_branch(from: Point3<f32>, to: Point3<f32>, data: &mut Vec<u16>, size: usize) {
        let delta = (to - from.coords).coords.normalize();
        let mut pos = from;

        loop {
            data[from.z.round() as usize * size * size
                + from.y.round() as usize * size
                + from.x.round() as usize] = BRANCH_COLOR;

            pos += delta;
            if pos.x.abs() >= to.x.abs() || pos.y.abs() >= to.y.abs() || pos.z.abs() >= to.z.abs() {
                break;
            }
        }
    }

    fn create_branch_thickness(from: Point3<f32>, to: Point3<f32>, thickness: usize, data: &mut Vec<u16>, size: usize) {
        let delta = to - from.coords;

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
                if delta.x.abs() < delta.y.abs() || delta.x.abs() < delta.z.abs() {
                    if delta.y.abs() < delta.z.abs() {
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

                Self::create_branch(from + offset, to + offset, data, size);
            }
        }
    }

    fn create_leaves(&mut self, data: &mut Vec<u16>, size: usize) {
        for branch in &self.branches {
            if !branch.leaf {
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

                        data[((branch.to.z.round() - 2.0) as usize + z) * size * size
                            + ((branch.to.y.round() - 2.0) as usize + y) * size
                            + (branch.to.x.round() - 2.0) as usize
                            + x] = LEAF_COLOR;
                    }
                }
            }
        }
    }

    pub fn grow(&mut self, data: &mut Vec<u16>, size: usize) {
        let branches = &self.branches;
        self.leaves.retain(|leaf| {
            branches.iter().fold(std::f32::MAX, |acc, branch| {
                distance(&branch.to, leaf).min(acc)
            }) > MIN_DIST
        });

        for leaf in &self.leaves {
            let branch = self
                .branches
                .iter_mut()
                .fold((std::f32::MAX, None), |acc, branch| {
                    let dist = distance(&branch.to, leaf);
                    if dist < acc.0 {
                        (dist, Some(branch))
                    } else {
                        acc
                    }
                })
                .1
                .unwrap();

            let dir = (leaf - branch.to).normalize();
            branch.add(dir);
        }

        let new_branches: Vec<_> = self
            .branches
            .iter_mut()
            .filter_map(|x| x.next_option())
            .collect();

        for branch in &new_branches {
            Self::create_branch_thickness(branch.from, branch.to, self.width as usize, data, size);
        }

        self.branches.extend(new_branches);
        if self.branches.len() == self.prev_len {
            self.change_count += 1;
        } else {
            self.prev_len = self.branches.len();
            self.change_count = 0;
        }

        if self.change_count >= 5 {
            self.create_leaves(data, size);
        }

        self.width -= 0.05;
    }
}
