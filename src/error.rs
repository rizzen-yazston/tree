// This file is part of `tree-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `tree-rizzen-yazston` crate.

use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug )]
#[non_exhaustive]
pub enum TreeError {
    RetrievingNode( usize ),
    NoChildrenAllowed( usize ),
    ExceedsChildren( usize, usize ),
    HasChildren( usize ),
    MissingInParent( usize, usize ),
    RootHasNoParent( usize ),
    NoChildrenFound( usize ),
    NoDataAllowed( usize ),
    NotAncestorOf( usize, usize, Box<TreeError> ),
    IsAncestorOf( usize, usize ),
}

impl Display for TreeError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            TreeError::RetrievingNode( index ) =>
                write!( formatter, "Failed to retrieve the node for the index {}.", index ),
            TreeError::NoChildrenAllowed( index ) =>
                write!( formatter, "No children is allowed for the node {}.", index ),
            TreeError::ExceedsChildren( position, index ) =>
                write!( formatter, "Position {} exceeds length of children for the node {}.", position, index ),
            TreeError::HasChildren( index ) =>
                write!( formatter, "Can't delete the node {} as it still has children.", index ),
            TreeError::MissingInParent( index, parent ) =>
                write!( formatter, "The node {} is missing in the children of its parent {}.", index, parent ),
            TreeError::RootHasNoParent( index ) =>
                write!( formatter, "The root node {} can't have a parent.", index ),
            TreeError::NoChildrenFound( index ) =>
                write!( formatter, "No children were found for the node {}.", index ),
            TreeError::NoDataAllowed( index ) =>
                write!( formatter, "No data is allowed for the node {}.", index ),
            TreeError::NotAncestorOf( index,is_ancestor, ref error ) =>
                write!(
                    formatter,
                    "The node {} is not an ancestor of node {}. The error is: ‘{:#?}’.",
                    is_ancestor,
                    index,
                    error,
                ),
            TreeError::IsAncestorOf( index,is_ancestor ) =>
                write!( formatter, "The node {} is an ancestor of the node {}.", is_ancestor, index, ),
        }
    }
}

// Does not make use of source.
impl Error for TreeError {}
