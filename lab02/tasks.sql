/*
 * @file   tasks.sql
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Lab01 tasks.
 * @date   2022-12-03
 *
 * @copyright Copyright (c) 2022
 */

-- Join, Grouping and View

-- 01. Show id and full name of the students and the code of their registered courses
SELECT s.firstname, s.lastname, s.id, r.course_code
FROM student AS s
INNER JOIN registration AS r ON s.id = r.student_id;

-- SELECT s.firstname, s.lastname, s.id, COUNT(r.course_code)
-- FROM student AS s
-- INNER JOIN registration AS r ON s.id = r.student_id
-- GROUP BY s.firstname, s.lastname, s.id;

-- 2. Show id and full name of the students and the name of their registered courses.
SELECT s.firstname, s.lastname, s.id, c.name AS course_name
FROM student AS s
INNER JOIN registration AS r ON s.id = r.student_id
INNER JOIN course AS c ON r.course_code = c.code;

-- 3. Show id and full name of the students and name of their current/old teachers.
SELECT s.firstname, s.lastname, s.id, t.fullname AS teacher_name
FROM student AS s
INNER JOIN registration AS r ON s.id = r.student_id
INNER JOIN course AS c ON r.course_code = c.code
INNER JOIN teacher AS t ON t.id = c.teacher_id;

-- 4. Show id and full name of the students, and number of teachers they have had any
--    course with them. Be careful that if student X is/was taking two courses which are
--    given by the same teacher Y, that student has only one teacher.
SELECT s.firstname, s.lastname, s.id, COUNT(DISTINCT c.teacher_id) AS number_of_teachers
FROM student AS s
INNER JOIN registration AS r ON s.id = r.student_id
INNER JOIN course AS c on c.code = r.course_code
INNER JOIN teacher AS t on t.id = c.teacher_id 
GROUP BY s.firstname, s.lastname, s.id;

-- 5. It is quite usual to make a report of the courses and the number of registered
--    students on each course in each year. Thus, it is good to create a view for that.
--    Please write a suitable sql command to create a view for that report. 

CREATE VIEW course_enrollment AS 
    SELECT c.name, COUNT(DISTINCT r.student_id) AS enrollments
    FROM course AS c 
    INNER JOIN registration AS r ON c.code = r.course_code
    GROUP BY c.name;

-- Use view with SELECT query
SELECT * FROM course_enrollment;

-- 6. Show a list of course name and number of students registered for that course in
--    year 2020. Is it better to use your created View in exercise 5 or writing a new Join
--    command?

-- Fix view to include year in the data column
CREATE VIEW course_enrollment_year AS 
    SELECT c.name, COUNT(DISTINCT r.student_id) AS enrollments, EXTRACT(YEAR FROM r.date) as year
    FROM course AS c 
    INNER JOIN registration AS r ON c.code = r.course_code
    GROUP BY c.name, year;

-- Usage: SELECT * FROM course_enrollment_year WHERE year = '2020';

SELECT c.name, COUNT(DISTINCT r.student_id) AS enrollments, EXTRACT(YEAR FROM r.date) as year
FROM course AS c
INNER JOIN registration AS r ON c.code = r.course_code
WHERE EXTRACT(YEAR FROM r.date) = '2020'
GROUP BY c.name, year;

-- Answer: If we were to use the previous view in exercise 5 it would not work
--         because it does not save the year in the data column. But a fix for it is by
--         adding it in the column name and extract only the year and also group it together with the name.

-- Procedures

-- 7. Here, we want you to write a store procedure to receive the first name and last
--    name of a student as input parameters and show the department of the student.
--    For example:

--    if first name = 'Carlos' and last name = 'Manuel', the procedure will return
--    | Department_Name |
--    | --------------- |
--    | Math            |

-- SELECT d.name AS department_name FROM student AS s
-- INNER JOIN department AS d ON s.department_code = d.code
-- WHERE s.firstname = 'Carlos' AND s.lastname = 'Manuel';

CREATE OR REPLACE FUNCTION get_department(fname VARCHAR, lname VARCHAR)
RETURNS TABLE (name VARCHAR)
LANGUAGE plpgsql
AS $$
-- DECLARE
-- VARIABLE DECLARATION
BEGIN
-- stored procedure body
    RETURN query
    SELECT d.name AS department_name FROM student AS s
    INNER JOIN department AS d ON s.department_code = d.code
    WHERE (s.firstname = fname) AND (s.lastname = lname);
END; $$

-- Note: Booby trap 'procedure' word confuses use that we need to create a procedure instead of function.

-- Usage: select * from get_department('Carlos', 'Manuel')

-- 8. Alter your store procedure in exercise 7 to make it more flexible: To receive the
--    first name, last name as input parameters, and show the department of all the
--    students who have similar first name and last name. For example:

--    if first name = 'carl' and last name = '', the procedure will return
--    | FirstName | LastName | DepartmentName |
--    | --------- | -------- | -------------- |
--    | Carlos    | Manuel   | Math           |

--    if first name = '' and last name = '', the procedure will return
--    | FirstName | LastName    | DepartmentName   |
--    | --------- | ----------- | ---------------- |
--    | Michale   | Robbin      | computer science |
--    | Carlos    | Manuel      | Math             |
--    | Enric     | Sitaraman   | computer science |
--    | Joseph    | Dosni       | Math             |
--    | Mario     | Robbin      | computer science |

SELECT s.firstname, s.lastname, d.name AS department_name FROM student AS s
INNER JOIN department AS d ON s.department_code = d.code
WHERE LOWER(s.firstname) LIKE LOWER('%%') AND LOWER(s.lastname) LIKE LOWER('%rob%');

CREATE OR REPLACE FUNCTION get_department(fname VARCHAR, lname VARCHAR)
RETURNS TABLE (fistname VARCHAR, lastname VARCHAR, department_name VARCHAR)
LANGUAGE plpgsql
AS $$
-- DECLARE
-- VARIABLE DECLARATION
BEGIN
-- stored procedure body
    RETURN query
    SELECT s.firstname, s.lastname, d.name AS department_name FROM student AS s
    INNER JOIN department AS d ON s.department_code = d.code
    WHERE LOWER(s.firstname) LIKE LOWER('%'||fname||'%') AND LOWER(s.lastname) LIKE LOWER('%'||lname||'%');
END; $$

-- Usage: select * from get_department('', '');

-- 9. Please check and be sure if your store procedures work successfully by executing
--    them!

-- They do smh and it's a store function not a fukken procedure
