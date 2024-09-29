#![allow(unused_imports)]

use crate::backend::{Pg, SqlGenerator};
use crate::{types, Migration, Table};

#[test]
fn in_schema() {
    let sql = Pg::add_column(
        false,
        Some("schema"),
        "author",
        &types::foreign("users", "id", types::ReferentialAction::NoAction, types::ReferentialAction::NoAction),
    );

    assert_eq!(
        sql,
        "\"author\" INTEGER NOT NULL REFERENCES \"schema\".\"users\"(id) ON UPDATE NO ACTION ON DELETE NO ACTION"
    );
}

#[test]
fn ext_schema() {
    let sql = Pg::add_column(
        false,
        Some("schema"),
        "author",
        &types::foreign_schema("other_schema", "users", "id", types::ReferentialAction::NoAction, types::ReferentialAction::NoAction),
    );

    assert_eq!(
        sql,
        "\"author\" INTEGER NOT NULL REFERENCES \"other_schema\".\"users\"(id) ON UPDATE NO ACTION ON DELETE NO ACTION"
    );
}
