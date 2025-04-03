use super::super::schema::cert_assessments;
use rocket_db_pools::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = cert_assessments)]
pub struct CertAssessment {
    pub peer_assessment_id: Uuid,
    pub cert_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
