// RGB Core Library: a reference implementation of RGB smart contract standards.
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! Data structures and APIs related to RGB data storage and data exchange
//! tasks.
//!
//! There are two main APIs which may be used to access RGB contract data (both
//! client-side-validated and data containers): [`Stash`] API and [`GraphAPI`].
//! They differ in the fact that [`Stash`] manages long-term persistance of
//! large-volume data, while [`GraphAPI`] is used by the data structures fitting
//! into the computer memory (`Consignment`s and `Disclosure`s of the standard
//! library).
//!
//! These data structures are not part of the client-side-validation
//! commitments (like [`ConsensusCommit`] and [`CommitEncode`]), however, some
//! of them MAY participate client-side-validation process (see
//! `Consignments`) or MAY have a restricted forms of validation (see
//! `Disclosure`), while others are constantly maintained in valid state by
//! the data management procedures ([`Stash`]).

mod anchor;
mod graph;
#[allow(clippy::module_inception)]
mod stash;
pub mod bundle;
mod consignment;

pub use anchor::ConcealAnchors;
pub use bundle::{BundleId, ConcealTransitions, TransitionBundle};
pub use consignment::{AnchoredBundle, Consignment, ConsignmentEndpoint};
pub use graph::{ConsistencyError, GraphApi};
pub use stash::Stash;
