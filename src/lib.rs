use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Default)]
pub struct Graph {
    pub points: Vec<Point>,
    pub edges: BTreeSet<(Point, Point)>,
}

impl Graph {
    pub fn add_edge(&mut self, p1: Point, p2: Point) {
        self.edges.insert((p1, p2));
        self.edges.insert((p2, p1));
    }
}

#[derive(Default)]
pub struct ActiveSet {
    points: BTreeMap<i64, BTreeSet<i64>>,
}

pub enum Region {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl ActiveSet {
    pub fn p_at_region_looking_from_response(&self, p: Point, region: Region) -> Vec<Point> {
        let mut res = vec![];
        match region {
            Region::R1 => {
                for (x, ys) in self.points.range(..p.x).rev() {
                    for y in ys.range(..x + p.y - p.x) {
                        res.push(Point{x: *x,y: *y});
                    }
                }
            },
            Region::R2 => {
                for (x, ys) in self.points.range(..p.x).rev() {
                    for y in ys.range(x - p.x + p.y..p.y).rev() {
                        res.push(Point{x: *x,y: *y});
                    }
                }
            },
            Region::R3 => {
                for (x, ys) in self.points.range(..p.x).rev() {
                    for y in ys.range(p.x + p.y + *x ..) {
                        res.push(Point{x: *x, y: *y});
                    }
                }
            },
            Region::R4 => {
                for (x, ys) in self.points.range(..p.x).rev() {
                    for y in ys.range(p.y .. p.x + p.y + *x) {
                        res.push(Point{x: *x, y: *y});
                    }
                }
            }
            _ => unimplemented!(),
        }
        res
    }

    pub fn remove(&mut self, p: Point) {
        let mut need_delete = false;
        if let Some(ys) = self.points.get_mut(&p.x) {
            ys.remove(&p.y);
            if ys.len() == 0 {
                need_delete = true;
            }
        };
        if need_delete {
            self.points.remove(&p.x);
        }
    }
    pub fn add(&mut self, p: Point) {
        let res = self.points.entry(p.x).or_insert(BTreeSet::new());
        res.insert(p.y);
    }
}
pub fn build(points: Vec<Point>) -> Graph {
    let mut res = Graph::default();
    let mut sweep_line = points.clone();
    sweep_line.sort_by_key(|p| p.x + p.y);
    let mut active_set = ActiveSet::default();
    let mut active_set_r2 = ActiveSet::default();
    for p in sweep_line.iter() {
        let s = active_set.p_at_region_looking_from_response(*p, Region::R1);
        let s_r2 = active_set_r2.p_at_region_looking_from_response(*p, Region::R2);
        for p2 in s {
            res.add_edge(*p, p2);
            active_set.remove(p2);
        }
        for p2 in s_r2 {
            res.add_edge(*p, p2);
            active_set_r2.remove(p2);
        }
        active_set.add(*p);
        active_set_r2.add(*p);
    }
    active_set = ActiveSet::default();
    active_set_r2 = ActiveSet::default();
    sweep_line.sort_by_key(|p| p.x - p.y);
    for p in sweep_line.iter() {
        let s = active_set.p_at_region_looking_from_response(*p, Region::R3);
        let s_r2 = active_set_r2.p_at_region_looking_from_response(*p, Region::R4);
        for p2 in s {
            res.add_edge(*p, p2);
            active_set.remove(p2);
        }
        for p2 in s_r2 {
            res.add_edge(*p, p2);
            active_set_r2.remove(p2);
        }
        active_set.add(*p);
        active_set_r2.add(*p);
    }
    res
}


#[test]
pub fn unit_test_a() {
    let pts = vec![Point{x: 0, y: 0}, Point{x: 1, y: 1}, Point{x: 4, y: 5}, Point{x: 1, y: 4}, Point{x: 1, y: 9}, Point{x: 8, y: 1}, Point{x: 0, y: 8}, Point{x: 9, y: 3}];
    println!("{:?}", build(pts));
}