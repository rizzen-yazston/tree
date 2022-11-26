// This file is part of `tree-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `tree-rizzen-yazston` crate.

//! Welcome to the Tree project.
//! 
//! A simple generic tree structure containing data nodes. The `tree` crate provides the `Tree` struct, that can store
//! data that implements the `Any` trait. Methods are provided for the manipulation of the tree structure, and obtaining
//! information regarding the tree structure and its nodes. Manipulating the data of the nodes is done by one method
//! `data_mut`, which simply provides a mutable reference to the vector containing the data. `data` method is just an
//! immutable reference if reading is only required. There is also a field of the node for specifying the type of node
//! stored in the tree, though it is not used internally by the various `Tree` methods. It can be read, and any type can
//! be used if it implements the `Any` trait.
//! 
//! # Examples
//! ```
//! use tree::{NodeFeatures, Tree};
//! 
//! enum Nodes {
//!     Root,
//!     Statement,
//!     Equal,
//!     Divide,
//!     Add,
//!     Leaf,
//! }
//! 
//! let mut tree = Tree::new();
//! let no_data = NodeFeatures { allow_children: true, allow_data: false };
//! let variable = NodeFeatures { allow_children: false, allow_data: true };
//! 
//! // Build tree of one statement: z = (x + y) / 2
//! // Just ignoring the `Result` using .ok() as this is a trivial example.
//! let mut index = tree.insert( 300, no_data.clone(), Box::new( Nodes::Root ) ).unwrap();
//! tree.insert( index, no_data.clone(), Box::new( Nodes::Statement ) ).ok();
//! tree.insert( 1, no_data.clone(), Box::new( Nodes::Equal ) ).ok();
//! index = tree.insert( 2, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "z".to_string() ) );
//! tree.insert( 2, no_data.clone(), Box::new( Nodes::Divide ) ).ok();
//! tree.insert( 4, no_data.clone(), Box::new( Nodes::Add ) ).ok();
//! index = tree.insert( 5, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "x".to_string() ) );
//! index = tree.insert( 5, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "y".to_string() ) );
//! index = tree.insert( 4, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "2".to_string() ) );
//! assert_eq!( tree.count(), 9, "9 nodes are present." );
//! ```

use core::any::Any;

/// Specifies the capabilities of the node being added.
/// `allow_children` as `true` indicates the node can have children. The only leaf nodes are set to `false`.
/// `allow_data` as `true` indicates the node can have data. The leaf nodes are normally set to `true`, and for other
/// non-leaf nodes it is entirely optional to contain data.
/// 
/// # Examples
/// 
/// See the example for `Tree` on usage.
#[derive( Clone, Debug )]
pub struct NodeFeatures {
    pub allow_children: bool,
    pub allow_data: bool,
}

/// Provides a simple tree structure for holding data nodes.
/// 
/// # Examples
/// ```
/// use tree::{NodeFeatures, Tree};
/// 
/// enum Nodes {
///     Root,
///     Statement,
///     Equal,
///     Divide,
///     Add,
///     Leaf,
/// }
/// 
/// let mut tree = Tree::new();
/// let no_data = NodeFeatures { allow_children: true, allow_data: false };
/// let variable = NodeFeatures { allow_children: false, allow_data: true };
/// 
/// // Build tree of one statement: z = (x + y) / 2
/// // Just ignoring the `Result` using .ok() as this is a trivial example.
/// let mut index = tree.insert( 300, no_data.clone(), Box::new( Nodes::Root ) ).unwrap();
/// tree.insert( index, no_data.clone(), Box::new( Nodes::Statement ) ).ok();
/// tree.insert( 1, no_data.clone(), Box::new( Nodes::Equal ) ).ok();
/// index = tree.insert( 2, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
/// tree.data_mut( index ).unwrap().push( Box::new( "z".to_string() ) );
/// tree.insert( 2, no_data.clone(), Box::new( Nodes::Divide ) ).ok();
/// tree.insert( 4, no_data.clone(), Box::new( Nodes::Add ) ).ok();
/// index = tree.insert( 5, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
/// tree.data_mut( index ).unwrap().push( Box::new( "x".to_string() ) );
/// index = tree.insert( 5, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
/// tree.data_mut( index ).unwrap().push( Box::new( "y".to_string() ) );
/// index = tree.insert( 4, variable.clone(), Box::new( Nodes::Leaf ) ).unwrap();
/// tree.data_mut( index ).unwrap().push( Box::new( "2".to_string() ) );
/// assert_eq!( tree.count(), 9, "9 nodes are present." );
/// ```
pub struct Tree {
    nodes: Vec<Option<Box<Node>>>,
    root: Option<usize>
}

