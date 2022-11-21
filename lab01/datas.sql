/*
 * @file   datas.sql
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Insert data into tables in correct order.
 * @date   2022-11-21
 *
 * @copyright Copyright (c) 2022
 */
INSERT INTO department VALUES
(1, 'computer science'),
(2, 'physics'         );
(3, 'music'           );

INSERT INTO teacher VALUES
(138, 'Maria Foster', 1),
(221, 'George Mardy', 2);

INSERT INTO course VALUES
('CAP21', 'Programming'   , 138, 1),
('CAP33', 'Database'      , 138, 1),
('PIS32', 'Thermodynamics', 221, 2);

INSERT INTO student VALUES
(100, 'Michale' , 'Robbin'   , 'Malmö', 1, '0731298058'),
(101, 'Carlos'  , 'Manuel'   ,  null  , 2, null        ),
(102, 'Enrique' , 'Sitaraman', 'Malmö', 1, '0731298959'),
(103, 'Joseph'  , 'Dosni'    , 'Lund' , 2, '0731298957'),
(104, 'Mario'   , 'Robbin'   , 'Tokyo', 1, null        );
(105, 'Tokyo'   , 'Tanaka'   , 'Tokyo', 3, null        );
(106, 'Jean-Ken', 'Johanny'  , 'Tokyo', 3, null        );
(107, 'Kamikaze', 'Boy'      , 'Tokyo', 3, null        );
(108, 'DJ Santa', 'Monica'   , 'Tokyo', 3, null        );
(109, 'Spear'   , 'Rib'      , 'Tokyo', 3, null        );
(110, 'Takuma'  , 'Mitamura' , 'Kyoto', 3, null        );

INSERT INTO registration VALUES
(12, 102, 'PIS32', '2020-02-06', 'VG'),
(13, 104, 'CAP21', '2020-02-06', 'G' ),
(14, 100, 'CAP21', '2020-11-18', 'NS'),
(15, 102, 'CAP21', '2020-11-18', 'NS'),
(16, 103, 'PIS32', '2020-11-18', 'NS');
