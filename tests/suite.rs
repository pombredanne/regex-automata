use regex_automata::{DenseDFARef, Regex, RegexBuilder};

use collection::{SUITE, RegexTester};

#[test]
fn unminimized_standard() {
    let mut builder = RegexBuilder::new();
    builder.minimize(false).premultiply(false).byte_classes(false);

    let mut tester = RegexTester::new().skip_expensive();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn unminimized_premultiply() {
    let mut builder = RegexBuilder::new();
    builder.minimize(false).premultiply(true).byte_classes(false);

    let mut tester = RegexTester::new().skip_expensive();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn unminimized_byte_class() {
    let mut builder = RegexBuilder::new();
    builder.minimize(false).premultiply(false).byte_classes(true);

    let mut tester = RegexTester::new();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn unminimized_premultiply_byte_class() {
    let mut builder = RegexBuilder::new();
    builder.minimize(false).premultiply(true).byte_classes(true);

    let mut tester = RegexTester::new();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn minimized_standard() {
    let mut builder = RegexBuilder::new();
    builder.minimize(true).premultiply(false).byte_classes(false);

    let mut tester = RegexTester::new().skip_expensive();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn minimized_premultiply() {
    let mut builder = RegexBuilder::new();
    builder.minimize(true).premultiply(true).byte_classes(false);

    let mut tester = RegexTester::new().skip_expensive();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn minimized_byte_class() {
    let mut builder = RegexBuilder::new();
    builder.minimize(true).premultiply(false).byte_classes(true);

    let mut tester = RegexTester::new();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

#[test]
fn minimized_premultiply_byte_class() {
    let mut builder = RegexBuilder::new();
    builder.minimize(true).premultiply(true).byte_classes(true);

    let mut tester = RegexTester::new();
    tester.test_all(builder, SUITE.tests());
    tester.assert();
}

// A basic sanity test that checks we can convert a regex to a smaller
// representation and that the resulting regex still passes our tests.
//
// If tests grow minimal regexes that cannot be represented in 16 bits, then
// we'll either want to skip those or increase the size to test to u32.
#[test]
fn u16() {
    let mut builder = RegexBuilder::new();
    builder.minimize(true).premultiply(false).byte_classes(true);

    let mut tester = RegexTester::new();
    for test in SUITE.tests() {
        let builder = builder.clone();
        let re: Regex<usize> = match tester.build_regex(builder, test) {
            None => continue,
            Some(re) => re,
        };
        let re = re.to_u16().unwrap();

        tester.test_is_match(test, &re);
        tester.test_find(test, &re);
    }
}

// Test that sparse DFAs work using the standard configuration.
#[test]
fn sparse_unminimized_standard() {
    let mut builder = RegexBuilder::new();
    builder.minimize(false).premultiply(false).byte_classes(false);

    let mut tester = RegexTester::new().skip_expensive();
    for test in SUITE.tests() {
        let builder = builder.clone();
        let re: Regex<usize> = match tester.build_regex(builder, test) {
            None => continue,
            Some(re) => re,
        };
        let re = ::regex_automata::DenseDFA::from_dfa_ref(re.forward()).to_sparse_dfa().unwrap();

        tester.test_is_match_sparse(test, &re);
    }
}

// Another basic sanity test that checks we can serialize and then deserialize
// a regex, and that the resulting regex can be used for searching correctly.
#[test]
fn serialization_roundtrip() {
    let mut builder = RegexBuilder::new();
    builder.minimize(true).premultiply(false).byte_classes(true);

    let mut tester = RegexTester::new();
    for test in SUITE.tests() {
        let builder = builder.clone();
        let re: Regex<usize> = match tester.build_regex(builder, test) {
            None => continue,
            Some(re) => re,
        };

        let fwd_bytes = re.forward().to_bytes_native_endian().unwrap();
        let rev_bytes = re.reverse().to_bytes_native_endian().unwrap();
        let fwd: DenseDFARef<usize> = unsafe {
            DenseDFARef::from_bytes(&fwd_bytes)
        };
        let rev: DenseDFARef<usize> = unsafe {
            DenseDFARef::from_bytes(&rev_bytes)
        };
        let re = Regex::from_dfa_refs(fwd, rev);

        tester.test_is_match(test, &re);
        tester.test_find(test, &re);
    }
}
