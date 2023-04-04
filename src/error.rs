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
                write!( formatter, "Failed to retrieve the node for index {}.", index ),
            TreeError::NoChildrenAllowed( index ) =>
                write!( formatter, "No children allowed for the node {}.", index ),
            TreeError::ExceedsChildren( position, index ) =>
                write!( formatter, "Position {} exceeds length of children for node {}.", position, index ),
            TreeError::HasChildren( index ) =>
                write!( formatter, "Can't delete node {} as it has children.", index ),
            TreeError::MissingInParent( index, parent ) =>
                write!( formatter, "Node {} is missing in the children of its parent {}.", index, parent ),
            TreeError::RootHasNoParent( index ) =>
                write!( formatter, "Root node {} can't have parent.", index ),
            TreeError::NoChildrenFound( index ) =>
                write!( formatter, "No children found for the node {}.", index ),
            TreeError::NoDataAllowed( index ) =>
                write!( formatter, "No data allowed for the node {}.", index ),
            TreeError::NotAncestorOf( index,is_ancestor, ref error ) =>
                write!(
                    formatter,
                    "Node {} is not an ancestor of node {}. Error is: ‘{:#?}’.",
                    is_ancestor,
                    index,
                    error,
                ),
            TreeError::IsAncestorOf( index,is_ancestor ) =>
                write!( formatter, "Node {} is an ancestor of node {}.", is_ancestor, index, ),
        }
    }
}

// Does not make use of source.
impl Error for TreeError {}
