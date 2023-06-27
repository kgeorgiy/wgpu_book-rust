cargo clippy --examples -- ^
    -W clippy::pedantic ^
    -W clippy::restriction ^
    -A clippy::cast_precision_loss ^
    -A clippy::expect_used ^
    -A clippy::implicit_return ^
    -A clippy::missing_docs_in_private_items ^
    -A clippy::missing_inline_in_public_items ^
    -A clippy::missing_trait_methods ^
    -A clippy::question_mark_used ^
    -A clippy::arithmetic_side_effects ^
    -A clippy::integer_arithmetic ^
    -A clippy::float_arithmetic ^
    -A clippy::as_conversions ^
    -A clippy::pub_use ^
    -A clippy::clone_on_ref_ptr ^
    -A clippy::exhaustive_structs ^
    -A clippy::wildcard_enum_match_arm ^
    -A clippy::std_instead_of_alloc ^
    -A clippy::single_char_lifetime_names ^
    -A clippy::let_underscore_untyped ^
    -A clippy::panic ^
    -A clippy::modulo_arithmetic ^
    -A clippy::self_named_module_files ^
    -A clippy::default_numeric_fallback ^
    -A clippy::print_stdout ^
    -A clippy::blanket_clippy_restriction_lints