use std::rc::Rc;
use std::cell::RefCell;
use std::iter::repeat;

use std::mem::swap;

pub type RcNode = Rc<RefCell<Node>>;  
// what to use ???
/*
use std::ops::{Deref, DerefMut};

struct RcNode(Rc<RefCell<Node>>);

impl Deref for RcNode {
    type Target = Rc<RefCell<Node>>;
    fn deref(&self) -> &Rc<RefCell<Node>> { &self.0 }
}
*/

trait SuperNode<KeyT>
{
    fn new(value: KeyT) -> Self;
    fn get_left(&self) -> Self;
    fn get_right(&self) -> Self;
    fn set_left(&self, new_node: Self) -> ();
    fn set_right(&self, new_node: Self) -> ();
    fn set_left_right_none(&self) -> ();
    fn get_key(&self) -> KeyT;
    fn get_child(&self) -> Self;
    fn has_child(&self)-> bool;
    fn set_child(&self, new_child: Self) -> ();
    fn has_right(&self)-> bool;
}

struct Node {
    child:  Option<RcNode>,
    left:   Option<RcNode>,
    right:  Option<RcNode>,
    deg:    usize,
    mark:   bool,
    key:    u32
}

impl SuperNode<u32> for RcNode
{
    fn new(value: u32) -> Self
    {
        Rc::new(RefCell::new(Node{
            child:  None,
            left:   None,
            right:  None,
            deg:    0,
            mark:   false,
            key:    value 
        }))
    }

    fn get_left(&self) -> Self
    {
        self.borrow().left.as_ref().unwrap_or(&self.clone()).clone()
    }

    fn get_right(&self) -> Self
    {
        self.borrow().right.as_ref().unwrap_or(&self.clone()).clone()
    }

    fn set_left(&self, new_node: Self)
    {
        self.borrow_mut().left = Some(new_node);
    }


    fn set_right(&self, new_node: Self) 
    {
        self.borrow_mut().right = Some(new_node);        
    }

    fn set_left_right_none(&self) 
    {
        self.borrow_mut().right = None;  
        self.borrow_mut().left = None;      
    }

    fn get_key(&self) -> u32
    {
        self.borrow().key
    }
    // panic when child is None
    fn get_child(&self) -> Self
    {
        self.borrow().child.as_ref().unwrap().clone()
    }

    fn set_child(&self, new_child: Self) -> ()
    {
        self.borrow_mut().child = Some(new_child);
    }
    
    fn has_child(&self)-> bool
    {
        self.borrow().child.is_some()
    }

    fn has_right(&self)-> bool
    {
        self.borrow().right.is_some()
    }
}

fn put_right(a: &RcNode, b: &RcNode) -> ()
{
    let a_right = a.get_right();
    b.set_left(a.clone());
    b.set_right(a_right.clone());
    a_right.set_left(b.clone());
    a.set_right(b.clone());
}

fn kill_lr_links(node: &RcNode)
{
    let left = node.get_left();
    let right = node.get_right();
    left.set_right(right.clone());
    right.set_left(left.clone());
    node.set_left_right_none();

   /* left.borrow_mut().right = Some(right.clone());
    right.borrow_mut().left = Some(left.clone());
    node.borrow_mut().left = None;
    node.borrow_mut().right = None;
    */
}

pub struct FibHeap {
    pub min: Option<RcNode>,
    pub n_roots: usize,
    pub n_all: usize,
}


impl FibHeap {
    pub fn new() -> Self
    {
        FibHeap
        {
            min: None,
            n_roots: 0,
            n_all: 0
        }
    }

    pub fn insert(&mut self, value: u32) -> ()
    {
        let new_node: RcNode = SuperNode::new(value);
        if self.min.is_some()
        {
            let min_node = self.get_min_node();
            put_right(&min_node, &new_node);
            if min_node.get_key() > new_node.get_key()
            {
                self.min = Some(new_node);
            }
        }
        else
        {

            self.min = Some(new_node);
        }
        self.n_roots += 1;
        self.n_all += 1;
    }

    pub fn get_size(&self) -> usize
    {
        self.n_all
    }

    fn get_min_node(&self) -> RcNode
    {
        self.min.as_ref().unwrap().clone()
    }

    pub fn extract_min(&mut self) -> Option<u32>
    {
        if self.min.is_none()
        {
            return None;
        }

        let z = self.get_min_node();
        let res:u32 = z.get_key();

        if self.n_all == 1
        {
            assert!(!z.has_child() && !z.has_right());
            self.min = None;
            self.n_roots = 0;
            self.n_all = 0;

        }
        else
        {
            if z.has_child()
            {
                let mut t = z.get_child();
                let n_child = z.borrow().deg;
                for _ in 0..n_child {
                    let nxt = t.get_right();
                    kill_lr_links(&t);
                    put_right(&z,&t);
                    self.n_roots += 1;
                    t = nxt;
                }
            }    
            self.min = Some(z.get_right());
            kill_lr_links(&z);
    
            self.n_roots -= 1;
            self.consolidate();
            self.n_all -= 1;
        }
        return Some(res);

    } //extract_min

    fn consolidate(&mut self) -> ()
    {
        #![allow(non_snake_case)]
        let Dn = ((self.n_all as f32).log(1.6).floor() as usize) ; // max degree assessment


        let mut A:Vec<Option<RcNode>> = repeat(None).take(Dn).collect();

        let mut x = self.min.as_ref().unwrap().clone();
        for _ in 0..self.n_roots // for all nodes in root list
        {
            let next = x.get_right();
            let mut d = x.borrow().deg;

            while A[d].is_some()
            { 
                let mut y = A[d].as_ref().unwrap().clone();

                if x.get_key() > y.get_key()
                {
                    swap(&mut x, &mut y);
                }
                self.fib_heap_link(&y, &x);

                A[d] = None;
                d += 1;
            }
            A[d] = Some(x);
            x = next;
        }

        self.n_roots = 0;
        for i in 0..Dn
        {
            if A[i].is_some()
            {
                let new_root_node = A[i].as_ref().unwrap().clone();
                kill_lr_links(&new_root_node);
                if self.n_roots == 0
                {
                    self.min = Some(new_root_node);
                }
                else
                {   
                    put_right(&self.min.as_ref().unwrap(), &new_root_node);
                    if self.min.as_ref().unwrap().get_key() > new_root_node.get_key()
                    {
                        self.min = Some(new_root_node);
                    }
                }
                self.n_roots += 1;
            }
        }
    } // consolidate

    fn fib_heap_link(&mut self, y: &RcNode, x: &RcNode)
    {
        kill_lr_links(&y);
       // y.borrow_mut().child = None;
        if x.has_child()
        {
            assert!(x.borrow().deg != 0);
            let x_child = x.get_child();
            put_right(&x_child, &y);
        }
        else
        {
            assert!(x.borrow().deg == 0);
            x.set_child(y.clone());
        }
        x.borrow_mut().deg += 1;
        y.borrow_mut().mark = false;
        self.n_roots -= 1;
    }

} // impl FibHeap


