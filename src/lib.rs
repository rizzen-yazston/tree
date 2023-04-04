// This file is part of `tree-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `tree-rizzen-yazston` crate.

//! Welcome to the Tree project.
//! 
//! A simple tree structure containing data nodes. The `tree` crate provides the `Tree` struct, which can store any
//! data that implements the `Any` trait. Methods are provided for the manipulation of the tree structure, and
//! obtaining information regarding the tree structure and its nodes. Manipulating the data of the nodes is done by one
//! method `data_mut()`, which simply provides a mutable reference to the vector containing the data. The `data()`
//! method is just an immutable reference if reading is only required.
//! 
//! Internally the tree makes use of a node struct that contains information about the node in the tree. The
//! information consists of: the immediate parent node, a vector containing children nodes, the features of the node,
//! the node type, the data type, and the data. Except for the features, that determines how the
//! node will behave in the tree, the rest are optional.
//! 
//! The node features are specified by passing an union of features as a parameter `features` of `insert()` and
//! `insert_at()` methods when the node is created and inserted into the tree. Currently the tree supports two
//! features:
//! 
//! - `ALLOW_CHILDREN`: indicates if the node can have children,
//! 
//! - `ALLOW_DATA`: indicates if the node can have data.
//! 
//! At the time of creating the nodes, the `node_type` and `data_type` parameters are passed, these are read only, thus
//! can't be changed later. The node type is normally used to indicate what the node is, especially when the type can't
//! be determined from the node's data, or lacks any data (such as structure information). The data type is generally
//! used when the data of the tree is of different types. The data type aids in determining how to correctly downcast
//! to the actual type. As the node can support multiple data instances, it is recommended that all the data instances
//! are of the same type, due to there being only one data type field for the node. Though it is possible to use an
//! elaborate string than a simple enum to indicate all the data types used in the node.
//! 
//! NOTE: Once `core::error::Error` is no longer experimental, this library will then only depend on the `core`, thus
//! will be suitable for `no-std` environments.
//! 
//! # Examples
//! 
//! This example uses the `String` type for the data type for all the nodes that have data, thus the parameter
//! `data_type` is `None` to indicate it is not used. A string `"String"` could be used to explicitly indicate the
//! data is a `String`. Alternative a simple enum could be used if all the data types are known at compile time to
//! indicate the data type.
//! 
//! Due the data being strings, that carry little structure information (not all nodes contains data), a good choice is
//! to use an enum to indicate the node type. As with `data_type`, strings could also be used for the `node_type`.
//! 
//! ```
//! use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};
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
//! let no_data = ALLOW_CHILDREN;
//! let variable = ALLOW_DATA;
//! 
//! // Build tree of one statement: z = (x + y) / 2
//! // Just ignoring the `Result` using .ok() as this is a trivial example.
//! let mut index = tree.insert( 300, no_data.clone(), Some( Box::new( Nodes::Root ) ), None ).unwrap();
//! tree.insert( index, no_data.clone(), Some( Box::new( Nodes::Statement ) ), None ).ok();
//! tree.insert( 1, no_data.clone(), Some( Box::new( Nodes::Equal ) ), None ).ok();
//! index = tree.insert( 2, variable.clone(), Some( Box::new( Nodes::Leaf ) ), None ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "z".to_string() ) );
//! tree.insert( 2, no_data.clone(), Some( Box::new( Nodes::Divide ) ), None ).ok();
//! tree.insert( 4, no_data.clone(), Some( Box::new( Nodes::Add ) ), None ).ok();
//! index = tree.insert( 5, variable.clone(), Some( Box::new( Nodes::Leaf ) ), None ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "x".to_string() ) );
//! index = tree.insert( 5, variable.clone(), Some( Box::new( Nodes::Leaf ) ), None ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "y".to_string() ) );
//! index = tree.insert( 4, variable.clone(), Some( Box::new( Nodes::Leaf ) ), None ).unwrap();
//! tree.data_mut( index ).unwrap().push( Box::new( "2".to_string() ) );
//! assert_eq!( tree.count(), 9, "9 nodes are present." );
//! ```

pub mod tree;
pub use crate::tree::*;
pub mod error;
pub use error::*;
