CREATE TABLE department(
    code INT NOT NULL UNIQUE,
    name VARCHAR(30) NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE teacher (
    id INT NOT NULL UNIQUE,
    fullname VARCHAR(40),
    department_code INT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (department_code) REFERENCES department(code) ON UPDATE CASCADE
);

CREATE TABLE student (
    id INT NOT NULL UNIQUE,
    firstname VARCHAR(10) NOT NULL,
    lastname VARCHAR(30) NOT NULL,
    city VARCHAR(15),
    department_code INT NOT NULL,
    telephone CHAR(10),
    PRIMARY KEY (id),
    FOREIGN KEY (department_code) REFERENCES department(code) ON UPDATE CASCADE
);

CREATE TABLE course (
    code CHAR(5) NOT NULL UNIQUE,
    name VARCHAR(15) NOT NULL,
    teacher_id INT NOT NULL,
    department_code INT NOT NULL,
    PRIMARY KEY (code),
    FOREIGN KEY (teacher_id) REFERENCES teacher(id) ON UPDATE CASCADE,
    FOREIGN KEY (department_code) REFERENCES department(code) ON UPDATE CASCADE
);

CREATE TABLE registration (
    number BIGINT NOT NULL UNIQUE,
    student_id INT NOT NULL,
    course_code CHAR(5) NOT NULL,
    date DATE NOT NULL,
    status VARCHAR(2) NOT NULL,
    PRIMARY KEY (number),
    FOREIGN KEY (student_id) REFERENCES student(id) ON UPDATE CASCADE,
    FOREIGN KEY (course_code) REFERENCES course(code) ON UPDATE CASCADE
);

