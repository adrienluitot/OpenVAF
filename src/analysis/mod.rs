//  * ******************************************************************************************
//  * Copyright (c) 2019 Pascal Kuthe. This file is part of the VARF project.
//  * It is subject to the license terms in the LICENSE file found in the top-level directory
//  *  of this distribution and at  https://gitlab.com/DSPOM/VARF/blob/master/LICENSE.
//  *  No part of VARF, including this file, may be copied, modified, propagated, or
//  *  distributed except according to the terms contained in the LICENSE file.
//  * *******************************************************************************************

pub mod dominator_tree;
pub use dominator_tree::DominatorTree;
mod constant_folding;
pub use constant_folding::ConstantFoldState;
pub mod data_flow;
mod extraction;
#[cfg(test)]
mod test;
pub use extraction::ExtractionDependencyHandler;