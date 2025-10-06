use std::cell::{Ref, RefCell};
use std::cmp::min;
use std::rc::{Rc, Weak};
use crate::chess::table::Board;

#[derive(Clone, Debug)]
pub struct Node<T>
{
    children: RefCell<Vec<Rc<Node<T>>>>,
    index: RefCell<usize>,
    parent: RefCell<Option<Weak<Node<T>>>>,
    value: RefCell<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<Self>
    {
        Rc::new(
            Node {
                children: RefCell::new(Vec::new()),
                index: RefCell::new(usize::default()),
                parent: RefCell::new(None),
                value: RefCell::new(value)
            }
        )
    }

    pub fn abandon(&self, child: &Rc<Self>)
    {
        let index = *child.index.borrow();

        *child.parent.borrow_mut() = None;
        self.children.borrow_mut().swap_remove(index);

        let count = self.children.borrow().len();

        if count != 0 {
            let index = min(index, count - 1);

            *self
                .children
                .borrow_mut()[index]
                .index
                .borrow_mut() = index;
        }
    }

    pub fn adopt(self: &Rc<Self>, child: &Rc<Self>) { child.attach(self); }

    pub fn attach(self: &Rc<Self>, parent: Node<Board>)
    {
        self.detach();

        *self.index.borrow_mut() = parent.children.borrow().len();
        *self.parent.borrow_mut() = Some(Rc::downgrade(&parent));
        parent.children.borrow_mut().push(self.clone());
    }

    pub fn detach(self: &Rc<Self>)
    {
        self.parent().map(|parent| parent.abandon(self));
    }

    pub fn above(&self, n: usize) -> Result<Rc<Self>, (usize, Option<Rc<Self>>)>
    {
        match self.parent() {
            Some(mut parent) => {
                let mut i = n - 1;
                let mut grandparent = parent.parent();

                while grandparent.is_some() {
                    if i == 0 {
                        break;
                    } else {
                        parent = grandparent.clone().unwrap();
                        grandparent = grandparent.unwrap().parent();
                        i -= 1;
                    }
                }

                if i == 0 {
                    Ok(parent)
                } else {
                    Err((n - i, Some(parent)))
                }
            }

            None => Err((0, None))
        }
    }

    pub fn children(&self) -> Ref<Vec<Rc<Node<T>>>> { self.children.borrow() }

    pub fn is_leaf(&self) -> bool { self.children.borrow().is_empty() }
    pub fn is_root(&self) -> bool { self.parent.borrow().is_none() }

    pub fn grandparent(&self) -> Option<Rc<Self>>
    {
        self.parent().and_then(|parent| parent.parent())
    }

    pub fn parent(&self) -> Option<Rc<Self>>
    {
        self.parent.borrow().as_ref().and_then(|parent| parent.upgrade())
    }

    pub fn upgrade(self: &Rc<Self>)
    {
        self
            .grandparent()
            .map(|grandparent| self.attach(&grandparent.clone()));
    }

    pub fn value(&self) -> Ref<T> { self.value.borrow() }
    pub fn set_value(&self, value: T) { *self.value.borrow_mut() = value; }
}