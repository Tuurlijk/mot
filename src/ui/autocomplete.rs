use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
};
use std::time::{Duration, Instant};

/// State for an autocomplete dropdown
#[derive(Clone, Debug)]
pub struct AutocompleteState<T: Clone> {
    pub input: String,
    pub last_searched_input: String,
    pub is_dropdown_visible: bool,
    pub min_chars_to_search: usize,
    pub items: Vec<T>,
    pub list_state: ListState,
    pub is_loading: bool,
    pub scroll_offset: usize,
    // Debounce state
    pub last_keystroke: Option<Instant>,
    pub debounce_duration: Duration,
}

impl<T: Clone> Default for AutocompleteState<T> {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            input: String::new(),
            last_searched_input: String::new(),
            is_dropdown_visible: false,
            min_chars_to_search: 3,
            items: Vec::new(),
            list_state,
            is_loading: false,
            scroll_offset: 0,
            // Initialize debounce state
            last_keystroke: None,
            debounce_duration: Duration::from_millis(300), // Default debounce duration
        }
    }
}

impl<T: Clone> AutocompleteState<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn needs_search(&self) -> bool {
        self.input.len() >= self.min_chars_to_search && self.input != self.last_searched_input
    }

    pub fn mark_searched(&mut self) {
        self.last_searched_input = self.input.clone();
        self.is_loading = true;
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.list_state.selected().and_then(|i| self.items.get(i))
    }

    pub fn update_items(&mut self, items: Vec<T>) {
        self.items = items;
        // Always show dropdown if we have items, regardless of previous state
        self.is_dropdown_visible = !self.items.is_empty();
        // Reset selection to first item if we have items
        if !self.items.is_empty() {
            self.list_state.select(Some(0));
        }
        self.is_loading = false;
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
        // Mark as needing search every time input changes
        self.last_searched_input = String::new();
    }

    pub fn remove_char(&mut self) {
        self.input.pop();
        // Mark as needing search every time input changes
        self.last_searched_input = String::new();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.last_searched_input.clear();
        self.is_dropdown_visible = false;
        self.items.clear();
        self.is_loading = false;
    }

    /// Records the time of the last keypress relevant to input modification.
    pub fn record_keypress(&mut self) {
        self.last_keystroke = Some(Instant::now());
    }

    /// Checks if the debounce timeout has been reached since the last keypress
    /// and if the input meets the criteria for triggering a refresh.
    pub fn check_debounce_timeout(&self) -> bool {
        if let Some(last_key_time) = self.last_keystroke {
            let elapsed = Instant::now().duration_since(last_key_time);
            if elapsed >= self.debounce_duration {
                // Check if input has changed and meets criteria (length or empty)
                if self.input != self.last_searched_input
                    && (self.input.len() >= self.min_chars_to_search || self.input.is_empty())
                {
                    return true; // Debounce timed out, input changed, refresh needed
                }
            }
        }
        false // Not timed out, or input hasn't changed meaningfully
    }
}

pub struct Autocomplete<'a, T, F>
where
    T: Clone,
    F: Fn(&T) -> String,
{
    pub state: &'a mut AutocompleteState<T>,
    pub block: Option<Block<'a>>,
    pub input_style: Style,
    pub dropdown_style: Style,
    pub selected_style: Style,
    pub transform_fn: F,
    pub placeholder: Option<&'a str>,
}

impl<'a, T, F> Autocomplete<'a, T, F>
where
    T: Clone,
    F: Fn(&T) -> String,
{
    pub fn new(state: &'a mut AutocompleteState<T>, transform_fn: F) -> Self {
        Self {
            state,
            block: None,
            input_style: Style::default(),
            dropdown_style: Style::default(),
            selected_style: Style::default().add_modifier(Modifier::REVERSED),
            transform_fn,
            placeholder: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn input_style(mut self, style: Style) -> Self {
        self.input_style = style;
        self
    }

    pub fn dropdown_style(mut self, style: Style) -> Self {
        self.dropdown_style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
}

impl<T, F> Widget for Autocomplete<'_, T, F>
where
    T: Clone,
    F: Fn(&T) -> String,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the overall block if specified
        let area = match &self.block {
            Some(b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        // Split the area for input and potential dropdown
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        // Render input value
        let display_text = if self.state.input.is_empty() && self.placeholder.is_some() {
            Paragraph::new(self.placeholder.unwrap()).style(self.input_style)
        } else {
            Paragraph::new(self.state.input.clone()).style(self.input_style)
        };

        display_text.render(chunks[0], buf);

        // Render dropdown if visible
        if self.state.is_dropdown_visible {
            // Calculate max available space
            let max_height = (chunks[1].height).min(10);

            if max_height == 0 {
                return; // No space to show dropdown
            }

            // Calculate position for dropdown - always beneath input field
            let dropdown_area = Rect::new(area.x, chunks[0].y + 1, area.width, max_height);

            // Create Clear widget to ensure dropdown overlays properly
            Clear.render(dropdown_area, buf);

            if self.state.is_loading {
                // Show loading indicator
                let loading = Paragraph::new("Loading...")
                    .style(self.input_style)
                    .block(Block::default().borders(Borders::ALL));
                loading.render(dropdown_area, buf);
            } else if self.state.items.is_empty() {
                // Show "no results" message
                let no_results = Paragraph::new("No matching items found")
                    .style(self.input_style)
                    .block(Block::default().borders(Borders::ALL));
                no_results.render(dropdown_area, buf);
            } else {
                // Adjust scroll offset to keep selected item visible
                if let Some(selected) = self.state.list_state.selected() {
                    if selected >= self.state.scroll_offset + max_height as usize {
                        self.state.scroll_offset = selected.saturating_sub(max_height as usize - 1);
                    } else if selected < self.state.scroll_offset {
                        self.state.scroll_offset = selected;
                    }
                }

                // Create list items from visible items using the transform function
                let visible_items: Vec<ListItem> = self
                    .state
                    .items
                    .iter()
                    .skip(self.state.scroll_offset)
                    .take(max_height as usize)
                    .map(|item| {
                        let display = (self.transform_fn)(item);
                        ListItem::new(display)
                    })
                    .collect();

                // Create and render the list
                let list = List::new(visible_items)
                    .block(Block::default().borders(Borders::ALL))
                    .highlight_style(self.selected_style);

                // Render stateful list directly
                StatefulWidget::render(list, dropdown_area, buf, &mut self.state.list_state);
            }
        }
    }
}
