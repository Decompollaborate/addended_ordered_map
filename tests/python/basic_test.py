#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT OR Apache-2.0

from addended_ordered_map import AddendedOrderedMap, FindSettings, SizedValue


class TestValue(SizedValue):
    def __init__(self, key: int, size: int) -> None:
        self.key = key
        self.size = size

    def get_size(self) -> int:
        return self.size

    def __eq__(self, other) -> bool:
        if isinstance(other, TestValue):
            return self.key == other.key and self.size == other.size
        return False

    def __repr__(self) -> str:
        return f"TestValue({self.key}, {self.size})"


def basic_test():
    test_map: AddendedOrderedMap[int, TestValue, int] = AddendedOrderedMap()

    value_0x100C = TestValue(0x100C, 1)
    value_0x1000 = TestValue(0x1000, 4)
    value_0x1004 = TestValue(0x1004, 4)

    print(test_map)

    test_map.find_or_insert(0x100C, value_0x100C)
    print(test_map)

    test_map.find_or_insert(0x1000, value_0x1000)
    print(test_map)

    test_map.find_or_insert(0x1004, value_0x1004)
    print(test_map)

    val = test_map.find(0x1000)
    assert val == (0x1000, TestValue(0x1000, 4)), val
    assert val is not None
    assert val[1] is value_0x1000

    assert test_map.find(0x1002) == (0x1000, TestValue(0x1000, 4))
    val = test_map.find(0x1002)
    assert val is not None
    assert val[1] is value_0x1000

    assert test_map.find(0x0F00) is None

    assert test_map.find(0x2000) is None

    assert test_map.find(0x1002, FindSettings(False)) is None

    assert test_map.find(0x1008) is None

    assert test_map.find_left_of(0x1004, True) == (0x1004, value_0x1004)
    assert test_map.find_left_of(0x1004, False) == (0x1000, value_0x1000)

    assert test_map.find_right_of(0x1004, True) == (0x1004, value_0x1004)
    assert test_map.find_right_of(0x1004, False) == (0x100C, value_0x100C)

    assert test_map.find_left_of(0x1004, True) == test_map.find_right_of(0x1004, True)

    # Check references work and modifying them also work
    assert test_map.find(0x1010) is None
    value_0x100C.size = 0x10
    val = test_map.find(0x1010)
    assert val == (0x100C, TestValue(0x100C, 0x10)), val
    assert val is not None
    assert val[1] is value_0x100C
    val[1].size = 1
    assert test_map.find(0x1010) is None

    iterable = iter(test_map)
    assert next(iterable) == (0x1000, value_0x1000)
    assert next(iterable) == (0x1004, value_0x1004)
    assert next(iterable) == (0x100C, value_0x100C)
    try:
        next(iterable)
    except Exception as e:
        assert isinstance(e, StopIteration), e

    ranged = test_map.range(0x1002, 0x1006)
    val = next(ranged)
    assert val == (0x1004, value_0x1004)
    assert val[1] is value_0x1004
    try:
        next(iterable)
    except Exception as e:
        assert isinstance(e, StopIteration), e


    assert test_map.contains_key_exact(0x1004)
    val = test_map.pop_exact(0x1004)
    assert val is not None
    assert val == (0x1004, value_0x1004)
    assert val[1] is value_0x1004
    assert not test_map.contains_key_exact(0x1004)
    assert test_map.pop_exact(0x1004) is None

    val_test = test_map.pop_range(0x1008, 0x1010)
    assert val_test == [(0x100C, value_0x100C)], val_test
    val_test = test_map.pop_range(0x1008, 0x1010)
    assert val_test == [], val_test


def basic_test_with():
    test_map: AddendedOrderedMap[int, TestValue, int] = AddendedOrderedMap()

    value_0x100C = TestValue(0x100C, 1)
    value_0x1000 = TestValue(0x1000, 4)
    value_0x1004 = TestValue(0x1004, 4)

    print(test_map)

    test_map.find_or_insert_with(0x100C, lambda: value_0x100C)
    print(test_map)

    test_map.find_or_insert_with(0x1000, lambda: value_0x1000)
    print(test_map)

    test_map.find_or_insert_with(0x1004, lambda: value_0x1004)
    print(test_map)

    val = test_map.find(0x1000)
    assert val == (0x1000, TestValue(0x1000, 4)), val
    assert val is not None
    assert val[1] is value_0x1000

    assert test_map.find(0x1002) == (0x1000, TestValue(0x1000, 4))
    val = test_map.find(0x1002)
    assert val is not None
    assert val[1] is value_0x1000

    assert test_map.find(0x0F00) is None

    assert test_map.find(0x2000) is None

    assert test_map.find(0x1002, FindSettings(False)) is None

    assert test_map.find(0x1008) is None

    assert test_map.find_left_of(0x1004, True) == (0x1004, value_0x1004)
    assert test_map.find_left_of(0x1004, False) == (0x1000, value_0x1000)

    assert test_map.find_right_of(0x1004, True) == (0x1004, value_0x1004)
    assert test_map.find_right_of(0x1004, False) == (0x100C, value_0x100C)

    assert test_map.find_left_of(0x1004, True) == test_map.find_right_of(0x1004, True)

    # Check references work and modifying them also work
    assert test_map.find(0x1010) is None
    value_0x100C.size = 0x10
    val = test_map.find(0x1010)
    assert val == (0x100C, TestValue(0x100C, 0x10)), val
    assert val is not None
    assert val[1] is value_0x100C
    val[1].size = 1
    assert test_map.find(0x1010) is None

    iterable = iter(test_map)
    assert next(iterable) == (0x1000, value_0x1000)
    assert next(iterable) == (0x1004, value_0x1004)
    assert next(iterable) == (0x100C, value_0x100C)
    try:
        next(iterable)
    except Exception as e:
        assert isinstance(e, StopIteration), e

    ranged = test_map.range(0x1002, 0x1006)
    val = next(ranged)
    assert val == (0x1004, value_0x1004)
    assert val[1] is value_0x1004
    try:
        next(iterable)
    except Exception as e:
        assert isinstance(e, StopIteration), e


    assert test_map.contains_key_exact(0x1004)
    val = test_map.pop_exact(0x1004)
    assert val is not None
    assert val == (0x1004, value_0x1004)
    assert val[1] is value_0x1004
    assert not test_map.contains_key_exact(0x1004)
    assert test_map.pop_exact(0x1004) is None

    val_test = test_map.pop_range(0x1008, 0x1010)
    assert val_test == [(0x100C, value_0x100C)], val_test
    val_test = test_map.pop_range(0x1008, 0x1010)
    assert val_test == [], val_test


basic_test()
basic_test_with()

print("OK")
