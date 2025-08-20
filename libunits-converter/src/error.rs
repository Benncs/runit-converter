// SPDX-License-Identifier: GPL-3.0-or-later

use thiserror::*;

use crate::datatypes::Dimension;

#[derive(Error, Debug)]
pub enum UnitError {
    #[error("Error in query: {0}")]
    Query(String),

    #[error("Unit doesn´t exist : {0}")]
    BadUnit(String),

    // #[error("Dimensions mismtach {0} {0}")]
    // BadDimension(Dimension, Dimension),
    //
    #[error("Dimensions mismtach")]
    BadDimension,

    #[error("Impossible to parse {0}")]
    ParseError(String),

    #[error("{0}")]
    Custom(String),
}
