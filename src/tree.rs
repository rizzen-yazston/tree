// This file is part of `tree-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `tree-rizzen-yazston` crate.

use crate::TreeError;
use core::any::Any;

/// Indicates that the node can have children.
/// 
///  Used for the `features` parameter of the [`insert`] and [`insert_at`] methods.
/// 
/// [`insert`]: Tree::insert
/// [`insert_at`]: Tree::insert_at
pub const ALLOW_CHILDREN: u8 = 0b00000001;

/// Indicates that the node can have data.
/// 
///  Used for the `features` parameter of the [`insert`] and [`insert_at`] methods.
/// 
/// [`insert`]: Tree::insert
/// [`insert_at`]: Tree::insert_at
pub const ALLOW_DATA: u8 = 0b00000010;

/// See the crate's information page for details regarding the struct.
pub struct Tree {
    nodes: Vec<Option<Node>>,
    root: Option<usize>
}

impl Tree {

    // -- Tree structure manipulation --

    /// Create a new empty tree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 0, ALLOW_CHILDREN, None, None ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// ```
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            root: None
        }
    }

    /// Create a node, and append it to the end of the `node_index` node's children.
    /// 
    /// The `features` parameter specifies the features of the node in how it will behave. The features are bitwise
    /// flags and can be selected, and simply or'ed (`|`) together when passing the selected features.
    /// 
    /// Available features are:
    /// 
    /// - [`ALLOW_CHILDREN`]: indicates if the node can have children,
    /// 
    /// - [`ALLOW_DATA`]: indicates if the node can have data.
    /// 
    /// Both `node_type` and `data_type` are optional, though normally one of them is used, and are read-only once the
    /// node has been created. The node type is generally used when the data type of the node is not specified, or the
    /// data type. As these fields are not used internally of the `Tree` methods, they are of any type that implements
    /// the [`Any`] trait, and thus allows greatest flexibility in how these two fields are used by the user of the
    /// `Tree`. The data type is used to indicate the type of data stored within in the node. 
    /// 
    /// The data for the node is added or manipulated by using the [`data_mut`] method.
    /// 
    /// If there is no root node for the tree, then the value of `node_index` will be discarded (ignored).
    /// 
    /// If no error, the returned [`usize`] value is the index of the created node in the tree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 425, ALLOW_CHILDREN, None, None ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// ```
    /// 
    /// [`ALLOW_CHILDREN`]: ALLOW_CHILDREN
    /// [`ALLOW_DATA`]: ALLOW_DATA
    /// [`Any`]: core::any::Any
    /// [`data_mut`]: Tree::data_mut
    /// [`usize`]: usize
    pub fn insert(
        &mut self,
        node_index: usize,
        features: u8,
        node_type: Option<Box<dyn Any>>,
        data_type: Option<Box<dyn Any>>,
    ) -> Result<usize, TreeError> {
        let mut children = None;
        let mut parent = None;
        let mut data = None;

        // `node_index` is ignored when first node is inserted into tree.
        if !self.root.is_none() {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( TreeError::RetrievingNode( node_index ) )
            };
            if index_node.features & ALLOW_CHILDREN != ALLOW_CHILDREN {
                return Err( TreeError::NoChildrenAllowed( node_index ) );
            }
            parent = Some( node_index );
        }
        if features & ALLOW_CHILDREN == ALLOW_CHILDREN {
            children = Some( Vec::<usize>::new() );
        }
        if features & ALLOW_DATA == ALLOW_DATA {
            data = Some( Vec::<Box<dyn Any>>::new() );
        }
        let node = Some( Node {
            node_type,
            features,
            parent,
            children,
            data,
            data_type,
        } );
        let mut _index = 0;
        match self.nodes.iter().position( |x| x.is_none() ) {
            None => {
                _index = self.nodes.len();
                self.nodes.push( node );
            },
            Some( position ) => {
                _index = position;
                *self.nodes.get_mut( position ).unwrap() = node; 
            }
        }
        if self.root.is_none() {
            self.root = Some( _index );
        } else {
            let Some( index_node ) = self.node_mut( node_index ) else {
                return Err( TreeError::RetrievingNode( node_index ) )
            };
            index_node.children.as_mut().unwrap().push( _index );
        }
        Ok( _index )
    }

    /// Create a node and insert as a child to the `node_index` node at the `position` specified. The `position` must
    /// be in the range of 0 to number of children.
    /// 
    /// See [`insert`] for usage details, as `insert_at` only differs with the additional `position` parameter.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 4, ALLOW_CHILDREN, None, None ).ok();
    /// tree.insert( 0, ALLOW_CHILDREN, None, None ).ok();
    /// tree.insert_at( 0, 0, ALLOW_CHILDREN, None, None ).ok();
    /// assert_eq!( tree.count(), 3, "3 nodes is present." );
    /// ```
    /// 
    /// [`insert`]: Tree::insert
    pub fn insert_at(
        &mut self,
        node_index: usize,
        position: usize,
        features: u8,
        node_type: Option<Box<dyn Any>>,
        data_type: Option<Box<dyn Any>>,
    ) -> Result<usize, TreeError> {
        let mut children = None;
        let mut parent = None;
        let mut data = None;

        // `node_index` is ignored when first node is inserted into tree.
        if !self.root.is_none() {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( TreeError::RetrievingNode( node_index ) )
            };
            if index_node.features & ALLOW_CHILDREN != ALLOW_CHILDREN {
                return Err( TreeError::NoChildrenAllowed( node_index ) );
            }
            if position > index_node.children.as_ref().unwrap().len() {
                return Err( TreeError::ExceedsChildren( position, node_index ) );
            }
            parent = Some( node_index );
        }
        if features & ALLOW_CHILDREN == ALLOW_CHILDREN {
            children = Some( Vec::<usize>::new() );
        }
        if features & ALLOW_DATA == ALLOW_DATA {
            data = Some( Vec::<Box<dyn Any>>::new() );
        }
        let node = Some( Node {
            node_type,
            features,
            parent,
            children,
            data,
            data_type,
        } );
        let mut _index = 0;
        match self.nodes.iter().position( |x| x.is_none() ) {
            None => {
                _index = self.nodes.len();
                self.nodes.push( node );
            },
            Some( position ) => {
                _index = position;
                *self.nodes.get_mut( position ).unwrap() = node; 
            }
        }
        if self.root.is_none() {
            self.root = Some( _index );
        } else {
            let Some( index_node ) = self.node_mut( node_index ) else {
                return Err( TreeError::RetrievingNode( node_index ) )
            };
            index_node.children.as_mut().unwrap().insert( position, _index );
        }
        Ok( _index )
    }

    /// Deletes the specified node `node_index` from the tree.
    /// 
    /// # WARNING
    /// 
    /// This is a destructive method that destroys the data when deleting the node.
    /// 
    /// If wanting the data, use [`take`] method instead.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 68, ALLOW_CHILDREN, None, None ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// match tree.delete( 0 ) {
    ///     Err( error ) => println!( "{}", error ),
    ///     Ok( _ ) => println!( "Succeeded to delete node." )
    /// }
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// ```
    /// 
    /// [`take`]: Tree::take
    pub fn delete( &mut self, node_index: usize ) -> Result<(), TreeError> {
        let mut _parent = None;
        {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( TreeError::RetrievingNode( node_index ) )
            };
            if
                index_node.features & ALLOW_CHILDREN == ALLOW_CHILDREN
                && !index_node.children.as_ref().unwrap().is_empty()
            {
                return Err( TreeError::HasChildren( node_index ) );
            }
            _parent = index_node.parent;
        }
        {
            if !_parent.is_none() {
                let parent = _parent.unwrap();
                let Some( parent_node ) = self.node_mut( parent ) else {
                    return Err( TreeError::RetrievingNode( parent ) )
                };
                let children = parent_node.children.as_mut().unwrap();
                let Some( _position ) = children.iter().position( |&x| x == node_index ) else {
                    return Err( TreeError::MissingInParent( node_index, parent ) ); // Serious integrity issue.
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

    /// Deletes the specified node `node_index` from the tree, and return its data (if any).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 128, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap(); // Deleting root node, and take data.
    /// let data = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// assert_eq!( *data, "String data".to_string(), "Data of node is a string" );
    /// ```
    pub fn take( &mut self, node_index: usize ) -> Result<Option<Vec<Box<dyn Any>>>, TreeError> {
        let mut _parent = None;
        {
            let Some( index_node ) = self.node( node_index ) else {
                return Err( TreeError::RetrievingNode( node_index ) )
            };
            if
                index_node.features & ALLOW_CHILDREN == ALLOW_CHILDREN
                && !index_node.children.as_ref().unwrap().is_empty()
            {
                return Err( TreeError::HasChildren( node_index ) );
            }
            _parent = index_node.parent;
        }
        {
            if !_parent.is_none() {
                let parent = _parent.unwrap();
                let Some( parent_node ) = self.node_mut( parent ) else {
                    return Err( TreeError::RetrievingNode( parent ) )
                };
                let children = parent_node.children.as_mut().unwrap();
                let Some( _position ) = children.iter().position( |&x| x == node_index ) else {
                    return Err( TreeError::MissingInParent( node_index, parent ) ); // Serious integrity issue.
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
    /// # WARNING
    /// 
    /// All data in the nodes will be destroyed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 254, ALLOW_CHILDREN, None, None ).ok();
    /// assert_eq!( tree.count(), 1, "1 node is present." );
    /// tree.clear();
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// ```
    pub fn clear( &mut self ) {
        self.root = None;
        self.nodes.clear();
    }

    /// Move part of the tree from one position to another within the tree.
    /// 
    /// The `destination` node must be able to have children, else move will not occur. Also the `source` node can't
    /// already be an ancestor of `destination` node.
    /// 
    /// Parameter `position` is optional, and when passed as `None` the position is taken to be the last child of the
    /// `destination` node.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{ Tree, ALLOW_CHILDREN, ALLOW_DATA, TreeError };
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 338, ALLOW_CHILDREN, None, None, ).ok();
    /// tree.insert( 0, ALLOW_CHILDREN, None, None, ).ok();
    /// tree.insert( 0, ALLOW_CHILDREN, None, None, ).ok();
    /// tree.insert( 1, ALLOW_CHILDREN, None, None, ).ok().unwrap();
    /// tree.insert( 3, ALLOW_CHILDREN, None, None, ).ok();
    /// assert_eq!( tree.parent( 3 ).unwrap(), 1, "Parent of node 3 must be 1." );
    /// tree.move_nodes( 3, 2, None ).ok();
    /// assert_eq!( tree.parent( 3 ).unwrap(), 1, "Parent of node 3 must be 2." );
    /// assert_eq!( tree.parent( 4 ).unwrap(), 3, "Parent of node 4 must be 3." );
    /// ```
    pub fn move_nodes(
        &mut self,
        source: usize,
        destination: usize,
        position: Option<usize>
    ) -> Result<(), TreeError> {
        
        // Check if destination allows for children.
        {
            let Some( node ) = self.node( destination ) else {
                return Err( TreeError::RetrievingNode( destination ) )
            };
            if node.features & ALLOW_CHILDREN != ALLOW_CHILDREN {
                return Err( TreeError::NoChildrenAllowed( destination ) );
            }
        }

        // Check that source is not an ancestor to destination.
        match self.is_ancestor_of( destination, source ) {
            Ok( _ ) => return Err( TreeError::IsAncestorOf( source, destination ) ),
            Err( _ ) => {}
        };
        let mut _parent = None;
        {
            let Some( index_node ) = self.node( source ) else {
                return Err( TreeError::RetrievingNode( source ) )
            };
            _parent = index_node.parent;
        }
        let parent = _parent.unwrap();

        // Check if source is already a child of destination, if so just a position change in destination's children.
        if parent == destination {
            let Some( node ) = self.node_mut( destination ) else {
                return Err( TreeError::RetrievingNode( destination ) )
            };
            let children = node.children.as_mut().unwrap();
            let Some( source_position ) = children.iter().position( |&x| x == source ) else {
                return Err( TreeError::MissingInParent( source, destination ) ); // Serious integrity issue.
            };
            let destination_position = match position {
                Some( value ) => value,
                None => children.len() - 1
            };
            if source_position == destination_position {
                // Nothing to do.
            } else if destination_position < source_position {
                children.remove( source_position );
                children.insert( destination_position, source );
            } else {
                children.insert( destination_position, source );
                children.remove( source_position );
            }
            return Ok( () )
        }

        // Remove source from source's parent's children.
        {
            let Some( node ) = self.node_mut( parent ) else {
                return Err( TreeError::RetrievingNode( parent ) )
            };
            let children = node.children.as_mut().unwrap();
            let Some( _position ) = children.iter().position( |&x| x == source ) else {
                // Serious integrity issue.
                return Err( TreeError::MissingInParent( source, parent ) );
            };
            children.remove( _position );
        }

        // Add source to destination's children
        {
            let Some( node ) = self.node_mut( destination ) else {
                return Err( TreeError::RetrievingNode( destination ) )
            };
            let children = node.children.as_mut().unwrap();
            let destination_position = match position {
                Some( value ) => value,
                None => children.len() - 1
            };
            children.insert( destination_position, source );
        }

        // Change source's parent to destination
        let Some( node ) = self.node_mut( source ) else {
            return Err( TreeError::RetrievingNode( source ) )
        };
        node.parent = Some( destination );
        Ok( () )
    }

    // -- information methods --

    /// Check if `node_index` exists in the tree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 53, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
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

    /// Obtain reference to the node type for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 514, ALLOW_CHILDREN | ALLOW_DATA, Some( Box::new( "node type 2".to_string() ) ), None ).ok();
    /// let type_any_ref = tree.node_type( 0 ).ok().unwrap().as_ref().unwrap();
    /// let type_usize = type_any_ref.downcast_ref::<String>().unwrap();
    /// assert_eq!( *type_usize, "node type 2" );
    /// ```
    pub fn node_type( &self, node_index: usize ) -> Result<&Option<Box<dyn Any>>, TreeError> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) )
        };
        Ok( &index_node.node_type )
    }

    /// Obtain reference to the node's features for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 16, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// let features_ref = tree.features( 0 );
    /// let features = *features_ref.as_ref().unwrap();
    /// assert_eq!( features & ALLOW_CHILDREN, ALLOW_CHILDREN );
    /// assert_eq!( features & ALLOW_DATA, ALLOW_DATA );
    /// ```
    pub fn features( &self, node_index: usize ) -> Result<&u8, TreeError> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) )
        };
        Ok( &index_node.features )
    }

    /// Obtain reference to the node's immediate parent for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 23, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// assert_eq!( tree.parent( 1 ).ok(), Some( 0 ), "Parent is root node." );
    /// ```
    pub fn parent( &self, node_index: usize ) -> Result<usize, TreeError> {
        if Some( node_index ) == self.root {
            return Err( TreeError::RootHasNoParent( node_index ) );
        }
        let Some( index_node ) = self.node( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) )
        };
        Ok( *index_node.parent.as_ref().unwrap() )
    }

    /// Determine if a node `is_ancestor` is an ancestor of the specified node `node_index`. This method will iterate
    /// through the parents until the root node.
    /// 
    /// `true` is returned if the node is found to be ancestor of the specified node, else `false` is returned.
    /// 
    /// # Example
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 338, ALLOW_CHILDREN, None, None, ).ok();
    /// tree.insert( 0, ALLOW_CHILDREN, None, None, ).ok();
    /// tree.insert( 0, ALLOW_CHILDREN, None, None, ).ok();
    /// let last = tree.insert( 1, ALLOW_CHILDREN, None, None, ).ok().unwrap();
    /// assert_eq!( last, 3 );
    /// let mut result = tree.is_ancestor_of( 3, 0 ).unwrap();
    /// assert!( result, "Root is grandparent of node 3." );
    /// result = tree.is_ancestor_of( 3, 2 ).unwrap();
    /// assert!( !result, "Node 2 is not a parent of node 3." );
    /// ```
    pub fn is_ancestor_of( &self, node_index: usize, is_ancestor: usize ) -> Result<bool, TreeError> {
        let parent = match self.parent( node_index ) {
            Ok( result ) => result,
            Err( error ) => return match error {
                TreeError::RootHasNoParent( _ ) => Ok( false ),
                _ => Err( error )
            }
        };
        if parent == is_ancestor {
            return Ok( true );
        }
        Ok( self.is_ancestor_of( parent, is_ancestor )? )
    }

    /// Obtain reference to the node children for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 624, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// let children = tree.children( 0 ).ok().unwrap();
    /// assert_eq!( children.len(), 1, "Has 1 child." );
    /// ```
    pub fn children( &self, node_index: usize ) -> Result<&Vec<usize>, TreeError> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) )
        };
        if index_node.features & ALLOW_CHILDREN != ALLOW_CHILDREN {
            return Err( TreeError::NoChildrenAllowed( node_index ) );
        }
        Ok( &index_node.children.as_ref().unwrap() )
    }

    /// Convenience method to obtain the first child of the node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 624, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// let first = tree.first( 0 ).ok().unwrap();
    /// assert_eq!( first, 1, "First child is index 1." );
    /// ```
    pub fn first( &self, node_index: usize ) -> Result<usize, TreeError> {
        let children = self.children( node_index )?;
        let Some( index ) = children.first() else {
            return Err( TreeError::NoChildrenFound( node_index ) )
        };
        Ok( *index )
    }

    /// Convenience method to obtain the last child of the node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 624, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// let last = tree.last( 0 ).ok().unwrap();
    /// assert_eq!( last, 1, "Last child is index 1." );
    /// ```
    pub fn last( &self, node_index: usize ) -> Result<usize, TreeError> {
        let children = self.children( node_index )?;
        let Some( index ) = children.last() else {
            return Err( TreeError::NoChildrenFound( node_index ) )
        };
        Ok( *index )
    }

    /// Convenience method to obtain the nth child `position` of the node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 624, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// let child = tree.child( 0, 0 ).ok().unwrap();
    /// assert_eq!( child, 1, "Has 1 child with index 1." );
    /// ```
    pub fn child( &self, node_index: usize, position: usize ) -> Result<usize, TreeError> {
        let children = self.children( node_index )?;
        let Some( index ) = children.get( position ) else {
            return Err( TreeError::NoChildrenFound( node_index ) )
        };
        Ok( *index )
    }

    /// Obtain the depth of the specified node `node_index` from the root.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 72, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// let depth = tree.depth( 1 ).ok().unwrap();
    /// assert_eq!( depth, 1, "Has 1 child." );
    /// ```
    pub fn depth( &self, mut node_index: usize ) -> Result<usize, TreeError> {
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
                return Err( TreeError::RetrievingNode( node_index ) );
            }
        }
    }

    /// Get length of internal vector of nodes, including the empty nodes (deleted/taken).
    /// 
    /// For actual number of nodes in the tree, use [`count`] method.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 297, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// match tree.delete( 1 ) {
    ///     Err( error ) => println!( "{}", error ),
    ///     Ok( _ ) => println!( "Succeeded to delete node." )
    /// }
    /// assert_eq!( tree.count(), 1, "Has 1 node." );
    /// assert_eq!( tree.len(), 2, "Internal vector is 2." );
    /// ```
    /// 
    /// [`count`]: Tree::count
    pub fn len( &self ) -> usize {
        self.nodes.len()
    }

    /// Count the nodes of the tree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 297, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.insert( 0, ALLOW_DATA, None, None ).ok();
    /// assert_eq!( tree.count(), 2, "Has 2 nodes." );
    /// ```
    pub fn count( &self ) -> usize {
        self.nodes.iter().filter( |n| !n.is_none() ).count()
    }

    // -- Data methods --

    /// Obtain a mutable reference to the node's data for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 974, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
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
    pub fn data_mut( &mut self, node_index: usize ) -> Result<&mut Vec<Box<dyn Any>>, TreeError> {
        let Some( index_node ) = self.node_mut( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) );
        };
        if index_node.features & ALLOW_DATA != ALLOW_DATA/* !index_node.features.allow_data*/ {
            return Err( TreeError::NoDataAllowed( node_index ) );
        }
        Ok( index_node.data.as_mut().unwrap() )
    }

    /// Obtain an immutable reference to the node's data for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 550, ALLOW_CHILDREN | ALLOW_DATA, None, None ).ok();
    /// tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    /// let data_vec_ref = tree.data_ref( 0 ).ok().unwrap();
    /// let data = data_vec_ref.get( 0 ).unwrap().downcast_ref::<String>().unwrap();
    /// assert_eq!( *data, "String data".to_string() );
    /// ```
    pub fn data_ref( &self, node_index: usize ) -> Result<&Vec<Box<dyn Any>>, TreeError> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) )
        };
        if index_node.features & ALLOW_DATA != ALLOW_DATA/* !index_node.features.allow_data*/ {
            return Err( TreeError::NoDataAllowed( node_index ) );
        }
        Ok( &index_node.data.as_ref().unwrap() )
    }

    /// Obtain reference to the data type for the specified node `node_index`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 514, ALLOW_CHILDREN | ALLOW_DATA, None, Some( Box::new( "String".to_string() ) ) ).ok();
    /// let type_any_ref = tree.data_type( 0 ).ok().unwrap().as_ref().unwrap();
    /// let data_type = type_any_ref.downcast_ref::<String>().unwrap();
    /// assert_eq!( *data_type, "String" );
    /// ```
    pub fn data_type( &self, node_index: usize ) -> Result<&Option<Box<dyn Any>>, TreeError> {
        let Some( index_node ) = self.node( node_index ) else {
            return Err( TreeError::RetrievingNode( node_index ) )
        };
        Ok( &index_node.data_type )
    }

    // -- Internal methods --

    fn node( &self, node_index: usize ) -> Option<&Node> {
        if let Some( option ) = self.nodes.get( node_index ) {
            if let Some( node ) = option {
                return Some( node );
            }
        }
        None
    }

    fn node_mut( &mut self, node_index: usize ) -> Option<&mut Node> {
        if let Some( option ) = self.nodes.get_mut( node_index ) {
            if let Some( node ) = option {
                return Some( node );
            }
        }
        None
    }
}

// Internal structs, functions, etc.

struct Node {
    node_type: Option<Box<dyn Any>>,
    features: u8,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
    data: Option<Vec<Box<dyn Any>>>,
    data_type: Option<Box<dyn Any>>,
}
