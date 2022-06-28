INSERT INTO l2_blocks values (
    0,
    'verified',
    '0x5665b93dae4f6ed5c0427893a42846462b0ac3b845173781cc3e0d35c3751e50',
    '0xc8d4656f1b33e756cbefff0f1eef54f9f8431856aec2de5985ac88a7953badad',
    0,
    1656294068305608
) ON CONFLICT DO NOTHING;

INSERT INTO l2_blocks values (
    1,
    'verified',
    '0xd90bce2038b722089c11bb734e132d304ed51a903d941d0d4342e5c9b61e2a50',
    '0xa4ee974098af0f19d271f4dbd30b56e61541d000e0bcbb3678398c100c03971f',
    101,
    165629413208841
) ON CONFLICT DO NOTHING;

INSERT INTO l2_blocks values (
    2,
    'committed',
    '0x',
    '0x',
    202,
    1656294151207637
) ON CONFLICT DO NOTHING;

INSERT INTO l2_blocks values (
    3,
    'committed',
    '0x5f0bdcc1b1e244ebc334230af6256c442939c967a0adef629dfe6dfedef50ec3',
    '0x481e721b76a4b3529e8295ecbe9dea92a75972fee0452c73d60f0111e500048a',
    303,
    1656294164796285
) ON CONFLICT DO NOTHING;

INSERT INTO l2_blocks values (
    4,
    'uncommitted',
    '0x11a8b1d0fcf6ad3166e0fab72d3177ae95618e40ec89da7ab0e73926f479b596',
    '0xdc58fcf0a09fd00d5cb34406c79d679368b28a830a8ad74a0e05212673a0344b',
    404,
    165629417725157
) ON CONFLICT DO NOTHING;

INSERT INTO l2_blocks values (
    5,
    'uncommitted',
    '0x870312c09a1e9fad557ff366b62ab749db28c6662e347529465273cb75554aef',
    '0x5fc24d2d4f89d596c98ea2beca774ab1780428446edbb0d9a6edb1acc6607baf',
    505,
    1656294190311673
) ON CONFLICT DO NOTHING;

UPDATE l2_blocks SET status = 'verified' WHERE id = 5;
