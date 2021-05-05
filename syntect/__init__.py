# -*- coding: utf-8 -*-
# ----------------------------------------------------------------------------
# Copyright (c) Spyder Project Contributors
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# ----------------------------------------------------------------------------


"""Python wrapper around syntect library."""

# Standard library imports
import os
import os.path as osp

# Local imports
from .syntect import (highlight, load_syntax_folder, load_theme_folder,
                      escape_to_console, escape_to_latex,
                      Style, Color, FontStyleConst,
                      Syntax, Theme, SyntaxSet, ThemeSet, __version__)

HERE = osp.dirname(osp.abspath(__file__))
GRAMMARS = osp.join(HERE, 'grammars')
MIN_GRAMMARS = osp.join(HERE, 'minimal_grammars')


class FontStyle:
    """Font style modifiers"""
    BOLD = FontStyleConst.bold()
    ITALIC = FontStyleConst.italic()
    UNDERLINE = FontStyleConst.underline()


def load_minimum_syntax(use_thread):
    """Load a minimum set of language grammars."""
    return load_syntax_folder(MIN_GRAMMARS, use_thread)


def load_default_syntax(use_thread):
    """Load the default syntax definitions included with pysyntect."""
    return load_syntax_folder(GRAMMARS, use_thread)


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

# Package version
__version__
