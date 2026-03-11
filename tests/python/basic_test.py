#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT OR Apache-2.0

import addended_ordered_map


class TestValue(addended_ordered_map.SizedValue):
    def __init__(self, size: int, whatever: int = 5) -> None:
        self.size = size

    def get_size(self) -> int:
        return self.size


def basic_test():
    test_map = addended_ordered_map.AddendedOrderedMap()

    test_map.find_or_insert(0x100C, TestValue(1))
    test_map.find_or_insert(0x1000, TestValue(4))
    test_map.find_or_insert(0x1004, TestValue(4))

    assert (0x1000, TestValue(4)) == test_map.find(0x1000)

    assert (0x1000, TestValue(4)) == test_map.find(0x1002)

    assert None == test_map.find(0x0F00)

    assert None == test_map.find(0x2000)

    # assert None == test_map.find(0x1002)

    assert None == test_map.find(0x1008)


basic_test()
