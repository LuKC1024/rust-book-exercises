//! In this file, you will design functions to implement a high-level specification.
//! The goal is to have you explore the different possible implementations of a spec in Rust,
//! and to articulate the trade-offs in terms of generality, performance, and usability.

// EXAMPLE: below is a completed function that demonstrates how each problem will work.
// Each problem contains a specification above the function. Your goal is to design the function
// signature and implementation. For each parameter and the return type, you should describe
// (a) a reasonable space of possible types, and (b) your rationale for picking a particular type.
// Make sure to add at least one unit test for your particular implementation.

/// round_all is a function that takes:
///   * v: representing a collection of numbers
/// and rounds every number in-place in v to the nearest integer.
pub fn round_all(
    // (1) v could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &mut[_] because
    //     we do not need to change the size or order of the collection, but do need to change the elements.
    // (2) v could be a &mut [{number type}], and "round to the nearest integer" implies the use of floats.
    // (3) The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
    //     function that works for both types, so we arbitrarily pick f32 for now.
    v: &mut [f32],
)
// No return value, since this function only mutates an input.
{
    for n in v.iter_mut() {
        *n = n.round();
    }
}

#[test]
fn round_all_test() {
    let mut v = vec![0.3, 0.7];
    round_all(&mut v);
    assert_eq!(v, vec![0., 1.]);
}

// Now you try!

/// P2a: find_contains is a function that takes:
///   * haystack: representing a collection of strings
///   * needle: representing a particular string
/// and returns a value:
///   * representing which strings in the collection contain the needle
pub fn find_contains(
    /* Pick and justify your parameters */
    // (1) haystack could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_].
    //     I choose &[_] because
    //        we do not need to change the size or order of the collection,
    //     and because
    //        we also do not need to change the elements.
    // (2) haystack could be a &[&str] or &[&String] or &[String].
    //     I choose &[&str] because it is the most permissive option.
    // (3) needle could be a &str or &String or String.
    //     I choose &str because it is the most permissive option.
    haystack: &[&str],
    needle: &str,
) -> Vec<String>
/* Pick and justify your return type */
    // (1) The return type could be Vec<String>. So I choose it.
    //     Types with `&` are not options because Rust will ask me to specify the "lifetime", which is a concept that I don't understand.
    //     Types with `[_]` are not options because the size depends on the inputs.
{
    let mut result = vec![];
    for s in haystack.iter() {
        if s.contains(needle) {
            result.push(String::from(*s));
        }
    }
    return result;
}

#[test]
fn find_contains_test() {
    /* Add your unit test here! */
    assert_eq!(
        find_contains(&["hello", "world"], "l"),
        vec!["hello", "world"]
    );
    assert_eq!(find_contains(&["hello", "world"], "ld"), vec!["world"]);
    assert_eq!(find_contains(&["hello", "world"], "ll"), vec!["hello"]);
}

/// P2b: fill_progress_bar is a function that takes:
///   * buf: a string to fill
///   * delims: a pair of delimiters to wrap the bar
///   * frac: the fraction of the bar to display
/// Then places a textual representation of the progress bar into `buf`.
/// For example, at a progress of 20% with bracketed delimiters, the bar would be:
///   [==        ]
pub fn fill_progress_bar(
    /* Pick and justify your parameters */
    // (1) buf could be a &mut String or &mut str.
    //     I choose &mut String because I don't know how to add content to an &mut str.
    //     Types without &mut are not options because we need to change the *same* buf.
    // (2) delims could be a pair of String or str or &str or &String or &mut str or &mut String.
    //     I choose a pair of &str because it is the most permissive option -- we need neither the ownership nor the "write access".
    // (3) frac could be {number type} or &{number type}.
    //     I choose {number type} because number types are freely copiable. Furthermore, copying them is sometimes cheaper
    //     than copying their references.
    // (4) frac could be f32 and f64.
    //     The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
    //     function that works for both types, so we arbitrarily pick f32 for now.
    buf: &mut String,
    delims: (&str, &str),
    frac: f32,
) -> ()
/* Pick and justify your return type */
    // (1) The return type could be anything.
    //     I choose the unit type to remind the users of this function that calling this function has a side effect.
{
    let total = buf.len();
    buf.clear();
    let (lft, rht) = delims;
    let barlen = total - lft.len() - rht.len();
    let n: usize = (barlen as f32 * frac).round() as usize;
    let m: usize = barlen - n;
    let fll = "=".repeat(n);
    let spc = " ".repeat(m);
    buf.push_str(lft);
    buf.push_str(&fll[..]);
    buf.push_str(&spc[..]);
    buf.push_str(rht)
}

#[test]
fn test_fill_progress_bar() {
    let mut s = String::from("[==        ]");
    fill_progress_bar(&mut s, ("[", "]"), 1.0);
    assert_eq!(s, "[==========]");
    fill_progress_bar(&mut s, ("<", ">"), 0.0);
    assert_eq!(s, "<          >");
    fill_progress_bar(&mut s, ("{", "}"), 0.2);
    assert_eq!(s, "{==        }");
}
