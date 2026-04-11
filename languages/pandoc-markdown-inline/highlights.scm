[
  (code_span)
  (link_title)
] @text.literal @text.literal.markup

[
  (emphasis_delimiter)
  (code_span_delimiter)
  (latex_span_delimiter)
] @punctuation.delimiter @punctuation.markup

(emphasis) @emphasis @emphasis.markup

(strong_emphasis) @emphasis.strong @emphasis.strong.markup

(strikethrough) @emphasis @strikethrough.markup

[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @link_uri @link_uri.markup

[
  (link_label)
  (link_text)
  (image_description)
] @link_text @link_text.markup

[
  (backslash_escape)
  (hard_line_break)
  (entity_reference)
  (numeric_character_reference)
] @string.escape

(latex_block) @text.literal @embedded

(image
  "!" @punctuation.delimiter @punctuation.markup)

(image
  [
    "["
    "]"
    "("
    ")"
  ] @punctuation.delimiter @punctuation.markup)

(inline_link
  [
    "["
    "]"
    "("
    ")"
  ] @punctuation.delimiter @punctuation.markup)

(shortcut_link
  [
    "["
    "]"
  ] @punctuation.delimiter @punctuation.markup)

(collapsed_reference_link
  [
    "["
    "]"
  ] @punctuation.delimiter @punctuation.markup)

(full_reference_link
  [
    "["
    "]"
  ] @punctuation.delimiter @punctuation.markup)
