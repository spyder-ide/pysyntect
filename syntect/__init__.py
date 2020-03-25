# -*- coding: utf-8 -*-

"""Python wrapper around syntect library."""

from .syntect import (highlight, load_syntax_folder, Style, Color,
                      FontStyleConst)


class FontStyle:
    """Font style modifiers"""
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


highlight
Style
Color
FontStyle
load_syntax_folder
