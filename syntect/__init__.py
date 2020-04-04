# -*- coding: utf-8 -*-

"""Python wrapper around syntect library."""

from .syntect import (highlight, load_syntax_folder, load_theme_folder,
                      Style, Color, FontStyleConst, Syntax, Theme,
                      SyntaxSet, ThemeSet, __version__)


class FontStyle:
    """Font style modifiers"""
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


# Package functions
highlight
load_syntax_folder
load_theme_folder

# Package classes
Style
Color
FontStyle
Syntax
Theme
SyntaxSet
ThemeSet

# Package version
__version__
