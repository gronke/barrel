//! All add_column combinations for mysql
#![allow(unused_imports)]

use crate::backend::{MySql, SqlGenerator};
use crate::types;
use crate::table::{Trigger,TriggerAction,TriggerActionTime};

#[test]
fn trigger_after_insert() {
    let trigger = Trigger {
        table_name: "users".to_string(),
        action: TriggerAction::INSERT,
        time: TriggerActionTime::AFTER
    };
    let sql = MySql::create_or_replace_trigger(&trigger);
    //let sql = MySql::create_or_replace_trigger("users", TriggerAction::INSERT, TriggerActionTime::AFTER);
    assert_eq!(String::from("CREATE OR REPLACE TRIGGER `t_users_after_insert` AFTER INSERT ON `users`"), sql);
}
