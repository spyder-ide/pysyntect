/// Class wrapper around [`syntect::highlighting::Theme`] and [`syntect::highlighting::ThemeSet`]

// PyO3 imports
use pyo3::prelude::*;
use pyo3::exceptions::{PyKeyError};
use pyo3::class::PyMappingProtocol;

// Syntect imports
use syntect::highlighting::{Color, Theme, ThemeSet};

// Local imports
use crate::base::ColorWrap;


fn pack_color(color_opt: Option<Color>) -> Option<ColorWrap> {
    match color_opt {
        Some(color) => {
            let wrapped_color = ColorWrap {
                color
            };
            Some(wrapped_color)
        }
        None => None
    }
}

/// Handle to a theme descriptor.
///
/// This class points and refers to the main colors and settings defined by
/// the theme.
#[pyclass(name="Theme")]
#[derive(Debug, Clone)]
pub struct ThemeHandle {
    pub theme: Theme,
}

#[pymethods]
impl ThemeHandle {
    /// Name of the theme.
    #[getter]
    fn name(&self) -> Option<&str> {
        match &self.theme.name {
            Some(x) => Some(x.as_str()),
            None => None
        }
    }

    /// The default backgound color of the view.
    #[getter]
    fn background(&self) -> Option<ColorWrap> {
        let background_color = self.theme.settings.background;
        pack_color(background_color)
    }

    /// The default color for text.
    #[getter]
    fn foreground(&self) -> Option<ColorWrap> {
        let foreground_color = self.theme.settings.foreground;
        pack_color(foreground_color)
    }

    /// Color of the caret.
    #[getter]
    fn caret(&self) -> Option<ColorWrap> {
        let caret_color = self.theme.settings.caret;
        pack_color(caret_color)
    }

    /// Color of the line the caret is in.
    #[getter]
    fn line_highlight(&self) -> Option<ColorWrap> {
        let line_highlight_color = self.theme.settings.line_highlight;
        pack_color(line_highlight_color)
    }

    /// The color to use for the squiggly underline drawn under misspelled words.
    #[getter]
    fn misspelling(&self) -> Option<ColorWrap> {
        let misspelling_color = self.theme.settings.misspelling;
        pack_color(misspelling_color)
    }

    /// A color made available for use by the theme.
    #[getter]
    fn accent(&self) -> Option<ColorWrap> {
        let accent_color = self.theme.settings.accent;
        pack_color(accent_color)
    }

    /// Color of bracketed sections of text when the caret is in a bracketed section.
    #[getter]
    fn bracket_contents_foreground(&self) -> Option<ColorWrap> {
        let bracket_contents_foreground_color = self.theme.settings.bracket_contents_foreground;
        pack_color(bracket_contents_foreground_color)
    }

    /// Foreground color of the brackets when the caret is next to a bracket.
    #[getter]
    fn brackets_foreground(&self) -> Option<ColorWrap> {
        let brackets_foreground_color = self.theme.settings.brackets_foreground;
        pack_color(brackets_foreground_color)
    }

    /// Background color of the brackets when the caret is next to a bracket.
    #[getter]
    fn brackets_background(&self) -> Option<ColorWrap> {
        let brackets_background_color = self.theme.settings.brackets_background;
        pack_color(brackets_background_color)
    }

    /// Color of tags when the caret is next to a tag.
    #[getter]
    fn tags_foreground(&self) -> Option<ColorWrap> {
        let tags_foreground_color = self.theme.settings.tags_foreground;
        pack_color(tags_foreground_color)
    }

    /// The border color for "other" matches.
    #[getter]
    fn highlight(&self) -> Option<ColorWrap> {
        let highlight_color = self.theme.settings.highlight;
        pack_color(highlight_color)
    }

    /// Background color of regions matching the current search.
    #[getter]
    fn find_highlight(&self) -> Option<ColorWrap> {
        let find_highlight_color = self.theme.settings.find_highlight;
        pack_color(find_highlight_color)
    }

