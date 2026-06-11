use droid_wrap::{android_main, with_env, Result, java::lang::System};

#[android_main]
fn main() -> Result<()> {
    with_env(|env| {
        System::gc(env)?;
        System::run_finalization(env)?;
        System::exit(env, 0)?;
        Ok(())
    })
}
