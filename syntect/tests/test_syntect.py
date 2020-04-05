# -*- coding: utf-8 -*-
# ----------------------------------------------------------------------------
# Copyright (c) Spyder Project Contributors
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# ----------------------------------------------------------------------------

"""Tests for pysyntect."""

# Standard library imports
import os
import os.path as osp
import json
import itertools

# PyTest imports
import pytest

# Local imports
from syntect import (highlight, load_theme_folder, load_syntax_folder)


HERE = osp.dirname(__file__)
GRAMMARS = osp.abspath(osp.join(HERE, '..', 'grammars'))
THEMES = osp.abspath(osp.join(HERE, '..', '..', 'themes'))
TEST_FILES = osp.abspath(osp.join(HERE, 'assets'))
RESULT_FILE = osp.join(TEST_FILES, 'results.json')

available_files = os.listdir(TEST_FILES)
available_extensions = [osp.splitext(f)[-1] for f in available_files
                        if osp.splitext(f)[-1] != '.json']
expected_results = json.load(open(RESULT_FILE, 'r'))
available_themes = list(expected_results.keys())
combinations = itertools.product(available_themes, available_extensions)


@pytest.fixture(scope='module')
def syntect_base():
    themes = load_theme_folder(THEMES)
    syntax = load_syntax_folder(GRAMMARS)
    return themes, syntax


@pytest.fixture
def expected_fixture():
    with open(RESULT_FILE, 'r') as f:
        expected = json.load(f)
    return expected


@pytest.fixture
def theme_extension_fixture(expected_fixture, request):
    theme, extension = request.param
    extension = extension[1:]
    path = osp.join(TEST_FILES, 'test.{0}'.format(extension))
    with open(path, 'r') as f:
        contents = f.read()
    return theme, extension, contents, expected_fixture[theme][extension]


@pytest.mark.parametrize('theme_extension_fixture', combinations,
                         indirect=True)
def test_highlight(syntect_base, theme_extension_fixture):
    def map_colors(style):
        components = ['r', 'g', 'b', 'a']
        foreground, background = style.foreground, style.background
        foreground_colors = [getattr(foreground, x) for x in components]
        background_colors = [getattr(background, x) for x in components]
        return foreground_colors + background_colors

    themes, syntax = syntect_base
    theme, extension, text, expected_result = theme_extension_fixture
    styles = highlight(text, extension, syntax, themes[theme])
    test_result = [map_colors(s[0]) for s in styles]
    assert test_result == expected_result
