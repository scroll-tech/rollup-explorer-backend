delete from batch;
delete from chunk;
delete from l2_block;

/* batch */

INSERT INTO batch values (
    4,
    'batch-4',
    6,
    'chunk-6',
    6,
    'chunk-6',
    'batch-state-root-4',
    'batch-withdraw-root-4',
    '',
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    5,
    '0x52dd8c4568daa7df8a5fbffe7cba4def97409ab2bfec47a4b66c140d1fb61162',
    CURRENT_TIMESTAMP,
    '0x1911710df1000a7bee02fb340ae906804c7f778bd145fea1710b76c1a3103965',
    CURRENT_TIMESTAMP
) ON CONFLICT DO NOTHING;

INSERT INTO batch values (
    3,
    'batch-3',
    5,
    'chunk-5',
    5,
    'chunk-5',
    'batch-state-root-3',
    'batch-withdraw-root-3',
    '',
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    4,
    '0xf60e145b7eecab24830224dda3770e7f284b6f4f8d750f4a1595402906af699f',
    CURRENT_TIMESTAMP,
    '0xa9dd95aee224402d4abaeac142337450afe897299927b9336d5785e8c6133587',
    CURRENT_TIMESTAMP
) ON CONFLICT DO NOTHING;

INSERT INTO batch values (
    2,
    'batch-2',
    3,
    'chunk-3',
    4,
    'chunk-4',
    'batch-state-root-2',
    'batch-withdraw-root-2',
    '',
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    3,
    '0x7077c274526c182ebb304f7e904d7547e7557083283d22334e2b827732adc227',
    CURRENT_TIMESTAMP,
    '0xaa096608f4144b225fe1c0fe8867f0a6e52e428454648674dcb16c5ac8c0a72a',
    CURRENT_TIMESTAMP
) ON CONFLICT DO NOTHING;

INSERT INTO batch values (
    1,
    'batch-1',
    1,
    'chunk-1',
    2,
    'chunk-2',
    'batch-state-root-1',
    'batch-withdraw-root-1',
    '',
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    2,
    '0xb2c4670936ebfb12fcf7c390bcf1040a381f4f992b13d7d32879d756d54a8627',
    CURRENT_TIMESTAMP,
    '0x98d7c2dee6fa1cf200262a22882df35ae55e3cf78ecd554fce1d12ea12c1552c',
    CURRENT_TIMESTAMP
) ON CONFLICT DO NOTHING;

/* chunk */

INSERT INTO chunk values (
    6,
    'chunk-6',
    10,
    'block-10',
    10,
    'block-10',
    6,
    6,
    6,
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    'batch-4',
    6,
    6,
    6,
    6
) ON CONFLICT DO NOTHING;

INSERT INTO chunk values (
    5,
    'chunk-5',
    9,
    'block-9',
    9,
    'block-9',
    5,
    5,
    5,
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    'batch-3',
    5,
    5,
    5,
    5
) ON CONFLICT DO NOTHING;

INSERT INTO chunk values (
    4,
    'chunk-4',
    7,
    'block-7',
    8,
    'block-8',
    4,
    4,
    4,
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    'batch-2',
    4,
    4,
    4,
    4
) ON CONFLICT DO NOTHING;

INSERT INTO chunk values (
    3,
    'chunk-3',
    5,
    'block-5',
    6,
    'block-6',
    3,
    3,
    3,
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    'batch-2',
    3,
    3,
    3,
    3
) ON CONFLICT DO NOTHING;

INSERT INTO chunk values (
    2,
    'chunk-2',
    3,
    'block-3',
    4,
    'block-4',
    2,
    2,
    2,
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    'batch-1',
    2,
    2,
    2,
    2
) ON CONFLICT DO NOTHING;

INSERT INTO chunk values (
    1,
    'chunk-1',
    1,
    'block-1',
    2,
    'block-2',
    1,
    1,
    1,
    1,
    NULL,
    NULL,
    NULL,
    NULL,
    'batch-1',
    1,
    1,
    1,
    1
) ON CONFLICT DO NOTHING;

/* l2_block */

INSERT INTO l2_block values (
    10,
    'block-10',
    '',
    '',
    '',
    '',
    10,
    10,
    1658409526,
    'chunk-6'
) ON CONFLICT DO NOTHING;

insert into l2_block values (
    9,
    'block-9',
    '',
    '',
    '',
    '',
    9,
    9,
    1658409526,
    'chunk-5'
) on conflict do nothing;

insert into l2_block values (
    8,
    'block-8',
    '',
    '',
    '',
    '',
    8,
    8,
    1658409526,
    'chunk-4'
) on conflict do nothing;

insert into l2_block values (
    7,
    'block-7',
    '',
    '',
    '',
    '',
    7,
    7,
    1658409526,
    'chunk-4'
) on conflict do nothing;

insert into l2_block values (
    6,
    'block-6',
    '',
    '',
    '',
    '',
    6,
    6,
    1658409526,
    'chunk-3'
) on conflict do nothing;

insert into l2_block values (
    5,
    'block-5',
    '',
    '',
    '',
    '',
    5,
    5,
    1658409526,
    'chunk-3'
) on conflict do nothing;

insert into l2_block values (
    4,
    'block-4',
    '',
    '',
    '',
    '',
    4,
    4,
    1658409526,
    'chunk-2'
) on conflict do nothing;

insert into l2_block values (
    3,
    'block-3',
    '',
    '',
    '',
    '',
    3,
    3,
    1658409526,
    'chunk-2'
) on conflict do nothing;

insert into l2_block values (
    2,
    'block-2',
    '',
    '',
    '',
    '',
    2,
    2,
    1658409526,
    'chunk-1'
) on conflict do nothing;

insert into l2_block values (
    1,
    'block-1',
    '',
    '',
    '',
    '',
    1,
    1,
    1658409526,
    'chunk-1'
) on conflict do nothing;
