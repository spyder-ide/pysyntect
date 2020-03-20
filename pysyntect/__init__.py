# -*- coding: utf-8 -*-

"""Python wrapper around syntect library."""

from .pysyntect import highlight, Style, Color, FontStyleConst


class FontStyle:
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


highlight
Style
Color
FontStyle
