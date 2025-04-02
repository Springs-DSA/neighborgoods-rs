-- Create the certifications table
CREATE TABLE certifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    description TEXT,
    lifetime INTEGER, -- stored as number of days
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_certifications_updated_at
BEFORE UPDATE ON certifications
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
