/*
 * @file   tasks.sql
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Lab01 tasks.
 * @date   2022-11-21
 *
 * @copyright Copyright (c) 2022
 */

-- 01. Create tables see tables.sql
-- 02. Insert data see datas.sql

-- 03. List (select) id and full name of the students (in ascending order) whose
--     first names start with "m" and are from "malmö".
SELECT id, firstname, lastname FROM student
WHERE firstname LIKE 'M%' AND city = 'Malmö';

-- 04. List (select) the number of students at each department.
SELECT COUNT(id) as students, department_code FROM student
GROUP BY department_code;

-- 05. Give a list of name of the cities where the students live
--     (do not list NULL cities).
SELECT city FROM student WHERE city IS NOT NULL;

-- 06. List the name of the students who has not taken any course yet.
--   a. Without JOIN
SELECT firstname, lastname FROM student
WHERE id IN (SELECT student_id FROM registration WHERE status = 'NS');
--   b. With JOIN
SELECT firstname, lastname FROM student
INNER JOIN registration ON student.id = registration.student_id
WHERE status = 'NS';

-- 07. List id and full name of the students who have taken a course from
--     "computer science" department (using JOIN).
SELECT id, firstname, lastname, name FROM student
INNER JOIN department ON student.department_code = department.code
WHERE name = 'computer science';

-- 08. List all courses offered by "computer science" department (using JOIN)
SELECT c.name AS course_name, d.name AS department_name FROM course AS c
INNER JOIN department AS d ON c.department_code = d.code
WHERE d.name = 'computer science';

-- 09. List the code and name of all the courses and full name of their
--     responsible teachers.
SELECT code, name, fullname FROM course
INNER JOIN teacher ON course.teacher_id = teacher.id;

-- 10. Update the name of “physic” department to “Math” in department table.
UPDATE department SET name = 'Math' WHERE name = 'physics';

-- 11. Delete all the students whose last name starts with “rob” form the
--     database (What is the result? Is it possible to do that? Why? What is
--     the solution if we want to delete those students from our database?).
DELETE FROM student WHERE lastname LIKE 'Rob%';

--     This can't be run if the table does not have DELETE CASCADE on the
--     foreign key.
