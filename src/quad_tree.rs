use crate::geometry::{Rectangle, Vector};

pub const MAX_IN_NODE: usize = 1;

pub struct QuadTree {
    node: Node,
    aabb: Rectangle,
}

impl QuadTree {
    pub fn new(aabb: Rectangle) -> Self {
        Self {
            node: Node::root(),
            aabb,
        }
    }

    pub fn insert(&mut self, object_id: usize, aabb: Rectangle) {
        self.node.insert((object_id, aabb), self.aabb);
    }

    pub fn might_collide<'a>(
        &'a self,
        object_id: usize,
        aabb: Rectangle,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        self.node.might_collide((object_id, aabb), self.aabb)
    }

    pub fn export(&self, vec: &mut Vec<f64>) {
        self.node.export(vec);
    }
}

struct Node {
    objects: Vec<(usize, Rectangle)>,
    children: Option<Box<[Node; 4]>>,
}

impl Node {
    fn root() -> Self {
        Self {
            objects: Vec::new(),
            children: None,
        }
    }

    fn insert(&mut self, object: (usize, Rectangle), node_aabb: Rectangle) {
        match &self.children {
            None => {
                self.objects.push(object);
                if self.objects.len() <= MAX_IN_NODE {
                    return;
                }

                let children = [Node::root(), Node::root(), Node::root(), Node::root()];
                self.children = Some(Box::new(children));

                let sub_aabbs = divide_aabb(node_aabb);
                let objects = std::mem::take(&mut self.objects);

                for obj in objects.into_iter() {
                    self.insert_to_children(obj, &sub_aabbs);
                }
            }
            Some(_) => {
                let sub_aabbs = divide_aabb(node_aabb);
                self.insert_to_children(object, &sub_aabbs);
            }
        }
    }

    fn insert_to_children(&mut self, object: (usize, Rectangle), sub_aabbs: &[Rectangle; 4]) {
        let object_aabb = object.1;
        let mut sub_node = usize::MAX;
        #[allow(clippy::needless_range_loop)]
        for i in 0..4 {
            if object_aabb.collides_with(&sub_aabbs[i]) {
                if sub_node == usize::MAX {
                    sub_node = i;
                } else {
                    self.objects.push(object);
                    return;
                }
            }
        }

        if sub_node == usize::MAX {
            self.objects.push(object);
            return;
        }

        if let Some(children) = &mut self.children {
            children[sub_node].insert(object, sub_aabbs[sub_node]);
        }
    }

    pub fn might_collide<'a>(
        &'a self,
        object: (usize, Rectangle),
        node_aabb: Rectangle,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        match &self.children {
            None => self.children_iter(object),
            Some(children) => {
                let sub_aabbs = divide_aabb(node_aabb);

                let iterator = self.children_iter(object).chain(
                    (0..4_usize)
                        .filter(move |i| sub_aabbs[*i].collides_with(&object.1))
                        .flat_map(move |i| children[i].might_collide(object, sub_aabbs[i])),
                );
                Box::new(iterator)
            }
        }
    }

    fn children_iter<'a>(
        &'a self,
        object: (usize, Rectangle),
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(
            self.objects
                .iter()
                .cloned()
                .filter(move |it| it.0 != object.0)
                .map(|it| it.0),
        )
    }

    fn export(&self, vec: &mut Vec<f64>) {
        match &self.children {
            None => vec.push(0.0),
            Some(children) => {
                vec.push(1.0);
                for ch in children.iter() {
                    ch.export(vec);
                }
            }
        }
    }
}

fn divide_aabb(aabb: Rectangle) -> [Rectangle; 4] {
    let half = aabb.size * 0.5;
    [
        Rectangle::new_vec(aabb.coord, half),
        Rectangle::new_vec(aabb.coord + Vector::new(half.x, 0.0), half),
        Rectangle::new_vec(aabb.coord + Vector::new(0.0, half.y), half),
        Rectangle::new_vec(aabb.coord + half, half),
    ]
}
