# -*- coding: utf-8 -*-
# ----------------------------------------------------------------------------
# Copyright (c) Spyder Project Contributors
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# ----------------------------------------------------------------------------


"""Python wrapper around syntect library."""

from .syntect import (highlight, load_syntax_folder, load_theme_folder,
                      escape_to_console, Style, Color, FontStyleConst,
                      Syntax, Theme, SyntaxSet, ThemeSet, __version__)


class FontStyle:
    """Font style modifiers"""
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


# Package functions
highlight
load_syntax_folder
load_theme_folder
escape_to_console

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
