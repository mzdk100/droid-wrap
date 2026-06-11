use droid_wrap::{android_main, android_context, java_object_to_string, with_env, Result, android::app::Activity};

#[android_main]
fn main() -> Result<()> {
    with_env(|env| {
        let context = android_context(env);
        let activity = unsafe { Activity::from_raw(env, context.into_raw()) };

        let title = activity.get_title(env)?;
        if !title.is_null() {
            let title_str = java_object_to_string(env, &title)?;
            println!("Activity title: {}", title_str);
        }

        activity.finish(env)?;

        Ok(())
    })
}
