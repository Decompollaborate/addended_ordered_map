#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Callable, Iterator, Generic, Optional, TypeVar

K = TypeVar("K")
V = TypeVar("V", bound=SizedValue)
S = TypeVar("S")


class FindSettings:
    def __init__(
        self,
        allow_addend: bool,
    ) -> None: ...

class SizedValue(Generic[S]):
    def get_size(self) -> S: ...


class AddendedOrderedMap(Generic[K, V, S]):
    def __init__(self) -> None: ...

    def len(self) -> int: ...
    def is_empty(self) -> bool: ...

    def find(
        self,
        key: K,
        settings: FindSettings = FindSettings(True),
    ) -> Optional[tuple[K, V]]: ...

    def find_key(
        self,
        key: K,
        settings: FindSettings = FindSettings(True),
    ) -> Optional[K]: ...

    def find_value(
        self,
        key: K,
        settings: FindSettings = FindSettings(True),
    ) -> Optional[V]: ...

    def find_left_of(
        self,
        key: K,
        inclusive: bool = False
    ) -> Optional[tuple[K, V]]: ...

    def find_right_of(
        self,
        key: K,
        inclusive: bool = False
    ) -> Optional[tuple[K, V]]: ...

    def find_or_insert(
        self,
        key: K,
        new_value: V,
        settings: FindSettings = FindSettings(True),
    ) -> tuple[V, bool]: ...

    def find_or_insert_with(
        self,
        key: K,
        new_default: Callable[[], V],
        settings: FindSettings = FindSettings(True),
    ) -> tuple[V, bool]: ...

    def contains_key_exact(
        self,
        key: K,
    ) -> bool: ...

    def pop_exact(
        self,
        key: K,
    ) -> Optional[tuple[K, V]]: ...

    def pop_range(
        self,
        left: Optional[K],
        right: Optional[K],
    ) -> list[tuple[K, V]]: ...

    def clear(self) -> None: ...

    def __iter__(self) -> Iterator[tuple[K, V]]: ...

    def range(
        self,
        left: Optional[K],
        right: Optional[K],
    ) -> Iterator[tuple[K, V]]: ...

    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
