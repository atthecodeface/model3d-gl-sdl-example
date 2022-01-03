/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    hierarchy.rs
@brief   Hiearchy using usize indices to arrays of elements
 */

//a Documentation
/*!
This module provides a hierarchy of nodes and iterators over them
 */

//a Imports
use indent_display::{IndentedDisplay, IndentedOptions, Indenter};

//a Constants
/// Compile-time setting for adding extra debugging information
const DEBUG_ITERATOR: bool = false;

//a Node
//tp Node
/// A node in the hierarchy
pub struct Node<T> {
    /// An optional parent index - if None, this is a root
    parent: Option<usize>,
    /// Array of child indices
    children: Vec<usize>,
    /// Data associated with the node
    pub data: T,
}

//ip Node
impl<T> Node<T> {
    //fp new
    /// Create a new node in the hierarchy
    pub fn new(data: T, parent: Option<usize>) -> Self {
        let children = Vec::new();
        Self {
            parent,
            children,
            data,
        }
    }

    //fp has_parent
    /// Returns true if the node has a parent - i.e. it is not the
    /// root of the hierarchy
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    //fp set_parent
    /// Set the parent of a node
    pub fn set_parent(&mut self, parent: Option<usize>) {
        self.parent = parent;
    }

    //mp add_child
    /// Add a child of this node
    pub fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }

    //mp has_children
    /// Return true if the node has children
    pub fn has_children(&self) -> bool {
        self.children != []
    }

    //zz All done
}

//a Hierarchy
//tp Hierarchy
/// A hierarchy of nodes, each of which has a data of the type of the
/// tree
pub struct Hierarchy<T> {
    /// The elements in the hierarchy
    elements: Vec<Node<T>>,
    /// The roots in the hierarchy - more than one tree can be stored
    /// in the hierarchy
    roots: Vec<usize>,
}

//ip Hierarchy
impl<T> Hierarchy<T> {
    //fp new
    /// Create a new hierarchy
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            roots: Vec::new(),
        }
    }

    //mp len
    /// Return the number of elements in the hierarchy
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    //mp add_node
    /// Add a node to the hierarchy
    pub fn add_node(&mut self, data: T) -> usize {
        let n = self.elements.len();
        self.elements.push(Node::new(data, None));
        n
    }

    //mp relate
    /// Add a relation from a parent to a child in the hierarchy
    pub fn relate(&mut self, parent: usize, child: usize) {
        self.elements[parent].add_child(child);
        self.elements[child].set_parent(Some(parent));
    }

    //mp find_roots
    /// Find all the roots of the hierarchy and record it
    pub fn find_roots(&mut self) {
        self.roots = Vec::new();
        for (i, e) in self.elements.iter().enumerate() {
            if !e.has_parent() {
                self.roots.push(i);
            }
        }
    }

    //mp borrow_node
    /// Borrow a node in the hierarchy
    pub fn borrow_node(&self, index: usize) -> &T {
        &self.elements[index].data
    }

    //mp borrow_mut
    /// Mutuably borrow a node in the hierarchy
    pub fn borrow_mut(&mut self) -> (&Vec<usize>, &mut Vec<Node<T>>) {
        (&self.roots, &mut self.elements)
    }

    //mp borrow_roots
    /// Borrow the roots of the hierarchy
    pub fn borrow_roots(&self) -> &Vec<usize> {
        &self.roots
    }

    //mp enum_from
    /// Enumerate the nodes from a particular node
    pub fn enum_from<'z>(&'z self, node: usize) -> NodeEnum<T> {
        NodeEnum::new(&self.elements, node)
    }

    //mp iter_from
    /// Iterate the nodes from a particular node
    pub fn iter_from<'z>(&'z self, node: usize) -> NodeIter<T> {
        NodeIter::new(&self.elements, node)
    }

    //mp borrow_elements
    /// Borrow all the elements
    pub fn borrow_elements<'z>(&'z self) -> &Vec<Node<T>> {
        &self.elements
    }

    //zz All done
}

//ip IndentedDisplay for Hierarchy
impl<'a, Opt: IndentedOptions<'a>, T: IndentedDisplay<'a, Opt>> IndentedDisplay<'a, Opt>
    for Hierarchy<T>
{
    //mp fmt
    /// Display for humans with indent
    fn indent(&self, f: &mut Indenter<'a, Opt>) -> std::fmt::Result {
        use std::fmt::Write;
        let mut sub = f.sub();
        for (i, e) in self.elements.iter().enumerate() {
            if !e.has_parent() {
                for x in self.enum_from(i) {
                    match x {
                        NodeEnumOp::Push(x, _) => {
                            self.elements[x].data.indent(&mut sub)?;
                            write!(sub, "\n")?;
                            sub = sub.sub();
                        }
                        NodeEnumOp::Pop(_, _) => {
                            sub = sub.pop();
                        }
                        _ => {}
                    };
                }
                // expliticly drop sub for cleanliness
            }
        }
        drop(sub);
        Ok(())
    }
}

