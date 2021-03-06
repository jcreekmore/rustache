extern crate rustache;

use rustache::{HashBuilder, Render};
use std::io::Cursor;

// - name: Interpolation
//     desc: A lambda's return value should be interpolated.
//     data:
//       lambda: !code
//         ruby:    'proc { "world" }'
//         perl:    'sub { "world" }'
//         js:      'function() { return "world" }'
//         php:     'return "world";'
//         python:  'lambda: "world"'
//         clojure: '(fn [] "world")'
//     template: "Hello, {{lambda}}!"
//     expected: "Hello, world!"
#[test]
fn test_spec_lambdas_interpolation() {
    let mut f = |_| { "world".to_string() };
    let data = HashBuilder::new()
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("Hello, {{lambda}}!", &mut rv).unwrap();

    assert_eq!("Hello, world!".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Interpolation - Expansion
//     desc: A lambda's return value should be parsed.
//     data:
//       planet: "world"
//       lambda: !code
//         ruby:    'proc { "{{planet}}" }'
//         perl:    'sub { "{{planet}}" }'
//         js:      'function() { return "{{planet}}" }'
//         php:     'return "{{planet}}";'
//         python:  'lambda: "{{planet}}"'
//         clojure: '(fn [] "{{planet}}")'
//     template: "Hello, {{lambda}}!"
//     expected: "Hello, world!"
#[test]
fn test_spec_lambdas_interpolation_expansion() {
    let mut f = |_| { "{{planet}}".to_string() };
    let data = HashBuilder::new()
                    .insert("planet", "world")
                    .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("Hello, {{lambda}}!", &mut rv).unwrap();

    assert_eq!("Hello, world!".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Interpolation - Alternate Delimiters
//     desc: A lambda's return value should parse with the default delimiters.
//     data:
//       planet: "world"
//       lambda: !code
//         ruby:    'proc { "|planet| => {{planet}}" }'
//         perl:    'sub { "|planet| => {{planet}}" }'
//         js:      'function() { return "|planet| => {{planet}}" }'
//         php:     'return "|planet| => {{planet}}";'
//         python:  'lambda: "|planet| => {{planet}}"'
//         clojure: '(fn [] "|planet| => {{planet}}")'
//     template: "{{= | | =}}\nHello, (|&lambda|)!"
//     expected: "Hello, (|planet| => world)!"
// #[test]
// fn test_spec_lambdas_interpolation_alternate_delimeters() {
//     let data = HashBuilder::new()
//                 .insert("planet", "world")
//                 .insert_lambda("lambda", |_| {
//                     "|planet| => {{planet}}".to_string()
//                 });
//     let mut rv = Cursor::new(Vec::new());

//     data.render("{{= | | =}}\nHello, (|&lambda|)!", &mut rv).unwrap();

//     assert_eq!("Hello, (|planet| => world)!".to_string(), String::from_utf8(rv.into_inner()).unwrap());
// }

//   - name: Interpolation - Multiple Calls
//     desc: Interpolated lambdas should not be cached.
//     data:
//       lambda: !code
//         ruby:    'proc { $calls ||= 0; $calls += 1 }'
//         perl:    'sub { no strict; $calls += 1 }'
//         js:      'function() { return (g=(function(){return this})()).calls=(g.calls||0)+1 }'
//         php:     'global $calls; return ++$calls;'
//         python:  'lambda: globals().update(calls=globals().get("calls",0)+1) or calls'
//         clojure: '(def g (atom 0)) (fn [] (swap! g inc))'
//     template: '{{lambda}} == {{{lambda}}} == {{lambda}}'
//     expected: '1 == 2 == 3'
#[test]
fn test_spec_lambdas_interpolation_multiple_calls() {
    let mut calls = 0;
    let mut f = |_| {
        calls += 1;
        calls.to_string()
    };
    let data = HashBuilder::new()
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("{{lambda}} == {{{lambda}}} == {{lambda}}", &mut rv).unwrap();

    assert_eq!("1 == 2 == 3".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Escaping
//     desc: Lambda results should be appropriately escaped.
//     data:
//       lambda: !code
//         ruby:    'proc { ">" }'
//         perl:    'sub { ">" }'
//         js:      'function() { return ">" }'
//         php:     'return ">";'
//         python:  'lambda: ">"'
//         clojure: '(fn [] ">")'
//     template: "<{{lambda}}{{{lambda}}}"
//     expected: "<&gt;>"
#[test]
fn test_spec_lambdas_escaping() {
    let mut f = |_| { ">".to_string() };
    let data = HashBuilder::new()
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("<{{lambda}}{{{lambda}}}", &mut rv).unwrap();

    assert_eq!("<&gt;>".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Section
//     desc: Lambdas used for sections should receive the raw section string.
//     data:
//       x: 'Error!'
//       lambda: !code
//         ruby:    'proc { |text| text == "{{x}}" ? "yes" : "no" }'
//         perl:    'sub { $_[0] eq "{{x}}" ? "yes" : "no" }'
//         js:      'function(txt) { return (txt == "{{x}}" ? "yes" : "no") }'
//         php:     'return ($text == "{{x}}") ? "yes" : "no";'
//         python:  'lambda text: text == "{{x}}" and "yes" or "no"'
//         clojure: '(fn [text] (if (= text "{{x}}") "yes" "no"))'
//     template: "<{{#lambda}}{{x}}{{/lambda}}>"
//     expected: "<yes>"
#[test]
fn test_spec_lambdas_section() {
    let mut f = |txt: String| {
                    if &txt[..] == "{{x}}" {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    }
                };
    let data = HashBuilder::new()
                .insert("x", "Error!")
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("<{{#lambda}}{{x}}{{/lambda}}>", &mut rv).unwrap();

    assert_eq!("<yes>".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Section - Expansion
//     desc: Lambdas used for sections should have their results parsed.
//     data:
//       planet: "Earth"
//       lambda: !code
//         ruby:    'proc { |text| "#{text}{{planet}}#{text}" }'
//         perl:    'sub { $_[0] . "{{planet}}" . $_[0] }'
//         js:      'function(txt) { return txt + "{{planet}}" + txt }'
//         php:     'return $text . "{{planet}}" . $text;'
//         python:  'lambda text: "%s{{planet}}%s" % (text, text)'
//         clojure: '(fn [text] (str text "{{planet}}" text))'
//     template: "<{{#lambda}}-{{/lambda}}>"
//     expected: "<-Earth->"
#[test]
fn test_spec_lambdas_section_expansion() {
    let mut f = |txt: String| {
                   let mut result = txt.clone();
                   result.push_str("{{planet}}");
                   result.push_str(&txt[..]);
                   result
                };
    let data = HashBuilder::new()
                .insert("planet", "Earth")
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("<{{#lambda}}-{{/lambda}}>", &mut rv).unwrap();

    assert_eq!("<-Earth->".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Section - Alternate Delimiters
//     desc: Lambdas used for sections should parse with the current delimiters.
//     data:
//       planet: "Earth"
//       lambda: !code
//         ruby:    'proc { |text| "#{text}{{planet}} => |planet|#{text}" }'
//         perl:    'sub { $_[0] . "{{planet}} => |planet|" . $_[0] }'
//         js:      'function(txt) { return txt + "{{planet}} => |planet|" + txt }'
//         php:     'return $text . "{{planet}} => |planet|" . $text;'
//         python:  'lambda text: "%s{{planet}} => |planet|%s" % (text, text)'
//         clojure: '(fn [text] (str text "{{planet}} => |planet|" text))'
//     template: "{{= | | =}}<|#lambda|-|/lambda|>"
//     expected: "<-{{planet}} => Earth->"
// #[test]
// fn test_spec_lambdas_section_alternate_delimeters() {
//     let data = HashBuilder::new()
//                 .insert("planet", "Earth")
//                 .insert_lambda("lambda", |txt| {
//                     let mut result = txt.to_string();
//                     result.push_str("{{planet}} => |planet|");
//                     result.push_str(txt.as_slice());
//                     result
//                 });
//     let mut rv = Cursor::new(Vec::new());

//     data.render_from_hb("{{= | | =}}<|#lambda|-|/lambda|>", &mut rv).unwrap();

//     assert_eq!("<-{{planet}} => Earth->".to_string(), String::from_utf8(rv.into_inner()).unwrap());
// }

//   - name: Section - Multiple Calls
//     desc: Lambdas used for sections should not be cached.
//     data:
//       lambda: !code
//         ruby:    'proc { |text| "__#{text}__" }'
//         perl:    'sub { "__" . $_[0] . "__" }'
//         js:      'function(txt) { return "__" + txt + "__" }'
//         php:     'return "__" . $text . "__";'
//         python:  'lambda text: "__%s__" % (text)'
//         clojure: '(fn [text] (str "__" text "__"))'
//     template: '{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}'
//     expected: '__FILE__ != __LINE__'
#[test]
fn test_spec_lambdas_section_multiple_calls() {
    let mut f = |txt: String| {
                    let mut result = "__".to_string();
                    result.push_str(&txt[..]);
                    result.push_str("__");
                    result
                };
    let data = HashBuilder::new()
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}", &mut rv).unwrap();

    assert_eq!("__FILE__ != __LINE__".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Inverted Section
//     desc: Lambdas used for inverted sections should be considered truthy.
//     data:
//       static: 'static'
//       lambda: !code
//         ruby:    'proc { |text| false }'
//         perl:    'sub { 0 }'
//         js:      'function(txt) { return false }'
//         php:     'return false;'
//         python:  'lambda text: 0'
//         clojure: '(fn [text] false)'
//     template: "<{{^lambda}}{{static}}{{/lambda}}>"
//     expected: "<>"
#[test]
fn test_spec_lambdas_inverted_section() {
    let mut f = |_| { "false".to_string() };
    let data = HashBuilder::new()
                .insert("static", "static")
                .insert_lambda("lambda", &mut f);
    let mut rv = Cursor::new(Vec::new());

    data.render("<{{^lambda}}{{static}}{{/lambda}}>", &mut rv).unwrap();

    assert_eq!("<>".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

