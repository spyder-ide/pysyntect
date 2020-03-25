
from typing import List, Tuple


class SyntaxSetHandle:
    languages: List[str] = ...


class Color:
    r: int = ...
    g: int = ...
    b: int = ...
    a: int = ...


class Style:
    foreground: Color = ...
    background: Color = ...
    font_style: int = ...


def load_syntax_folder(path: str) -> SyntaxSetHandle: ...
def highlight(text: str, language: str) -> List[Tuple[Style, str]]: ...