// IDEA: Implement move node/branch to another part of the tree

impl Tree {

    // -- Tree structure manipulation --

    /// Create a new empty tree.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 0, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// ```
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            root: None
        }
    }

    /// Create a node and appending it as last child to the `node_index` node.
    /// If there is no root node for the tree, then  the value of `node_index` and `NodeFeatures::allow_parent` boolean 
    /// will both be discarded (ignored).
    /// If no error, the returned `usize` value is the index of the created node in the tree.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 425, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// ```
    pub fn insert( &mut self, node_index: usize, features: NodeFeatures, node_type: Box<dyn Any> )
        -> Result<usize, String>
    {
        let mut children = None;
        let mut parent = None;
        let mut data = None;

        // `node_index` is ignored when first node is inserted into tree.
        if !self.root.is_none() {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( "Failed to retrieve index node.".to_string() )
            };
            if !index_node.features.allow_children {
                return Err( "Index node does not allow children.".to_string() );
            }
            parent = Some( node_index );
        }
        if features.allow_children {
            children = Some( Box::new( Vec::<usize>::new() ) );
        }
        if features.allow_data {
            data = Some( Vec::<Box<dyn Any>>::new() );
        }
        let node = Node {
            node_type,
            features,
            parent,
            children,
            data,
        };
        let index = self.nodes.len();
        self.nodes.push( Some( Box::new( node ) ) );
        if self.root.is_none() {
            self.root = Some( index );
        } else {
            let Some( index_node ) = self.node_mut( node_index ) else {
                return Err( "Failed to retrieve index node.".to_string() )
            };
            index_node.children.as_mut().unwrap().push( index );
        }
        Ok( index )
    }

    /// Create a node and insert as a child to the `node_index` node at the `position` specified. The `position` must be
    /// in the range of 0 to number of children.
    /// If there is no root node for the tree, then the values of `node_index`, `position` and
    /// `NodeFeatures::allow_parent` boolean will be discarded (ignored).
    /// If no error, the returned `usize` value is the index of the created node in the tree.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 4, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
    /// tree.insert( 0, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 1 ) ).ok();
    /// tree.insert_at( 0, 0, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 2 ) ).ok();
    /// assert_eq!( tree.count(), 3, "3 nodes is present." );
    /// ```
    pub fn insert_at( &mut self, node_index: usize, position: usize, features: NodeFeatures, node_type: Box<dyn Any>, )
        -> Result<usize, String>
    {
        let mut children = None;
        let mut parent = None;
        let mut data = None;

        // `node_index` is ignored when first node is inserted into tree.
        if !self.root.is_none() {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( "Failed to retrieve index node.".to_string() )
            };
            if !index_node.features.allow_children {
                return Err( "Index node does not allow children.".to_string() );
            }
            if position > index_node.children.iter().count() {
                return Err(
                    "Invalid position provided, that is greater than number of current children.".to_string()
                );
            }
            parent = Some( node_index );
        }
        if features.allow_children {
            children = Some( Box::new( Vec::<usize>::new() ) );
        }
        if features.allow_data {
            data = Some( Vec::<Box<dyn Any>>::new() );
        }
        let node = Node {
            node_type,
            features,
            parent,
            children,
            data,
        };
        let index = self.nodes.len();
        self.nodes.push( Some( Box::new( node ) ) );
        if self.root.is_none() {
            self.root = Some( index );
        } else {
            let Some( index_node ) = self.node_mut( node_index ) else {
                return Err( "Failed to retrieve index node.".to_string() )
            };
            index_node.children.as_mut().unwrap().insert( position, index );
        }
        Ok( index )
    }

    /// Deletes a node from the tree, discarding any data in the node.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 68, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// match tree.delete( 0 ) {
    ///     Err( error ) => println!( "{}", error ),
    ///     Ok( _ ) => println!( "Succeeded to delete node." )
    /// }
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// ```
    pub fn delete( &mut self, node_index: usize ) -> Result<(), String> {
        let mut _parent = None;
        {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( "Failed to retrieve index node.".to_string() )
            };
            if index_node.features.allow_children && !index_node.children.as_ref().unwrap().is_empty() {
                return Err( "Can't delete index node as it still has children.".to_string() );
            }
            _parent = index_node.parent;
        }
        {
            if !_parent.is_none() {
                let parent = _parent.unwrap();
                let Some( parent_node ) = self.node_mut( parent ) else {
                    return Err( "Failed to retrieve parent node.".to_string() )
                };
                let children = parent_node.children.as_mut().unwrap();
                let Some( _position ) = children.iter().position( |&x| x == parent ) else {
                    // Serious integrity issue.
                    return Err( "Index node as missing in the parent node's children.".to_string() );
                };
                children.remove( _position );
            }
        }
        let mut _node_ref = self.nodes.get_mut( node_index ).unwrap();
        *_node_ref = None;
        if Some( node_index ) == self.root {
            self.root = None;
            self.nodes.clear();
        }
        Ok( () )
    }

    /// Deletes a node from the tree, and return its data (if any).
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 128, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap(); // Deleting root node, and take data.
    /// let data = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// assert_eq!( *data, "String data".to_string(), "Data of node is a string" );
    /// ```
    pub fn take( &mut self, node_index: usize ) -> Result<Option<Vec<Box<dyn Any>>>, String> {
        let mut _parent = None;
        {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( "Failed to retrieve index node.".to_string() )
            };
            if index_node.features.allow_children && !index_node.children.as_ref().unwrap().is_empty() {
                return Err( "Can't delete index node as it still has children.".to_string() );
            }
            _parent = index_node.parent;
        }
        {
            if !_parent.is_none() {
                let parent = _parent.unwrap();
                let Some( parent_node ) = self.node_mut( parent ) else {
                    return Err( "Failed to retrieve parent node.".to_string() )
                };
                let children = parent_node.children.as_mut().unwrap();
                let Some( _position ) = children.iter().position( |&x| x == parent ) else {
                    // Serious integrity issue.
                    return Err( "Index node as missing in the parent node's children.".to_string() );
                };
                children.remove( _position );
            }
        }
        let mut _node_ref = self.nodes.get_mut( node_index ).unwrap();
        let node = _node_ref.take().unwrap();
        *_node_ref = None;
        if Some( node_index ) == self.root {
            self.root = None;
            self.nodes.clear();
        }
        Ok( node.data )
    }

    /// Clear the tree of all nodes.
    /// 
    /// Warning: All data in the nodes will be destroyed.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 254, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// tree.clear();
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// ```
    pub fn clear( &mut self ) {
        self.root = None;
        self.nodes.clear();
    }

    // IDEA: Implement move node/branch to another part of the tree


    // -- information methods --

    /// Check is node exists.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 53, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// assert!( tree.exists( 0 ) );
    /// assert!( !tree.exists( 1 ) );
    /// ```
    pub fn exists( &self, node_index: usize ) -> bool {
        if let Some( option ) = self.nodes.get( node_index ) {
            if let Some( _ ) = option {
                return true;
            }
        }
        false
    }

    /// Obtain reference to the node type for the specified node.
    /// If node does not exists, `Err( String )` is returned.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 514, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 5usize ) ).ok();
    /// let type_any_ref = tree.node_type( 0 ).ok().unwrap().as_ref();
    /// let type_usize = type_any_ref.downcast_ref::<usize>().unwrap();
    /// assert_eq!( *type_usize, 5 );
    /// ```
    pub fn node_type( &self, node_index: usize ) -> Result<&Box<dyn Any>, String> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( "Failed to retrieve node.".to_string() )
        };
        Ok( &index_node.node_type )
    }

    /// Obtain reference to the node features for the specified node.
    /// If node does not exists, `Err( String )` is returned.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 16, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// let features_ref = tree.features( 0 );
    /// let features = features_ref.as_ref().unwrap();
    /// assert!( features.allow_children );
    /// assert!( features.allow_data );
    /// ```
    pub fn features( &self, node_index: usize ) -> Result<&NodeFeatures, String> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( "Failed to retrieve node.".to_string() )
        };
        Ok( &index_node.features )
    }

    /// Obtain reference to the node parent for the specified node.
    /// If node does not exists, `Err( String )` is returned.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 23, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
    /// assert_eq!( tree.parent( 1 ).ok(), Some( 0 ), "Parent is root node." );
    /// ```
    pub fn parent( &self, node_index: usize ) -> Result<usize, String> {
        if Some( node_index ) == self.root {
            return Err( "Root node has no parent.".to_string() );
        }
        let Some( index_node ) = self.node( node_index ) else {
            return Err( "Failed to retrieve node.".to_string() )
        };
        Ok( *index_node.parent.as_ref().unwrap() )
    }

    /// Obtain reference to the node children for the specified node.
    /// If node does not exists, `Err( String )` is returned.
    /// Also `Err( String )` is returned is the node does not allow children.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 624, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
    /// let children = tree.children( 0 ).ok().unwrap();
    /// assert_eq!( children.len(), 1, "Has 1 child." );
    /// ```
    pub fn children( &self, node_index: usize ) -> Result<&Vec<usize>, String> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( "Failed to retrieve node.".to_string() )
        };
        if !index_node.features.allow_children {
            return Err( "Node does not allow children.".to_string() );
        }
        Ok( &index_node.children.as_ref().unwrap() )
    }

    /// Obtain the depth of the specified node from the root.
    /// If node does not exists, `Err( String )` is returned.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 72, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
    /// let depth = tree.depth( 1 ).ok().unwrap();
    /// assert_eq!( depth, 1, "Has 1 child." );
    /// ```
    pub fn depth( &self, mut node_index: usize ) -> Result<usize, String> {
        let mut depth = 0;
        loop {
            if let Some( node ) = self.node( node_index ) {
                if let Some( parent ) = node.parent.as_ref() {
                    node_index = *parent;
                    depth += 1;
                }
                else {
                    return Ok( depth );
                }
            }
            else {
                return Err( "Failed to retrieve node.".to_string() );
            }
        }
    }

    /// Count the nodes of the tree.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 297, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
    /// tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
    /// assert_eq!( tree.count(), 2, "Has 2 nodes." );
    /// ```
    pub fn count( &self ) -> usize {
        self.nodes.iter().filter( |n| !n.is_none() ).count()
    }


    // -- Data methods --

    /// Obtain mutable reference to the node data for the specified node.
    /// If node does not exists, `Err( String )` is returned.
    /// Also `Err( String )` is returned is the node does not allow data.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 974, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0usize ) ).ok();
    /// tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    /// let data_vec_mut = tree.data_mut( 0 ).ok().unwrap();
    /// let data = data_vec_mut.get_mut( 0 ).unwrap().downcast_mut::<String>().unwrap();
    /// 
    /// // mutate the data
    /// *data = "Mutated data".to_string();
    /// 
    /// // Take node to check if data did mutate.
    /// let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap(); // Deleting root node, and take data.
    /// let data_taken = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// assert_eq!( *data_taken, "Mutated data".to_string(), "Data of node is a mutated string" );
    /// ```
    pub fn data_mut( &mut self, node_index: usize ) -> Result<&mut Vec<Box<dyn Any>>, String> {
        let Some( index_node ) = self.node_mut( node_index ) else {
            return Err( "Failed to retrieve node.".to_string() )
        };
        if !index_node.features.allow_data {
            return Err( "Node does not allow data.".to_string() );
        }
        Ok( index_node.data.as_mut().unwrap() )
    }

    /// Obtain reference to the node data for the specified node.
    /// If node does not exists, `Err( String )` is returned.
    /// Also `Err( String )` is returned is the node does not allow data.
    /// 
    /// # Examples
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 550, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0usize ) ).ok();
    /// tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    /// let data_vec_ref = tree.data_ref( 0 ).ok().unwrap();
    /// let data = data_vec_ref.get( 0 ).unwrap().downcast_ref::<String>().unwrap();
    /// assert_eq!( *data, "String data".to_string() );
    /// ```
    pub fn data_ref( &self, node_index: usize ) -> Result<&Vec<Box<dyn Any>>, String> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( "Failed to retrieve node.".to_string() )
        };
        if !index_node.features.allow_data {
            return Err( "Node does not allow data.".to_string() );
        }
        Ok( &index_node.data.as_ref().unwrap() )
    }


    // -- Internal methods --

    fn node( &self, node_index: usize ) -> Option<&Node> {
        if let Some( option ) = self.nodes.get( node_index ) {
            if let Some( node ) = option {
                return Some( node.as_ref() );
            }
        }
        None
    }

    fn node_mut( &mut self, node_index: usize ) -> Option<&mut Node> {
        if let Some( option ) = self.nodes.get_mut( node_index ) {
            if let Some( node ) = option {
                return Some( node.as_mut() );
            }
        }
        None
    }
}

