use Cursive;
use std::rc::Rc;
use event::Callback;

#[derive(Default)]
pub struct MenuTree {
    pub children: Vec<MenuItem>,
}

pub enum MenuItem {
    Leaf(String, Rc<Callback>),
    Subtree(String, Box<MenuTree>),
    Delimiter,
}

impl MenuTree {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.children.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn add_delimiter(&mut self) {
        self.children.push(MenuItem::Delimiter);
    }

    pub fn delimiter(mut self) -> Self {
        self.add_delimiter();
        self
    }

    pub fn leaf<F: 'static + Fn(&mut Cursive)>(&mut self, title: &str, cb: F)
                                               -> &mut Self {
        self.children.push(MenuItem::Leaf(title.to_string(), Rc::new(Box::new(cb))));
        self
    }

    pub fn add_subtree(&mut self, title: &str, tree: MenuTree) -> &mut Self {
        self.children
            .push(MenuItem::Subtree(title.to_string(), Box::new(tree)));
        self
    }

    pub fn new_subtree(&mut self, title: &str) -> &mut Box<MenuTree> {
        self.add_subtree(title, MenuTree::new());
        match self.children.last_mut().unwrap() {
            &mut MenuItem::Subtree(_, ref mut tree) => tree,
            _ => panic!("??"),
        }
    }

    pub fn subtree(mut self, title: &str, tree: MenuTree) -> Self {
        self.add_subtree(title, tree);
        self
    }
}
