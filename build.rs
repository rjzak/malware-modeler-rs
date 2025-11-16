// SPDX-License-Identifier: Apache-2.0

use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

fn main() {
    let mut git = GitclBuilder::default();
    let git = git.sha(true).build().unwrap();

    Emitter::default()
        .add_instructions(&BuildBuilder::all_build().unwrap())
        .unwrap()
        .add_instructions(&git)
        .unwrap()
        .emit()
        .unwrap();
}