// Internal generic node used within the tree.
struct Node {
    node_type: Box<dyn Any>,
    features: NodeFeatures,
    parent: Option<usize>,
    children: Option<Box<Vec<usize>>>,
    data: Option<Vec<Box<dyn Any>>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let tree = Tree::new();
        assert_eq!( tree.count(), 0, "Has 0 nodes." );
    }

    #[test]
    fn insert() {
        let mut tree = Tree::new();
        tree.insert( 425, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
        assert_eq!( tree.count(), 1, "1 node is present." );
    }

    #[test]
    fn insert_at() {
        let mut tree = Tree::new();
        tree.insert( 4, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
        tree.insert( 0, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 1 ) ).ok();
        tree.insert_at( 0, 0, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 2 ) ).ok();
        assert_eq!( tree.count(), 3, "3 nodes is present." );
    }

    #[test]
    fn clear() {
        let mut tree = Tree::new();
        tree.insert( 254, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
        assert_eq!( tree.count(), 1, "1 node is present." );
        tree.clear();
        assert_eq!( tree.count(), 0, "0 nodes are present." );
    }

    #[test]
    fn delete() {
        let mut tree = Tree::new();
        tree.insert( 68, NodeFeatures { allow_children: true, allow_data: false }, Box::new( 0 ) ).ok();
        assert_eq!( tree.count(), 1, "1 node is present." );
        match tree.delete( 0 ) {
            Err( error ) => println!( "{}", error ),
            Ok( _ ) => println!( "Succeeded to delete node." )
        }
        assert_eq!( tree.count(), 0, "0 nodes are present." );
    }

    #[test]
    fn take() {
        let mut tree = Tree::new();
        tree.insert( 128, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
        tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
        assert_eq!( tree.count(), 1, "1 node is present." );
        let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap(); // Deleting root node, and take data.
        let data = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
        assert_eq!( tree.count(), 0, "0 nodes are present." );
        assert_eq!( *data, "String data".to_string(), "Data of node is a string" );
    }

    #[test]
    fn exists() {
        let mut tree = Tree::new();
        tree.insert( 53, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
        assert!( tree.exists( 0 ) );
        assert!( !tree.exists( 1 ) );
    }

    #[test]
    fn node_type() {
        let mut tree = Tree::new();
        tree.insert( 514, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 5usize ) ).ok();
        let type_any_ref = tree.node_type( 0 ).ok().unwrap().as_ref();
        let type_usize = type_any_ref.downcast_ref::<usize>().unwrap();
        assert_eq!( *type_usize, 5 );
    }

    #[test]
    fn features() {
        let mut tree = Tree::new();
        tree.insert( 16, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
        let features_ref = tree.features( 0 );
        let features = features_ref.as_ref().unwrap();
        assert!( features.allow_children );
        assert!( features.allow_data );
    }

    #[test]
    fn parent() {
        let mut tree = Tree::new();
        tree.insert( 23, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
        tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
        assert_eq!( tree.parent( 1 ).ok(), Some( 0 ), "Parent is root node." );
    }

    #[test]
    fn children() {
        let mut tree = Tree::new();
        tree.insert( 624, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
        tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
        let children = tree.children( 0 ).ok().unwrap();
        assert_eq!( children.len(), 1, "Has 1 child." );
    }

    #[test]
    fn depth() {
        let mut tree = Tree::new();
        tree.insert( 72, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0 ) ).ok();
        tree.insert( 0, NodeFeatures { allow_children: false, allow_data: true }, Box::new( 0 ) ).ok();
        let depth = tree.depth( 1 ).ok().unwrap();
        assert_eq!( depth, 1, "Has 1 child." );
    }

    #[test]
    fn data_mut() {
        let mut tree = Tree::new();
        tree.insert( 974, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0usize ) ).ok();
        tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
        let data_vec_mut = tree.data_mut( 0 ).ok().unwrap();
        let data = data_vec_mut.get_mut( 0 ).unwrap().downcast_mut::<String>().unwrap();
        
        // mutate the data
        *data = "Mutated data".to_string();
        
        // Take node to check if data did mutate.
        let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap(); // Deleting root node, and take data.
        let data_taken = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
        assert_eq!( tree.count(), 0, "0 nodes are present." );
        assert_eq!( *data_taken, "Mutated data".to_string(), "Data of node is a mutated string" );
    }

    #[test]
    fn data_ref() {
        let mut tree = Tree::new();
        tree.insert( 550, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0usize ) ).ok();
        tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
        let data_vec_ref = tree.data_ref( 0 ).ok().unwrap();
        let data = data_vec_ref.get( 0 ).unwrap().downcast_ref::<String>().unwrap();
        assert_eq!( *data, "String data".to_string() );
    }
}
