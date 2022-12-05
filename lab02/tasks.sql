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
    SELECT c.name, COUNT(DISTINCT r.student_id ) AS enrollments
    FROM course AS c 
    INNER JOIN registration AS r ON c.code = r.course_code
    GROUP BY c.name;
SELECT * FROM course_enrollment;

-- 6. Show a list of course name and number of students registered for that course in
--    year 2020. Is it better to use your created View in exercise 5 or writing a new Join
--    command?



-- Procedures

-- 7. Here, we want you to write a store procedure to receive the first name and last
--    name of a student as input parameters and show the department of the student.
--    For example:

-- 8. Alter your store procedure in exercise 7 to make it more flexible: To receive the
--    first name, last name as input parameters, and show the department of all the
--    students who have similar first name and last name. For example:

-- 9. Please check and be sure if your store procedures work successfully by executing
--    them!
