-- Drop the trigger
DROP TRIGGER IF EXISTS update_node_settings_updated_at ON node_settings;

-- Drop the function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop the table
DROP TABLE IF EXISTS node_settings;
