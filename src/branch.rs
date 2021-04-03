use nalgebra::{Point3, Rotation3, Vector3};

const BRANCH_COLOR: u16 = 0b1_01010_00111_00011;
const LEAF_COLOR: u16 = 0b1_01001_10100_00101;
const LEAF_SIZE: usize = 5;

pub enum Sequence {
    /// (Side branches, side branch length, center branch length, center branch thickness, rotation offset),
    /// Branch sequence, Center sequences
    Branch((usize, f32, f32, usize, usize), Box<Sequence>, Box<Sequence>),
    /// Leaf
    Leaf,
}

pub struct Branch {
    pub from: Point3<f32>,
    pub to: Point3<f32>,
    pub rotation: Rotation3<f32>,
    pub thickness: usize,
    pub extended: bool,
    pub branches: Vec<Branch>,
}

impl Branch {
    pub fn new(from: Point3<f32>, len: f32, thickness: usize, rotation: Rotation3<f32>) -> Self {
        let to = from + (rotation * Point3::new(0.0, len, 0.0)).coords;

        Self {
            from,
            to,
            rotation,
            thickness,
            extended: false,
            branches: Vec::new(),
        }
    }

    fn create_branch(from: Point3<f32>, to: Point3<f32>, data: &mut Vec<u16>, size: usize) {
        let delta = to - from.coords;
        let mut x = from.x.round();
        let mut y = from.y.round();
        let mut z = from.z.round();

        loop {
            data[z.round() as usize * size * size
                + y.round() as usize * size
                + x.round() as usize] = BRANCH_COLOR;

            if delta.x.abs() > delta.y.abs() && delta.x.abs() > delta.z.abs() {
                x += delta.x.signum();
                y += delta.y.signum() * (delta.y / delta.x).abs();
                z += delta.z.signum() * (delta.z / delta.x).abs();
                if (x - to.x.round()).abs() < 0.1 {
                    break;
                }
            } else if delta.y.abs() > delta.z.abs() {
                y += delta.y.signum();
                x += delta.x.signum() * (delta.x / delta.y).abs();
                z += delta.z.signum() * (delta.z / delta.y).abs();
                if (y - to.y.round()).abs() < 0.1 {
                    break;
                }
            } else {
                z += delta.z.signum();
                x += delta.x.signum() * (delta.x / delta.z).abs();
                y += delta.y.signum() * (delta.y / delta.z).abs();
                if (z - to.z.round()).abs() < 0.1 {
                    break;
                }
            }
        }
    }

    fn create_branches(&mut self, amount: usize, len: f32, offset: usize) {
        for i in 0..amount {
            let yaw = i * 360 / amount + offset;

            let rotation = Rotation3::from_euler_angles(
                -65.0 / 180.0 * std::f32::consts::PI,
                yaw as f32 / 180.0 * std::f32::consts::PI,
                0.0,
            );

            let branch = Branch::new(self.to, len, 1, self.rotation * rotation);
            self.branches.push(branch);
        }
    }

    fn extend_children(&mut self, sequence: &Sequence, data: &mut Vec<u16>, size: usize) {
        match sequence {
            Sequence::Branch((amount, _, center_len, ..), branch_seq, center_seq) => {
                for i in 0..*amount {
                    self.branches[i].extend(&branch_seq, data, size)
                }
                if *center_len != 0.0 {
                    self.branches[*amount].extend(&center_seq, data, size);
                }
            }
            _ => (),
        }
    }

    fn extend_self(&mut self, sequence: &Sequence, data: &mut Vec<u16>, size: usize) {
        match sequence {
            Sequence::Branch((amount, branch_len, center_len, center_thickness, offset), ..) => {
                self.create_branches(*amount, *branch_len, *offset);
                if *center_len != 0.0 {
                    let branch = Branch::new(self.to, *center_len, *center_thickness, self.rotation);
                    self.branches.push(branch);
                }
            }
            Sequence::Leaf => {
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

                            data[((self.to.z.round() - 2.0) as usize + z) * size * size
                                + ((self.to.y.round() - 2.0) as usize + y) * size
                                + (self.to.x.round() - 2.0) as usize
                                + x] = LEAF_COLOR;
                        }
                    }
                }
            }
        }
    }

    pub fn extend(&mut self, sequence: &Sequence, data: &mut Vec<u16>, size: usize) {
        if self.extended {
            self.extend_children(sequence, data, size);
        } else {
            self.extended = true;
            let delta = self.to - self.from.coords;

            for i in 0..self.thickness + 1 - (self.thickness & 1) {
                for j in 0..self.thickness + 1 - (self.thickness & 1) {
                    if (self.thickness & 1) == 0 {
                        let c = (i == 0) as usize
                            + (j == 0) as usize
                            + (i == self.thickness - (self.thickness & 1)) as usize
                            + (j == self.thickness - (self.thickness & 1)) as usize;

                        if c >= 2 {
                            continue;
                        }
                    }

                    let start = (self.thickness as f32 / 2.0).floor();
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

                    Self::create_branch(self.from + offset, self.to + offset, data, size);
                }
            }
            self.extend_self(sequence, data, size);
        }
    }

    // pub fn extend(&mut self, data: &mut Vec<u16>, size: usize) {
    //     if self.extended {
    //         for x in 0..3 {
    //             for y in 0..3 {
    //                 for z in 0..3 {
    //                     data[((self.to.z.round() - 1.0) as usize + z) * size * size
    //                         + ((self.to.y.round() - 1.0) as usize + y) * size
    //                         + (self.to.x.round() - 1.0) as usize
    //                         + x] = 0;
    //                 }
    //             }
    //         }

    //         Self::create_branch(self.from, self.to, data, size);

    //         for branch in &mut self.branches {
    //             branch.extend(data, size);
    //         }
    //     } else {
    //         self.extended = true;
    //         Self::create_branch(self.from, self.to, data, size);

    //         for x in 0..3 {
    //             for y in 0..3 {
    //                 for z in 0..3 {
    //                     data[((self.to.z.round() - 1.0) as usize + z) * size * size
    //                         + ((self.to.y.round() - 1.0) as usize + y) * size
    //                         + (self.to.x.round() - 1.0) as usize
    //                         + x] = LEAF_COLOR;
    //                 }
    //             }
    //         }

    //         for _ in 0..3 {
    //             let len = rand::random::<f32>() * 8.0 + 2.0;
    //             let yaw = rand::random::<f32>() * 360.0;

    //             let rotation = Rotation3::from_euler_angles(
    //                 -30.0 / 180.0 * std::f32::consts::PI,
    //                 yaw / 180.0 * std::f32::consts::PI,
    //                 0.0,
    //             );
    //             let branch = Branch::new(self.to, len, self.rotation * rotation);
    //             self.branches.push(branch);
    //         }
    //     }
    // }
}