//a NodeEnumOp
//tp NodeEnumOp
/// This enumeration is used as a node hierarchy is enumerated: a node
/// is pushed into, then children are pushed/popped, then the node is
/// popped.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeEnumOp<T> {
    /// Pushing in to the hierachy to new node index, and true if node has children
    Push(T, bool),
    /// Popping out to the hierachy to node index
    Pop(T, bool),
}

//ip NodeEnumOp
impl<T> NodeEnumOp<T> {
    //mp is_pop
    /// Return true if this is a Pop, false if it is a Push
    #[inline]
    pub fn is_pop(&self) -> bool {
        match self {
            Self::Pop(_, _) => true,
            _ => false,
        }
    }

    //zz All done
}

//a Recipe
//tp Recipe
/// Create a recipe from traversing a hierarchy from a node
///
/// The recipe is a [Vec] of [NodeEnumOp]s which describe entirely how
/// to traverse the hierarchy; essentially it is a record of an
/// enumeration of a hierarchy or part of a hierarchy
pub struct Recipe {
    /// The [NodeEnumOp]s that make up the traversal
    ops: Vec<NodeEnumOp<usize>>,
    /// The maximum depth required (maximum 'tree' depth from the initial node)
    max_depth: usize,
    /// The current depth (used in generating the recipe)
    depth: usize,
}

//ip Default for Recipe
impl Default for Recipe {
    fn default() -> Self {
        Self::new()
    }
}

//ip Recipe
impl Recipe {
    //fp new
    /// Create a new recipe
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            max_depth: 0,
            depth: 0,
        }
    }

    //mp add_op
    /// Add a new operation to the recipe
    pub fn add_op(&mut self, op: NodeEnumOp<usize>) {
        if op.is_pop() {
            self.depth -= 1;
        } else {
            self.depth += 1;
            if self.depth > self.max_depth {
                self.max_depth = self.depth;
            }
        }
        self.ops.push(op);
    }

    //dp take
    /// Deconstruct the recipe
    pub fn take(self) -> (usize, Vec<NodeEnumOp<usize>>) {
        (self.max_depth, self.ops)
    }

    //mp depth
    /// Find the maximum depth of the recipe
    pub fn depth(&self) -> usize {
        self.max_depth
    }

    //mp borrow_ops
    /// Borrow the operations that make the recipe
    pub fn borrow_ops<'z>(&'z self) -> &'z Vec<NodeEnumOp<usize>> {
        &self.ops
    }

    //mp of_ops
    /// Create a recipe from a [NodeNum] iterator
    pub fn of_ops<T>(iter: NodeEnum<T>) -> Self {
        let mut r = Self::new();
        for op in iter {
            r.add_op(op);
        }
        r
    }

    //zz Al done
}

//a NodeEnum
//ti NodeEnumState
/// This enumeration provides
#[derive(Debug, Clone, Copy)]
enum NodeEnumState {
    /// PreNode indicates that the element has not been returned yet
    PreNode(usize),
    PreChildren(usize),
    Child(usize, usize),
    PostChildren(usize),
}

//tp NodeEnum
/// An iterator structure to permit iteration over a hierarchy of nodes
///
/// The iterator yields pairs of (NodeEnumState, usize)
/// For a hierarchy of nodes:
///   A -> B -> C0
///             C1
///        D
///        E  -> F
/// the iterator will provide
///
///    Push(A,true)
///    Push(B,true)
///    Push(C0,false)
///    Pop(C0)
///    Push(C1,false)
///    Pop(C1)
///    Pop(B)
///    Push(D,false)
///    Pop(D)
///    Push(E,true)
///    Push(F,false)
///    Pop(F)
///    Pop(E)
///    Pop(A)
pub struct NodeEnum<'a, T> {
    /// Hierarchy of nodes that is being iterated over
    pub hierarchy: &'a [Node<T>],
    /// Stack of indices in to the hierarchy and whether the node at that point has been handled
    stack: Vec<NodeEnumState>,
}

//ip NodeEnum
impl<'a, T> NodeEnum<'a, T> {
    //fp new
    /// Create a new hierarchy node iterator
    pub fn new(hierarchy: &'a [Node<T>], root: usize) -> Self {
        let mut stack = Vec::new();
        stack.push(NodeEnumState::PreNode(root));
        Self { hierarchy, stack }
    }
}

