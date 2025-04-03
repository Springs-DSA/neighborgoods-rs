-- Create the user_certifications table
CREATE TABLE user_certifications (
    user_id UUID NOT NULL REFERENCES users(id),
    cert_id UUID NOT NULL REFERENCES certifications(id),
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, cert_id)
);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_user_certifications_updated_at
BEFORE UPDATE ON user_certifications
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
