use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use crate::schema::subscriptions;
use crate::schema::subscription_tokens;

#[derive(Queryable)]
pub struct Subscription {
    pub email: String,
    pub name: String,
    pub status: String
}

#[derive(Insertable)]
#[diesel(table_name = subscriptions)]
pub struct SubscriptionAdd{
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Utc>,
    pub status: String
}

#[derive(Insertable)]
#[diesel(table_name = subscription_tokens)]
pub struct SubscriptionTokensAdd{
    pub subscription_token: String,
    pub subscriber_id: Uuid
}

#[derive(Queryable)]
pub struct SubscriptionToken{
    pub subscription_token: String,
    pub subscriber_id: Uuid
}


#[derive(Deserialize)]
pub struct SubscribeFormData{
    pub email: String,
    pub name: String,
}
