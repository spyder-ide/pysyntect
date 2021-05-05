# -*- coding: utf-8 -*-
# ----------------------------------------------------------------------------
# Copyright (c) Spyder Project Contributors
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# ----------------------------------------------------------------------------


"""Python wrapper around syntect library."""

# Standard library imports
import os.path as osp

# Local imports
from .syntect import (highlight, load_syntax_folder, load_theme_folder,
                      escape_to_console, escape_to_latex,
                      Style, Color, FontStyleConst,
                      Syntax, Theme, SyntaxSet, ThemeSet, LoadingError,
                      SyntaxNotFoundError, __version__)

HERE = osp.dirname(osp.abspath(__file__))
GRAMMARS = osp.join(HERE, 'grammars')


class FontStyle:
    """Font style modifiers"""
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


def load_default_syntax() -> SyntaxSet:
    """Load the default syntax definitions included with pysyntect."""
    return load_syntax_folder(GRAMMARS)


# Package functions
highlight
load_syntax_folder
load_theme_folder
escape_to_console
escape_to_latex

# Package classes
Style
Color
FontStyle
Syntax
Theme
SyntaxSet
ThemeSet

# Exception classes
LoadingError
SyntaxNotFoundError

# Package version
__version__
