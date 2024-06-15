use droid_wrap::android::app::Activity;
use droid_wrap::java::lang::{CharSequence, Integer};


#[mobile_entry_point::mobile_entry_point]
fn main() {
    let act = Activity::from_ctx();
    act.finish();
    let cs = CharSequence::from("hello");
    dbg!(&cs);
    act.set_title(cs);
    dbg!(act);
    let num = Integer::new(6);
    dbg!(num);
}