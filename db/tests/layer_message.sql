INSERT INTO layer1_message values (
    1,
    1,
    '0xsender1',
    'target',
    'value',
    'fee',
    1,
    1,
    'calldata',
    '0x9a45a59e53778a984f3a7998a8afa5c499d4932c8334382ae7d3604d63d7f960',
    null,
    1
) ON CONFLICT DO NOTHING;

INSERT INTO layer2_message values (
    2,
    2,
    '0xsender1',
    'target',
    'value',
    'fee',
    2,
    2,
    'calldata',
    '0xc5aa0fd3fec9360ce942009e297c6d58c49a6b679ec84294b87e310d18f9c34c',
    null,
    'proof',
    2
) ON CONFLICT DO NOTHING;

INSERT INTO layer1_message values (
    3,
    3,
    '0xsender1',
    'target',
    'value',
    'fee',
    3,
    3,
    'calldata',
    '0x4979de527727e5e6b468f8e994328e9d221c8079f94305763df14aff15e00ea0',
    '0xf931e942953e1f7804980ea0f12b93510ddeabb5edd35a969658359674be6145',
    3
) ON CONFLICT DO NOTHING;

INSERT INTO layer2_message values (
    4,
    4,
    '0xsender1',
    'target',
    'value',
    'fee',
    4,
    4,
    'calldata',
    '0xd89b0a3ca4387efb30d644775ab52ba4aa95a8fa0386a96bb44544a2c0816b16',
    '0x282fac63e1f362fc147eb657793b3c77fd5d1d07f24b9c0086fb6def3c094d87',
    'proof',
    1
) ON CONFLICT DO NOTHING;

INSERT INTO layer1_message values (
    5,
    5,
    '0xsender1',
    'target',
    'value',
    'fee',
    5,
    5,
    'calldata',
    '0x3810c3bf97a458fe047c51a0d19c6b3e1611b64922167b0ead9a718abac806ad',
    null,
    2
) ON CONFLICT DO NOTHING;

INSERT INTO layer2_message values (
    6,
    6,
    '0xsender1',
    'target',
    'value',
    'fee',
    6,
    6,
    'calldata',
    '0x1711710df1000a7bee02fb340ae906804c7f778bd145fea1710b76c1a3103965',
    null,
    'proof',
    3
) ON CONFLICT DO NOTHING;
