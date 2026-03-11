#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Generic, Optional, TypeVar



class SizedValue:
    def get_size(self) -> int:
        ...


V = TypeVar("V", bound=SizedValue)


class AddendedOrderedMap(Generic[V]):
    def __init__(self) -> None:
        ...

    def find(self, key: int) -> Optional[tuple[int, V]]:
        ...

    def find_or_insert(self, key: int, new_value: V) -> tuple[V, bool]:
        ...
