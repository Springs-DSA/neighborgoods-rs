use super::super::schema::peer_assessments;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[db_enum(existing_type_path = "crate::schema::sql_types::AssessmentType")]
pub enum AssessmentType {
    POSITIVE,
    CRITICAL,
    OTHER,
}

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = peer_assessments)]
pub struct PeerAssessment {
    pub id: Uuid,
    pub assessor_id: Option<Uuid>,
    pub subject_id: Option<Uuid>,
    pub assessment: AssessmentType,
    pub comments: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
