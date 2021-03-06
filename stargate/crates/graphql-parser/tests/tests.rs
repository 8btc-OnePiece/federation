// Autogenerated by graphql-parser/build.rs
//
// This file is autogenerated by scanning the tests for *.graphql files.
// To add a sample to the test corpus, don't edit this file. Instead, just
// add a new .graphql file to the tests/ directory.
// The tests are added sorted by name so that different machines building will not yield a git diff.

// Sample names are easier to read with two underscores in them
#![allow(non_snake_case)]

mod helpers;
use graphql_parser::parse_query;
use graphql_parser::parse_schema;
use helpers::*;
use insta::assert_snapshot;
use pretty_assertions::assert_eq;

test!(
    directive,
    include_str!("directive.graphql"),
    include_str!("directive.graphql")
);
test!(
    directive_args,
    include_str!("directive_args.graphql"),
    include_str!("directive_args.graphql")
);
test!(
    directive_descriptions,
    include_str!("directive_descriptions.graphql"),
    include_str!("directive_descriptions.canonical.graphql")
);
test!(
    directive_repeatable,
    include_str!("directive_repeatable.graphql"),
    include_str!("directive_repeatable.graphql")
);
test!(
    empty_union,
    include_str!("empty_union.graphql"),
    include_str!("empty_union.graphql")
);
test!(
    enums,
    include_str!("enums.graphql"),
    include_str!("enums.graphql")
);
test!(
    extend_enum,
    include_str!("extend_enum.graphql"),
    include_str!("extend_enum.graphql")
);
test!(
    extend_input,
    include_str!("extend_input.graphql"),
    include_str!("extend_input.canonical.graphql")
);
test!(
    extend_interface,
    include_str!("extend_interface.graphql"),
    include_str!("extend_interface.graphql")
);
test!(
    extend_object,
    include_str!("extend_object.graphql"),
    include_str!("extend_object.graphql")
);
test!(
    extend_scalar,
    include_str!("extend_scalar.graphql"),
    include_str!("extend_scalar.graphql")
);
test!(
    fail_bad_args,
    include_str!("fail_bad_args.graphql"),
    include_str!("fail_bad_args.graphql")
);
test!(
    fail_onion,
    include_str!("fail_onion.graphql"),
    include_str!("fail_onion.graphql")
);
test!(
    fail_querry,
    include_str!("fail_querry.graphql"),
    include_str!("fail_querry.graphql")
);
test!(
    fragment,
    include_str!("fragment.graphql"),
    include_str!("fragment.graphql")
);
test!(
    fragment_spread,
    include_str!("fragment_spread.graphql"),
    include_str!("fragment_spread.graphql")
);
test!(
    implements,
    include_str!("implements.graphql"),
    include_str!("implements.graphql")
);
test!(
    implements_amp,
    include_str!("implements_amp.graphql"),
    include_str!("implements_amp.canonical.graphql")
);
test!(
    inline_fragment,
    include_str!("inline_fragment.graphql"),
    include_str!("inline_fragment.graphql")
);
test!(
    inline_fragment_dir,
    include_str!("inline_fragment_dir.graphql"),
    include_str!("inline_fragment_dir.graphql")
);
test!(
    input_type,
    include_str!("input_type.graphql"),
    include_str!("input_type.graphql")
);
test!(
    interface,
    include_str!("interface.graphql"),
    include_str!("interface.graphql")
);
test!(
    minimal_mutation,
    include_str!("minimal_mutation.graphql"),
    include_str!("minimal_mutation.graphql")
);
test!(
    minimal_query,
    include_str!("minimal_query.graphql"),
    include_str!("minimal_query.graphql")
);
test!(
    minimal_schema,
    include_str!("minimal_schema.graphql"),
    include_str!("minimal_schema.graphql")
);
test!(
    minimal_type,
    include_str!("minimal_type.graphql"),
    include_str!("minimal_type.graphql")
);
test!(
    mutation_directive,
    include_str!("mutation_directive.graphql"),
    include_str!("mutation_directive.graphql")
);
test!(
    named_query,
    include_str!("named_query.graphql"),
    include_str!("named_query.graphql")
);
test!(
    nested_selection,
    include_str!("nested_selection.graphql"),
    include_str!("nested_selection.graphql")
);
test!(
    query_aliases,
    include_str!("query_aliases.graphql"),
    include_str!("query_aliases.graphql")
);
test!(
    query_arguments,
    include_str!("query_arguments.graphql"),
    include_str!("query_arguments.graphql")
);
test!(
    query_directive,
    include_str!("query_directive.graphql"),
    include_str!("query_directive.graphql")
);
test!(
    query_kitchen_sink,
    include_str!("query_kitchen_sink.graphql"),
    include_str!("query_kitchen_sink.canonical.graphql")
);
test!(
    query_list_argument,
    include_str!("query_list_argument.graphql"),
    include_str!("query_list_argument.graphql")
);
test!(
    query_object_argument,
    include_str!("query_object_argument.graphql"),
    include_str!("query_object_argument.graphql")
);
test!(
    query_var_default_float,
    include_str!("query_var_default_float.graphql"),
    include_str!("query_var_default_float.graphql")
);
test!(
    query_var_default_list,
    include_str!("query_var_default_list.graphql"),
    include_str!("query_var_default_list.graphql")
);
test!(
    query_var_default_object,
    include_str!("query_var_default_object.graphql"),
    include_str!("query_var_default_object.graphql")
);
test!(
    query_var_default_string,
    include_str!("query_var_default_string.graphql"),
    include_str!("query_var_default_string.graphql")
);
test!(
    query_var_defaults,
    include_str!("query_var_defaults.graphql"),
    include_str!("query_var_defaults.graphql")
);
test!(
    query_vars,
    include_str!("query_vars.graphql"),
    include_str!("query_vars.graphql")
);
test!(
    scalar_type,
    include_str!("scalar_type.graphql"),
    include_str!("scalar_type.graphql")
);
test!(
    schema_kitchen_sink,
    include_str!("schema_kitchen_sink.graphql"),
    include_str!("schema_kitchen_sink.canonical.graphql")
);
test!(
    simple_object,
    include_str!("simple_object.graphql"),
    include_str!("simple_object.graphql")
);
test!(
    string_literal,
    include_str!("string_literal.graphql"),
    include_str!("string_literal.graphql")
);
test!(
    subscription_directive,
    include_str!("subscription_directive.graphql"),
    include_str!("subscription_directive.graphql")
);
test!(
    triple_quoted_literal,
    include_str!("triple_quoted_literal.graphql"),
    include_str!("triple_quoted_literal.graphql")
);
test!(
    union,
    include_str!("union.graphql"),
    include_str!("union.graphql")
);
test!(
    union_extension,
    include_str!("union_extension.graphql"),
    include_str!("union_extension.graphql")
);
test!(
    very_minimal_query,
    include_str!("very_minimal_query.graphql"),
    include_str!("very_minimal_query.graphql")
);
