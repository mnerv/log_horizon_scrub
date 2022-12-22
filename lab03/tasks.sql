/*
 * @file   tasks.sql
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Lab04 tasks.
 * @date   2022-12-22
 *
 * @copyright Copyright (c) 2022
 */
-- Part I. Trigger calling a Function

-- 1.

CREATE OR REPLACE FUNCTION registration_all_graded()
RETURNS TRIGGER
LANGUAGE plpgsql
AS
$$
DECLARE num INT;
BEGIN
    num := (SELECT COUNT(*) FROM student AS s
           INNER JOIN registration AS r ON s.id = r.student_id
           WHERE LOWER(r.status) = LOWER('NS') AND r.course_code = 'PIS32');
    
    IF num = 0 THEN
        RAISE NOTICE 'All % students in PIS32 are graded!', num;
    END IF;
    return NULL;
END;
$$

CREATE TRIGGER trg_registration_all_graded
    AFTER UPDATE
    ON registration
    FOR EACH STATEMENT
    EXECUTE FUNCTION registration_all_graded();

