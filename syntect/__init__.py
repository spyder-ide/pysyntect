# -*- coding: utf-8 -*-

"""Python wrapper around syntect library."""

from .syntect import (highlight, load_syntax_folder, Style, Color,
                      FontStyleConst, __version__)


class FontStyle:
    """Font style modifiers"""
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


# Package functions
highlight
load_syntax_folder

# Package classes
Style
Color
FontStyle

# Package version
__version__
