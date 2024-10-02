extern crate tag_parser;
use tag_parser::{*};
use std::mem;


const DATA: &str = 
r#"<poet author="Byron" title="The Girl of Cadiz" date="1809">

<bold>
Oh never talk again to me
</bold>
Of northern climes and British ladies;
<comment this="exampl" />
It has not been your lot to see,
Like me, the lovely Girl of Cadiz.
Although her eye be not of blue,
Nor fair her locks, like English lasses,
How far its own expressive hue
The languid azure eye surpasses!


</poet>"#; 


fn main() {
	let r = parse(DATA.as_bytes());
	let rr:Vec<Item::<&str>>;
  
	match r {
		Ok((_,r))  => {
			unsafe { rr = mem::transmute::<Vec<Item::<&[u8]>>,Vec<Item::<&str>>>(r); }  			
			println!("{:#?}", rr);
		},
		Err(r) => println!("{}", r),
	}
}
