INSERT INTO block_result values (
    1,
    '0xc5aa0fd3fec9360ce942009e297c6d58c49a6b679ec84294b87e310d18f9c34c',
    '{}',
    null,
    null,
    0,
    1,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO rollup_result values (
    1,
    5,
    '0xd90bce2038b722089c11bb734e132d304ed51a903d941d0d4342e5c9b61e2a50',
    '0xa4ee974098af0f19d271f4dbd30b56e61541d000e0bcbb3678398c100c03971f'
) ON CONFLICT DO NOTHING;

INSERT INTO block_result values (
    2,
    '0x7cef71f40787a953e1697757f4ab25865f75e2a40cfb50467d24d66a9359e5c9',
    '{}',
    null,
    null,
    1,
    1,
    1658409526
) ON CONFLICT DO NOTHING;

INSERT INTO rollup_result values (
    2,
    3,
    '0x41978bf9a6e2e799924760ddcb6118aa16751022f4415679d22eeadc71e81b8e',
    '0xd3e7eaa0b0c5b44075d8a8795430350591439562e94351317c671a608267e82b'
) ON CONFLICT DO NOTHING;
