use crate::auth::{Role, RoleChecker};
use crate::entities::issue::{self, IssueStatus};
use crate::entities::prelude::*;
use crate::entities::target;
use async_graphql::{Context, Object};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use std::sync::Arc;
use tracing::instrument;
#[allow(unused_imports)]
use sea_orm::QueryTrait; //for debugging
#[allow(unused_imports)]
use sea_orm::{DatabaseBackend}; //for debugging

#[derive(Debug)]
pub struct Query;

#[Object]
impl Query {
    #[graphql(guard = "RoleChecker::new(Role::Admin).or(RoleChecker::new(Role::Guest))")]
    #[instrument(skip(ctx))]
    async fn issue<'a>(&self, ctx: &Context<'a>, issue: i32) -> Option<issue::Model> {
        let db = ctx.data::<Arc<DatabaseConnection>>().unwrap().as_ref();
        Issue::find_by_id(issue).one(db).await.unwrap()
    }

    #[graphql(guard = "RoleChecker::new(Role::Admin).or(RoleChecker::new(Role::Guest))")]
    #[instrument(skip(ctx))]
    async fn issues<'a>(
        &self,
        ctx: &Context<'a>,
        issue_status: Option<issue::IssueStatus>,
        oldest_created_date: Option<chrono::NaiveDateTime>,
        newest_created_date: Option<chrono::NaiveDateTime>,
        target: Option<String>,
    ) -> Vec<issue::Model> {
        let db = ctx.data::<Arc<DatabaseConnection>>().unwrap().as_ref();
        let mut select = target::Entity::find().find_with_related(issue::Entity);
        if let Some(status) = issue_status {
            select =
                select.filter(<issue::Entity as sea_orm::EntityTrait>::Column::Status.eq(status));
        } else {
            select = select.filter(
                <issue::Entity as sea_orm::EntityTrait>::Column::Status.ne(IssueStatus::Closed),
            );
        }
        if let Some(t) = target {
            select = select.filter(<target::Entity as sea_orm::EntityTrait>::Column::Name.eq(t));
        }
        if let Some(ocd) = oldest_created_date {
            //TODO thoroughly think through the timezone implications here
            select = select.filter(<issue::Entity as sea_orm::EntityTrait>::Column::CreatedAt.gt(ocd));
        }
        if let Some(ncd) = newest_created_date {
            //TODO thoroughly think through the timezone implications here
            select = select.filter(<issue::Entity as sea_orm::EntityTrait>::Column::CreatedAt.lt(ncd));
        }
        //println!("{:?}", select.clone().build(DatabaseBackend::Sqlite).to_string()); //print the actual query string
        select
            .order_by_asc(crate::entities::target::Column::Name)
            .all(db)
            .await
            .unwrap()
            .into_iter()
            .map(|(_, i)| i)
            .reduce(|mut acc, mut c| {
                acc.append(&mut c);
                acc
            })
            .unwrap_or(vec![])
    }

    #[graphql(guard = "RoleChecker::new(Role::Admin).or(RoleChecker::new(Role::Guest))")]
    #[instrument(skip(_ctx))]
    async fn version<'a>(&self,
        _ctx: &Context<'a>
    ) -> String {
        let mut versionstring = String::new();
        versionstring.push_str(&format!("{}\n", env!("CARGO_PKG_VERSION")).to_string());
        if let Some(hash) = option_env!("VERGEN_GIT_SHA") {
            versionstring.push_str(&format!("Commit hash: {hash}\n").to_string());
        }
        if let Some(desc) = option_env!("VERGEN_GIT_DESCRIBE") {
            versionstring.push_str(&format!("Git: {}\n", desc).to_string());
        }
        if let Some(branch) = option_env!("VERGEN_GIT_BRANCH") {
            versionstring.push_str(&format!("Branch: {}\n", branch).to_string());
        }
        if let Some(bdate) = option_env!("VERGEN_BUILD_TIMESTAMP") {
            versionstring.push_str(&format!("Build Date: {}", bdate).to_string());
        }

        versionstring
    }
}