    /// Text color of regions matching the current search.
    #[getter]
    fn find_highlight_foreground(&self) -> Option<ColorWrap> {
        let find_highlight_foreground_color = self.theme.settings.find_highlight_foreground;
        pack_color(find_highlight_foreground_color)
    }

    /// Background color of the gutter.
    #[getter]
    fn gutter(&self) -> Option<ColorWrap> {
        let gutter_color = self.theme.settings.gutter;
        pack_color(gutter_color)
    }

    /// Foreground color of the gutter.
    #[getter]
    fn gutter_foreground(&self) -> Option<ColorWrap> {
        let gutter_foreground_color = self.theme.settings.gutter_foreground;
        pack_color(gutter_foreground_color)
    }

    /// The background color of selected text.
    #[getter]
    fn selection(&self) -> Option<ColorWrap> {
        let selection_color = self.theme.settings.selection;
        pack_color(selection_color)
    }

    /// A color that will override the scope-based text color of the selection.
    #[getter]
    fn selection_foreground(&self) -> Option<ColorWrap> {
        let selection_foreground_color = self.theme.settings.selection_foreground;
        pack_color(selection_foreground_color)
    }

    /// Color of the selection regions border.
    #[getter]
    fn selection_border(&self) -> Option<ColorWrap> {
        let selection_border_color = self.theme.settings.selection_border;
        pack_color(selection_border_color)
    }

    /// The background color of a selection in a view that is not currently focused.
    #[getter]
    fn inactive_selection(&self) -> Option<ColorWrap> {
        let inactive_selection_color = self.theme.settings.inactive_selection;
        pack_color(inactive_selection_color)
    }

    /// A color that will override the scope-based text color of the selection
    /// in a view that is not currently focused.
    #[getter]
    fn inactive_selection_foreground(&self) -> Option<ColorWrap> {
        let inactive_selection_foreground_color = self.theme.settings.inactive_selection_foreground;
        pack_color(inactive_selection_foreground_color)
    }

    /// Color of the guides displayed to indicate nesting levels.
    #[getter]
    fn guide(&self) -> Option<ColorWrap> {
        let guide_color = self.theme.settings.guide;
        pack_color(guide_color)
    }

    /// Color of the guide lined up with the caret.
    #[getter]
    fn active_guide(&self) -> Option<ColorWrap> {
        let active_guide_color = self.theme.settings.active_guide;
        pack_color(active_guide_color)
    }

    /// Color of the current guide’s parent guide level.
    #[getter]
    fn stack_guide(&self) -> Option<ColorWrap> {
        let stack_guide_color = self.theme.settings.stack_guide;
        pack_color(stack_guide_color)
    }

    /// The color of the shadow used when a text area can be horizontally scrolled.
    #[getter]
    fn shadow(&self) -> Option<ColorWrap> {
        let shadow_color = self.theme.settings.shadow;
        pack_color(shadow_color)
    }

}

/// Handle to a map of highlighting themes.
#[pyclass(name="ThemeSet")]
pub struct ThemeSetHandle {
    pub theme_set: ThemeSet,
}

#[pyproto]
impl PyMappingProtocol for ThemeSetHandle {
    fn __getitem__(&self, key: String) -> PyResult<ThemeHandle> {
        let opt = self.theme_set.themes.get(&key).cloned();
        match opt {
            Some(v) => {
                let handler = ThemeHandle {
                    theme: v
                };
                Ok(handler)
            },
            None => Err(PyKeyError::new_err(
                format!("Theme {} does not exist", key)
            ))
        }
    }
}

#[pymethods]
impl ThemeSetHandle {
    /// List of theme names included on this map.
    #[getter]
    fn themes(&self) -> Vec<&String> {
        let mut themes = Vec::<&String>::new();
        for key in self.theme_set.themes.keys() {
            themes.push(key)
        }
        themes
    }

    /// Load syntect default theme sets.
    ///
    /// Returns
    /// -------
    /// theme_set: ThemeSet
    ///     Handle to a list of syntax highlighting themes.
    #[staticmethod]
    fn load_defaults() -> ThemeSetHandle {
        let defaults = ThemeSet::load_defaults();
        ThemeSetHandle {
            theme_set: defaults
        }
    }
}
