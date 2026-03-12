#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT OR Apache-2.0

import addended_ordered_map


class TestValue(addended_ordered_map.SizedValue):
    def __init__(self, key: int, size: int) -> None:
        self.key = key
        self.size = size

    def get_size(self) -> int:
        return self.size

    def __eq__(self, other) -> bool:
        if isinstance(other, TestValue):
            return self.key == other.key and self.size == other.size
        return False


def basic_test():
    test_map = addended_ordered_map.AddendedOrderedMap()

    test_map.find_or_insert(0x100C, TestValue(0x100C, 1))
    test_map.find_or_insert(0x1000, TestValue(0x1000, 4))
    test_map.find_or_insert(0x1004, TestValue(0x1004, 4))

    assert test_map.find(0x1000) == (0x1000, TestValue(0x1000, 4))

    assert test_map.find(0x1002) == (0x1000, TestValue(0x1000, 4))

    assert test_map.find(0x0F00) is None

    assert test_map.find(0x2000) is None

    # assert test_map.find(0x1002) is None

    assert test_map.find(0x1008) is None


basic_test()
