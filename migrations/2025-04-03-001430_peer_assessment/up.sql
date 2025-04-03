-- Create enum type for assessment
CREATE TYPE assessment_type AS ENUM ('POSITIVE', 'CRITICAL', 'OTHER');

-- Create peer assessments table
CREATE TABLE peer_assessments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    assessor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    subject_id UUID REFERENCES users(id) ON DELETE SET NULL,
    assessment assessment_type NOT NULL,
    comments TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_peer_assessments_updated_at
BEFORE UPDATE ON peer_assessments
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
