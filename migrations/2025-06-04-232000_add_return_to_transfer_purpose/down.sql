-- Note: PostgreSQL doesn't support removing enum values directly
-- This would require recreating the enum type and updating all references
-- For now, we'll leave a comment indicating this limitation

-- To properly rollback this migration, you would need to:
-- 1. Create a new enum without 'return'
-- 2. Update all tables using the old enum to use the new one
-- 3. Drop the old enum
-- This is complex and should be done carefully in production

-- For development purposes, you may need to drop and recreate the database
SELECT 'Cannot automatically rollback enum value addition' as warning;
