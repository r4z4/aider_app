// handlebars_helpers.rs

use handlebars::handlebars_helper;

handlebars_helper!(to_title_case: |s: str| s.to_title_case());

