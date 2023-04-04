// This file is part of `tree-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `tree-rizzen-yazston` crate.

// Various unit tests for `Tree`.

use tree::{ Tree, ALLOW_CHILDREN, ALLOW_DATA, TreeError };

#[test]
fn count() {
    let tree = Tree::new();
    assert_eq!( tree.count(), 0, "Has 0 nodes." );
}

#[test]
fn insert() {
    let mut tree = Tree::new();
    tree.insert(
        425,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    assert_eq!( tree.count(), 1, "1 node is present." );
}

#[test]
fn insert_at() {
    let mut tree = Tree::new();
    tree.insert(
        4,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert_at(
        0,
        0,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    assert_eq!( tree.count(), 3, "3 nodes is present." );
}

#[test]
fn clear() {
    let mut tree = Tree::new();
    tree.insert(
        254,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    assert_eq!( tree.count(), 1, "1 node is present." );
    tree.clear();
    assert_eq!( tree.count(), 0, "0 nodes are present." );
}

#[test]
fn delete() {
    let mut tree = Tree::new();
    tree.insert(
        68,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
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
    tree.insert(
        128,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    assert_eq!( tree.count(), 1, "1 node is present." );
    
    // Deleting root node, and take data.
    let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap();
    let data = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
    assert_eq!( tree.count(), 0, "0 nodes are present." );
    assert_eq!( *data, "String data".to_string(), "Data of node is a string" );
}

#[test]
fn exists() {
    let mut tree = Tree::new();
    tree.insert(
        53,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    assert!( tree.exists( 0 ) );
    assert!( !tree.exists( 1 ) );
}

#[test]
fn node_type() {
    let mut tree = Tree::new();
    tree.insert(
        514,
        ALLOW_CHILDREN | ALLOW_DATA,
        Some( Box::new( "node type 2".to_string() ) ),
        None,
    ).ok();
    let type_any_ref = tree.node_type( 0 ).ok().unwrap().as_ref().unwrap();
    let type_usize = type_any_ref.downcast_ref::<String>().unwrap();
    assert_eq!( *type_usize, "node type 2" );
}

#[test]
fn features() {
    let mut tree = Tree::new();
    tree.insert(
        16,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    let features_ref = tree.features( 0 );
    let features = *features_ref.unwrap();
    assert_eq!( features & ALLOW_CHILDREN, ALLOW_CHILDREN );
    assert_eq!( features & ALLOW_DATA, ALLOW_DATA );
}

#[test]
fn parent() {
    let mut tree = Tree::new();
    tree.insert(
        23,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    assert_eq!( tree.parent( 1 ).ok(), Some( 0 ), "Parent is root node." );
}

#[test]
fn no_parent() {
    let mut tree = Tree::new();
    tree.insert(
        23,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    let mut result = false;
    match tree.parent( 0 ).err().unwrap() {
        TreeError::RootHasNoParent( _ ) => result = true,
        _ => {}
    };
    assert!( result, "Root node has no parent." );
}

#[test]
fn children() {
    let mut tree = Tree::new();
    tree.insert(
        624,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    let children = tree.children( 0 ).ok().unwrap();
    assert_eq!( children.len(), 1, "Has 1 child." );
}

#[test]
fn first() {
    let mut tree = Tree::new();
    tree.insert(
        713,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    let first = tree.first( 0 ).ok().unwrap();
    assert_eq!( first, 1, "First child is index 1." );
}

#[test]
fn last() {
    let mut tree = Tree::new();
    tree.insert(
        42,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    let last = tree.last( 0 ).ok().unwrap();
    assert_eq!( last, 1, "Last child is index 1." );
}

#[test]
fn child() {
    let mut tree = Tree::new();
    tree.insert(
        921,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    let child = tree.child( 0, 0 ).ok().unwrap();
    assert_eq!( child, 1, "Has 1 child with index 1." );
}

#[test]
fn depth() {
    let mut tree = Tree::new();
    tree.insert(
        72,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    let depth = tree.depth( 1 ).ok().unwrap();
    assert_eq!( depth, 1, "Has 1 child." );
}

#[test]
fn data_mut() {
    let mut tree = Tree::new();
    tree.insert(
        974,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    let data_vec_mut = tree.data_mut( 0 ).ok().unwrap();
    let data = data_vec_mut.get_mut( 0 ).unwrap().downcast_mut::<String>().unwrap();
    
    // mutate the data
    *data = "Mutated data".to_string();
    
    // Take node to check if data did mutate.
    let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap();
    let data_taken = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
    assert_eq!( tree.count(), 0, "0 nodes are present." );
    assert_eq!( *data_taken, "Mutated data".to_string(), "Data of node is a mutated string" );
}

#[test]
fn data_ref() {
    let mut tree = Tree::new();
    tree.insert(
        550,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        Some( Box::new( "String".to_string() ) ),
    ).ok();
    tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    let data_vec_ref = tree.data_ref( 0 ).ok().unwrap();
    let data = data_vec_ref.get( 0 ).unwrap().downcast_ref::<String>().unwrap();
    assert_eq!( *data, "String data".to_string() );
}

#[test]
fn data_type() {
    let mut tree = Tree::new();
    tree.insert(
        514,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        Some( Box::new( "String".to_string() ) ),
    ).ok();
    let type_any_ref = tree.data_type( 0 ).ok().unwrap().as_ref().unwrap();
    let data_type = type_any_ref.downcast_ref::<String>().unwrap();
    assert_eq!( *data_type, "String" );
}

#[test]
fn insert_uses_deleted_node() {
    let mut tree = Tree::new();
    tree.insert(
        338,
        ALLOW_CHILDREN | ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    assert_eq!( tree.count(), 4, "4 nodes are present." );
    assert_eq!( tree.len(), 4, "Node vector length is 4." );
    tree.delete( 1 ).ok();
    tree.delete( 2 ).ok();
    tree.insert(
        0,
        ALLOW_DATA,
        None,
        None,
    ).ok();
    assert_eq!( tree.count(), 3, "3 nodes are present." );
    assert_eq!( tree.len(), 4, "Node vector length is 4." );
    assert!( !tree.exists( 2 ), "Position 2 is None." );
    assert!( tree.exists( 3 ), "Position 3 is node." );
}

#[test]
fn is_ancestor_of() {
    let mut tree = Tree::new();
    tree.insert(
        338,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    let last = tree.insert(
        1,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok().unwrap();
    assert_eq!( last, 3 );
    let mut result = tree.is_ancestor_of( 3, 0 ).unwrap();
    assert!( result, "Root is grandparent of node 3." );
    result = tree.is_ancestor_of( 3, 2 ).ok().unwrap();
    assert!( !result, "Node 2 is not a parent of node 3." );
}

#[test]
fn move_nodes() {
    let mut tree = Tree::new();
    tree.insert(
        338,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert(
        0,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    tree.insert(
        1,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok().unwrap();
    tree.insert(
        3,
        ALLOW_CHILDREN,
        None,
        None,
    ).ok();
    assert_eq!( tree.parent( 3 ).unwrap(), 1, "Parent of node 3 must be 1." );
    tree.move_nodes( 3, 2, None ).ok();
    assert_eq!( tree.parent( 3 ).unwrap(), 1, "Parent of node 3 must be 2." );
    assert_eq!( tree.parent( 4 ).unwrap(), 3, "Parent of node 4 must be 3." );
}
