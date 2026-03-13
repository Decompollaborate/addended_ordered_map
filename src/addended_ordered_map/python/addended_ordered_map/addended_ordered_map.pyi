#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Callable, Generator, Generic, Optional, TypeVar

class FindSettings:
    def __init__(
        self,
        allow_addend: bool,
    ) -> None: ...

class SizedValue:
    def get_size(self) -> int: ...

V = TypeVar("V", bound=SizedValue)

class AddendedOrderedMap(Generic[V]):
    def __init__(self) -> None: ...

    def len(self) -> int: ...
    def is_empty(self) -> bool: ...

    def find(
        self,
        key: int,
        settings: FindSettings = FindSettings(True),
    ) -> Optional[tuple[int, V]]: ...

    def find_key(
        self,
        key: int,
        settings: FindSettings = FindSettings(True),
    ) -> Optional[int]: ...

    def find_value(
        self,
        key: int,
        settings: FindSettings = FindSettings(True),
    ) -> Optional[V]: ...

    def find_or_insert(
        self,
        key: int,
        new_value: V,
        settings: FindSettings = FindSettings(True),
    ) -> tuple[V, bool]: ...

    def find_or_insert_with(
        self,
        key: int,
        new_default: Callable[[], V],
        settings: FindSettings = FindSettings(True),
    ) -> tuple[V, bool]: ...

    def clear(self) -> None: ...

    def __iter__(self) -> Generator[tuple[int, V], None, None]: ...
