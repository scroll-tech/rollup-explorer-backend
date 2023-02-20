delete from block_batch;
delete from block_trace;

/* block_batch */

INSERT INTO block_batch values (
    'batch-5',
    5,
    9,
    '0x1ed1792ef028758437ca3c1056ef933d1c3b30c23ace67b0fe38e750175f15d2',
    10,
    '0x52dd8c4568daa7df8a5fbffe7cba4def97409ab2bfec47a4b66c140d1fb61162',
    '0x1911710df1000a7bee02fb340ae906804c7f778bd145fea1710b76c1a3103965',
    '',
    2,
    0,
    0,
    0
) ON CONFLICT DO NOTHING;

INSERT INTO block_batch values (
    'batch-4',
    4,
    5,
    '0x644086aefa61b70e3516fd8365a3ec577592a5771c72280e32dc52fc8c4064ed',
    8,
    '0xf60e145b7eecab24830224dda3770e7f284b6f4f8d750f4a1595402906af699f',
    '0xa9dd95aee224402d4abaeac142337450afe897299927b9336d5785e8c6133587',
    '',
    10,
    0,
    10,
    10,
    null,
    null,
    0,
    10,
    '0x7077c274526c182ebb304f7e904d7547e7557083283d22334e2b827732adc227',
    '0xaa096608f4144b225fe1c0fe8867f0a6e52e428454648674dcb16c5ac8c0a72a',
    1,
    null,
    current_timestamp,
    current_timestamp,
    current_timestamp,
    current_timestamp,
    current_timestamp
) ON CONFLICT DO NOTHING;

INSERT INTO block_batch values (
    'batch-3',
    3,
    4,
    '0xb2c4670936ebfb12fcf7c390bcf1040a381f4f992b13d7d32879d756d54a8627',
    4,
    '0xb2c4670936ebfb12fcf7c390bcf1040a381f4f992b13d7d32879d756d54a8627',
    '0x98d7c2dee6fa1cf200262a22882df35ae55e3cf78ecd554fce1d12ea12c1552c',
    '',
    1,
    0,
    0
) ON CONFLICT DO NOTHING;

INSERT INTO block_batch values (
    'batch-2',
    2,
    3,
    '0x41978bf9a6e2e799924760ddcb6118aa16751022f4415679d22eeadc71e81b8e',
    3,
    '0x41978bf9a6e2e799924760ddcb6118aa16751022f4415679d22eeadc71e81b8e',
    '0xd3e7eaa0b0c5b44075d8a8795430350591439562e94351317c671a608267e82b',
    '',
    20,
    0,
    20,
    1,
    null,
    null,
    0,
    5,
    '0x89e151dcbe42a96adb65c9858e853db31f49036dde754c117ed04842036d155d',
    '0x5e3212126060408466deacaef32a476478d4cfc7025b823e4d450cc3b60ad77b',
    1,
    null,
    current_timestamp,
    current_timestamp,
    current_timestamp,
    current_timestamp,
    current_timestamp
) ON CONFLICT DO NOTHING;

INSERT INTO block_batch values (
    'batch-1',
    1,
    1,
    '0xc5aa0fd3fec9360ce942009e297c6d58c49a6b679ec84294b87e310d18f9c34c',
    2,
    '0xd90bce2038b722089c11bb734e132d304ed51a903d941d0d4342e5c9b61e2a50',
    '0xa4ee974098af0f19d271f4dbd30b56e61541d000e0bcbb3678398c100c03971f',
    '',
    2,
    0,
    0
) ON CONFLICT DO NOTHING;

/* block_trace */

INSERT INTO block_trace values (
    10,
    '0x52dd8c4568daa7df8a5fbffe7cba4def97409ab2bfec47a4b66c140d1fb61162',
    '0x52dd8c4568daa7df8a5fbffe7cba4def97409ab2bfec47a4b66c140d1fb61162',
    '{}',
    'batch-5',
    10,
    10,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    9,
    '0x1ed1792ef028758437ca3c1056ef933d1c3b30c23ace67b0fe38e750175f15d2',
    '0x1ed1792ef028758437ca3c1056ef933d1c3b30c23ace67b0fe38e750175f15d2',
    '{}',
    'batch-5',
    9,
    9,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    8,
    '0xf60e145b7eecab24830224dda3770e7f284b6f4f8d750f4a1595402906af699f',
    '0xf60e145b7eecab24830224dda3770e7f284b6f4f8d750f4a1595402906af699f',
    '{}',
    'batch-4',
    8,
    8,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    7,
    '0xc6c95b2153983b78f15c76e3e603747b9bb6b7c9bb37cabc9418993018e60d05',
    '0xc6c95b2153983b78f15c76e3e603747b9bb6b7c9bb37cabc9418993018e60d05',
    '{}',
    'batch-4',
    7,
    7,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    6,
    '0x9713976e793698f802004a280e7b446bf1726b48169d14c9fc2306be2401bcc5',
    '0x9713976e793698f802004a280e7b446bf1726b48169d14c9fc2306be2401bcc5',
    '{}',
    'batch-4',
    6,
    6,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    5,
    '0x644086aefa61b70e3516fd8365a3ec577592a5771c72280e32dc52fc8c4064ed',
    '0x644086aefa61b70e3516fd8365a3ec577592a5771c72280e32dc52fc8c4064ed',
    '{}',
    'batch-4',
    5,
    5,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    4,
    '0xb2c4670936ebfb12fcf7c390bcf1040a381f4f992b13d7d32879d756d54a8627',
    '0xb2c4670936ebfb12fcf7c390bcf1040a381f4f992b13d7d32879d756d54a8627',
    '{}',
    'batch-3',
    4,
    4,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    3,
    '0x41978bf9a6e2e799924760ddcb6118aa16751022f4415679d22eeadc71e81b8e',
    '0x41978bf9a6e2e799924760ddcb6118aa16751022f4415679d22eeadc71e81b8e',
    '{}',
    'batch-2',
    30,
    30,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    2,
    '0xd90bce2038b722089c11bb734e132d304ed51a903d941d0d4342e5c9b61e2a50',
    '0xd90bce2038b722089c11bb734e132d304ed51a903d941d0d4342e5c9b61e2a50',
    '{}',
    'batch-1',
    2,
    2,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO block_trace values (
    1,
    '0xc5aa0fd3fec9360ce942009e297c6d58c49a6b679ec84294b87e310d18f9c34c',
    '0xc5aa0fd3fec9360ce942009e297c6d58c49a6b679ec84294b87e310d18f9c34c',
    '{}',
    'batch-1',
    1,
    1,
    1658409526
) ON CONFLICT DO NOTHING;
