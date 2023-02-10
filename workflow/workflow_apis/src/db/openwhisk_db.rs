use super::UserRepository;

use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::models::{ActionTable, NewActionDetails};
use crate::schema::action_details;
use color_eyre::Result;
use diesel::QueryDsl;
use tracing::instrument;
use uuid::Uuid;

impl UserRepository {
    #[instrument(skip(self, new_user))]
    pub async fn create_rule_table(&self, new_user: NewActionDetails) -> Result<ActionTable> {
        let user = new_user;
        let result = diesel::insert_into(action_details::table)
            .values(user.clone())
            .on_conflict(action_details::rule)
            .do_update()
            .set(user)
            .get_result(&self.pool.get().unwrap());
        Ok(result.unwrap())
    }

    #[instrument(skip(self))]
    pub async fn find_by_rule(&self, rule: &str) -> Result<Option<ActionTable>> {
        let conn = self.pool.get().unwrap();
        let mut items = action_details::table
            .filter(action_details::rule.eq(rule.clone()))
            .load::<ActionTable>(&conn)?;
        let res = items.pop();
        Ok(res)
    }

    #[instrument(skip(self))]
    pub async fn find_rule_by_user_id(&self, user_id: &Uuid) -> Result<Vec<ActionTable>> {
        let conn = self.pool.get().unwrap();
        let items = action_details::table
            .filter(action_details::user_id.eq(user_id.clone()))
            .load::<ActionTable>(&conn)?;
        Ok(items)
    }

    #[instrument(skip(self))]
    pub async fn find_rule_by_user_id_and_rule(
        &self,
        user_id: &Uuid,
        rule: String,
    ) -> Option<ActionTable> {
        let conn = self.pool.get().unwrap();
        let mut items = action_details::table
            .filter(action_details::user_id.eq(user_id.clone()))
            .filter(action_details::rule.eq(rule.clone()))
            .load::<ActionTable>(&conn)
            .unwrap();
        let res = items.pop();
        res
    }
}
