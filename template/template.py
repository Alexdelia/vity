"""Module docstring"""

from typing import Any

# comment


class Foo:
    """Foo class docstring"""

    def __init__(self) -> None:
        self.x = 0

    async def dummy(self, arg: int) -> dict[str, Any]:
        """dummy method docstring"""
        return {"arg": arg}

    def _private(self) -> None:
        """private method docstring"""

        i: int = 0

        while i < 10:
            print(i)
            i += 1