//ip Iterator for NodeEnum
impl<'a, T> Iterator for NodeEnum<'a, T> {
    type Item = NodeEnumOp<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            None
        } else {
            let se = self.stack.pop().unwrap();
            // Track the state for debugging
            if DEBUG_ITERATOR {
                println!("{:?}", se);
            }
            match se {
                NodeEnumState::PreNode(x) => {
                    self.stack.push(NodeEnumState::PreChildren(x));
                    let has_children = self.hierarchy[x].has_children();
                    Some(NodeEnumOp::Push(x, has_children))
                }
                NodeEnumState::PreChildren(x) => {
                    // Push(x) has happened
                    self.stack.push(NodeEnumState::Child(x, 0));
                    self.next()
                }
                NodeEnumState::Child(x, n) => {
                    // Children of x prior to n have happened
                    if let Some(c) = self.hierarchy[x].children.get(n) {
                        self.stack.push(NodeEnumState::Child(x, n + 1));
                        self.stack.push(NodeEnumState::PreNode(*c));
                    } else {
                        // run out of children
                        self.stack.push(NodeEnumState::PostChildren(x));
                    }
                    self.next()
                }
                NodeEnumState::PostChildren(x) => {
                    // Push(x) and all children ops have happened
                    let has_children = self.hierarchy[x].has_children();
                    Some(NodeEnumOp::Pop(x, has_children))
                }
            }
        }
    }
}

//ip NodeIter
/// An iterator over part of a [Hierarchy] that returns a reference to
/// the node as it traverses
pub struct NodeIter<'a, T> {
    node_enum: NodeEnum<'a, T>,
}

//ip NodeIter
impl<'a, T> NodeIter<'a, T> {
    //fp new
    /// Create a new hierarchy node iterator
    pub fn new(hierarchy: &'a [Node<T>], root: usize) -> Self {
        Self {
            node_enum: NodeEnum::new(hierarchy, root),
        }
    }
}

//ip Iterator for NodeIter
impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = NodeEnumOp<(usize, &'a T)>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.node_enum.next() {
            Some(NodeEnumOp::Push(x, c)) => {
                Some(NodeEnumOp::Push((x, &self.node_enum.hierarchy[x].data), c))
            }
            Some(NodeEnumOp::Pop(x, c)) => {
                Some(NodeEnumOp::Pop((x, &self.node_enum.hierarchy[x].data), c))
            }
            None => None,
        }
    }
}

//a Test
#[cfg(test)]
mod test_node {
    use super::*;
    use indent_display::NullOptions;

    //fi basic_hierarchy
    pub fn basic_hierarchy() -> Hierarchy<&'static str> {
        let mut h = Hierarchy::new();
        let a = h.add_node("A");
        let b = h.add_node("B");
        let c0 = h.add_node("C0");
        let c1 = h.add_node("C1");
        let d = h.add_node("D");
        let e = h.add_node("E");
        let f = h.add_node("F");
        h.relate(a, b);
        h.relate(a, d);
        h.relate(a, e);
        h.relate(b, c0);
        h.relate(b, c1);
        h.relate(e, f);
        h.find_roots();
        h
    }

    //fi test_0
    #[test]
    fn test_0() {
        let h = basic_hierarchy();
        assert_eq!(h.borrow_roots(), &[0], "Expect roots to just be A");
    }

    //fi test_display
    #[test]
    fn test_display() {
        let h = basic_hierarchy();
        let mut f = Vec::<u8>::new();
        let opt = NullOptions {};
        let mut ind = Indenter::new(&mut f, " ", &opt);
        h.indent(&mut ind).unwrap();
        drop(ind);
        assert_eq!(
            f,
            b" A
  B
   C0
   C1
  D
  E
   F
"
        );
    }

    //fi test_recipe
    #[test]
    fn test_recipe() {
        let h = basic_hierarchy();
        let mut r = Recipe::new();
        for op in h.enum_from(0) {
            r.add_op(op);
        }
        let (max_depth, ops) = r.take();
        assert_eq!(max_depth, 3, "Max depth of tree is 3");
        assert_eq!(
            ops,
            vec![
                NodeEnumOp::Push(0, true),
                NodeEnumOp::Push(1, true),
                NodeEnumOp::Push(2, false),
                NodeEnumOp::Pop(2, false),
                NodeEnumOp::Push(3, false),
                NodeEnumOp::Pop(3, false),
                NodeEnumOp::Pop(1, true),
                NodeEnumOp::Push(4, false),
                NodeEnumOp::Pop(4, false),
                NodeEnumOp::Push(5, true),
                NodeEnumOp::Push(6, false),
                NodeEnumOp::Pop(6, false),
                NodeEnumOp::Pop(5, true),
                NodeEnumOp::Pop(0, true),
            ],
            "Recipe mismatch"
        );
    }
}
