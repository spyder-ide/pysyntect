
from typing import List, Tuple


class Color:
    """Class describing an RGBA color."""
    r: int = ...
    g: int = ...
    b: int = ...
    a: int = ...


class Syntax:
    """Handle to a language syntax definition."""
    name: str
    language: str


class SyntaxSet:
    """Handle to a list of language syntax definitions."""

    languages: List[str] = ...

    def find_syntax_by_extension(self, extension: str) -> Syntax:
        """
        Find language syntax descriptor.

        Find and return the syntax definition for a language, given
        its file extension.

        Parameters
        ----------
        extension: str
            Extension name of the language to lookup.

        Returns
        -------
        syntax: Syntax
            Handle to the language syntax definition.

        Raises
        ------
        SyntaxNotFoundError
            If the provided language file extension does not have a known
            syntax definition on this set.
        """
        ...


class Theme:
    """
    Handle to a theme descriptor.

    This class points and refers to the main colors and settings defined by
    the theme.
    """
    name: str
    background: Color
    foreground: Color
    caret: Color
    line_highlight: Color
    misspelling: Color
    accent: Color
    bracket_contents_foreground: Color
    brackets_foreground: Color
    brackets_background: Color
    tags_foreground: Color
    highlight: Color
    find_highlight: Color
    find_highlight_foreground: Color
    gutter: Color
    gutter_foreground: Color
    selection: Color
    selection_foreground: Color
    selection_border: Color
    inactive_selection: Color
    inactive_selection_foreground: Color
    guide: Color
    active_guide: Color
    stack_guide: Color
    shadow: Color


class ThemeSet:
    """Handle to a map of highlighting themes."""

    themes: List[str]

    def __getitem__(self, theme: str) -> Theme: ...


class Style:
    """Style color description used to highlight a token."""
    foreground: Color = ...
    background: Color = ...
    font_style: int = ...


def load_theme_folder(path: str) -> ThemeSet: ...
def load_syntax_folder(path: str) -> SyntaxSet: ...
def load_default_syntax() -> SyntaxSet: ...
def highlight(text: str, language: str, syntax_set: SyntaxSet,
              theme: Theme) -> List[Tuple[Style, str]]: ...
def escape_to_console(all_ranges: List[Tuple[Style, str]],
                      display_bg: bool) -> None: ...
