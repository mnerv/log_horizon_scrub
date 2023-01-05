INSERT INTO admin
    VALUES
    (Default, 'eric@hopestore.se', 'hello'),
    (Default, 'pratchaya@hopestore.se', 'hello');

CALL insert_supplier(
    1,
    'Electrokit AB',
    'Elektronikkomponenter, byggsatser, mätinstrument och tillbehör.',
    '556667-9683',
    'Västkustvägen 7',
    '211 24',
    'Malmö',
    'Sverige',
    '040-298760'
);

CALL insert_supplier(
    1,
    'Handelsbolag Padel For AlI Malmö',
    'Organisera turneringar',
    '969796-6431',
    'Claesgatan 11',
    '214 26',
    'Malmö',
    'Sverige',
    '040-111111'
);

CALL insert_supplier(
    1,
    'Boosting Service',
    'Boost rank',
    '222222-2222',
    'Humlegatan 4',
    '211 27',
    'Malmö',
    'Sverige',
    '040-222222'
);

INSERT INTO product(supplier_id, name, description, quantity, price)
    VALUES
    (1, 'Raspberry Pi 4 Model B', 'Unobtainium', 100, 9999.99),
    (1, 'BTB16-800CWRG TO-220 800V 16A triac', '', 100, 19.00),
    (1, 'Axelnav med klämfäste 6mm', '', 5, 112.00),

    (2, 'Fix match A', '', 10, 444.00),

    (3, 'Boost Tier 1', 'Boost 1 rank', 10, 100.00),
    (3, 'Boost Tier 2', 'Boost 2 rank', 10, 200.00),
    (3, 'Boost Tier 3', 'Boost 2 rank', 10, 300.00);

