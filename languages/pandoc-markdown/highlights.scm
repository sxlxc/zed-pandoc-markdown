(atx_heading
  heading_content: (inline) @title @title.markup)

(setext_heading
  heading_content: (paragraph) @title @title.markup)

[
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
  (setext_h1_underline)
  (setext_h2_underline)
  (thematic_break)
] @punctuation.special @punctuation.markup

[
  (link_title)
  (indented_code_block)
] @text.literal @text.literal.markup

(fenced_code_block_delimiter) @punctuation.delimiter @punctuation.embedded.markup

(info_string
  (language) @label @label.markup)

(link_destination) @link_uri @link_uri.markup

(link_label) @link_text @link_text.markup

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
  (task_list_marker_checked)
  (task_list_marker_unchecked)
] @punctuation.list_marker @punctuation.list_marker.markup

[
  (block_continuation)
  (block_quote_marker)
] @punctuation.special @punctuation.markup

[
  (backslash_escape)
  (entity_reference)
  (numeric_character_reference)
] @string.escape

(pipe_table_header
  "|" @punctuation.delimiter @punctuation.markup)

(pipe_table_row
  "|" @punctuation.delimiter @punctuation.markup)

(pipe_table_delimiter_row
  "|" @punctuation.delimiter @punctuation.markup)

(pipe_table_delimiter_cell
  "-" @punctuation.delimiter @punctuation.markup)

[
  (pipe_table_align_left)
  (pipe_table_align_right)
] @punctuation.special @punctuation.markup
