// TODO: Figure out what is actually needed
use nom::{
    branch::alt,
    bytes::streaming::{tag, tag_no_case, take_while},
    combinator::{map, map_res, recognize},
    sequence::pair,
    IResult,
};
use std::{borrow::Cow, str::from_utf8};

use crate::{parser::core::*, types::*};

fn no_inferiors(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Noinferiors"), |_s| {
        NameAttribute::NoInferiors
    })(i)
}

fn no_select(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Noselect"), |_s| NameAttribute::NoSelect)(i)
}

fn marked(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Marked"), |_s| NameAttribute::Marked)(i)
}

fn unmarked(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Unmarked"), |_s| NameAttribute::Unmarked)(i)
}

fn all(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\All"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::All)
    })(i)
}

fn archive(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Archive"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Archive)
    })(i)
}

fn drafts(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Drafts"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Drafts)
    })(i)
}

fn flagged(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Flagged"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Flagged)
    })(i)
}

fn junk(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Junk"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Junk)
    })(i)
}

fn sent(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Sent"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Sent)
    })(i)
}

fn trash(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(tag_no_case(b"\\Trash"), |_s| {
        NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Trash)
    })(i)
}

fn extension_str(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        recognize(pair(tag(b"\\"), take_while(is_atom_char))),
        from_utf8,
    )(i)
}

fn extension(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    map(extension_str, |s| {
        NameAttribute::Extension(Cow::Borrowed(s))
    })(i)
}

fn special_use_mailbox(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    alt((all, archive, drafts, flagged, junk, sent, trash))(i)
}

fn name_attribute(i: &[u8]) -> IResult<&[u8], NameAttribute> {
    alt((
        no_inferiors,
        no_select,
        marked,
        unmarked,
        special_use_mailbox,
        extension,
    ))(i)
}

pub(crate) fn name_attributes(i: &[u8]) -> IResult<&[u8], Vec<NameAttribute>> {
    parenthesized_list(name_attribute)(i)
}
