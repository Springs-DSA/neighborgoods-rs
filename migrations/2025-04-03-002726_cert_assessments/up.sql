-- Create the cert_assessments join table
CREATE TABLE cert_assessments (
    peer_assessment_id UUID NOT NULL REFERENCES peer_assessments(id) ON DELETE CASCADE,
    cert_id UUID NOT NULL REFERENCES certifications(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (peer_assessment_id, cert_id)
);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_cert_assessments_updated_at
BEFORE UPDATE ON cert_assessments
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
