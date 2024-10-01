
use parcelona::parser_combinators::{*};
use parcelona::u8::{*};

#[derive(Debug)]
pub enum Item<I> {
	IText(I),
	ITag(Tag<I>),
} 

#[derive(Debug)]
pub struct Tag<I> {
	pub name: I,
	pub attributes: Option<Vec<(I,I)>>,
	pub items: Vec<Item<I>>,
}

const OPEN_TAG_NOTFOUND:  &str = r#"opent tag '<' not found"#;
const CLOSE_TAG_NOTFOUND: &str = r#"close tag '>' not found"#;
const SEP_NOTFOUND:       &str = r#"'=' not found"#;
const END_TAG_NOTFOUND:   &str = r#"end tag '</ >' not found"#;
const QUOTE_NOTFOUND:     &str = r#"quote " not found"#;
const NAME_ERR:           &str = r#"name parse error"#;
const VALUE_ERR:          &str = r#"value parse error"#;
const ATTR_ERR:           &str = r#"attribut parse error"#;
const TEXT_ERR1:          &str = r#"text parse error"#;
const HEAD_ERR:           &str = r#"head parse error"#;
const CONTENT_ERR:        &str = r#"inner content parse error"#;

static NAME: StaticClassOfSymbols<u8> = StaticClassOfSymbols::new()
	.range_enable_set(ALPHA_NUM)
	.one_enable_set(&[45,46,95]); // - . _

static VALUE: StaticClassOfSymbols<u8> = StaticClassOfSymbols::new()
	.one_disable_set(&[34]) // "
	.default_enable_one(true);

static TEXT: StaticClassOfSymbols<u8> = StaticClassOfSymbols::new()
	.one_disable_set(br#"<>\"#)  // <>\
	.parts_enable_set(&[br#"\\"#, br#"\<"#, br#"\>"#])
	.default_enable_one(true);   

fn parse_tag(input: &[u8]) -> ParseResult<u8,Item<&[u8]>> {
	let space  = seq(is_space);
	let open   = between_opt(space, starts_with(b"<"), space).msg_err(OPEN_TAG_NOTFOUND);
	let close  = between_opt(space, starts_with(b">"), space).msg_err(CLOSE_TAG_NOTFOUND);
	let sep    = starts_with(b"=").msg_err(SEP_NOTFOUND);
	let quotes = between_opt(space, starts_with(b"\""), space).msg_err(QUOTE_NOTFOUND);
	let name_parser  = between_opt(space, &NAME, space).msg_err(NAME_ERR);
	let value_parser = fmap(between(quotes, &VALUE, quotes).msg_err(VALUE_ERR),<[u8]>::trim_ascii);
	let text  = fmap(TEXT.msg_err(TEXT_ERR1),|x|{Item::<&[u8]>::IText(<[u8]>::trim_ascii(x))});
	let attrs = sep_pair(name_parser, sep, value_parser).msg_err(ATTR_ERR).more().option();

    // firs line tag
	let (input, (tag_name, tag_attrs)) = between(open, pair(name_parser, attrs), close)
		.msg_err(HEAD_ERR).strerr().parse(input)?;
	
    // inner content
	let (input, it) = (text,parse_tag).alt().msg_err(CONTENT_ERR).more().strerr().parse(input)?;
    
    // close line tag
	let (input, _) = between(open, pair(any(b"/"), starts_with(tag_name)), close)
		.msg_err(END_TAG_NOTFOUND).strerr().parse(input)?;

	Ok((input, Item::<&[u8]>::ITag(Tag {
		name: tag_name,
		attributes: tag_attrs,
		items: it,
	})))
}

pub fn parse<'a>(input: &'a[u8]) -> ParseResult<'a,u8,Vec<Item::<&'a[u8]>>> {
	let text  = fmap(TEXT.msg_err(TEXT_ERR1),|x|{Item::<&[u8]>::IText(<[u8]>::trim_ascii(x))});

	(text,parse_tag).alt().msg_err(CONTENT_ERR).more().strerr().parse(input)
}
