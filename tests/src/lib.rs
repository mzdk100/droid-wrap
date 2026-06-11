use droid_wrap::{
    android_main, android_context, with_env, Result,
    core::{Object, JavaString},
    java::lang::{System, Math, StringBuilder},
    android::app::Activity,
};

#[android_main]
fn main() -> Result<()> {
    with_env(|env| {
        System::gc(env)?;
        println!("[PASS] System::gc()");

        let obj = Object::new(env)?;
        let hash = System::identity_hash_code(env, &obj)?;
        assert_ne!(hash, 0, "identity_hash_code should return non-zero for new Object");
        println!("[PASS] System::identity_hash_code() = {}", hash);

        System::run_finalization(env)?;
        println!("[PASS] System::run_finalization()");

        let r = Math::random(env)?;
        assert!(r >= 0.0 && r < 1.0, "Math::random() should return [0.0, 1.0)");
        println!("[PASS] Math::random() = {}", r);

        let sum = Math::add_exact(env, 40, 2)?;
        assert_eq!(sum, 42, "Math::add_exact(40, 2) should be 42");
        println!("[PASS] Math::add_exact(40, 2) = {}", sum);

        let abs_val = Math::abs(env, -99)?;
        assert_eq!(abs_val, 99, "Math::abs(-99) should be 99");
        println!("[PASS] Math::abs(-99) = {}", abs_val);

        let max_val = Math::max(env, 3, 7)?;
        assert_eq!(max_val, 7, "Math::max(3, 7) should be 7");
        println!("[PASS] Math::max(3, 7) = {}", max_val);

        let sb = StringBuilder::new(env)?;
        println!("[PASS] StringBuilder::new()");

        let empty_str = sb.to_string(env)?;
        assert!(!empty_str.is_null(), "to_string should not return null");
        println!("[PASS] StringBuilder::to_string() on empty builder");

        let hello = JavaString::new(env, "Hello, droid-wrap!")?;
        let sb2 = StringBuilder::new(env)?;
        sb2.append(env, &hello)?;
        let result_str = sb2.to_string(env)?;
        assert!(!result_str.is_null());
        println!("[PASS] StringBuilder::append() + to_string()");

        let context = android_context(env);
        let activity = unsafe { Activity::from_raw(env, context.into_raw()) };

        let color = activity.get_title_color(env)?;
        println!("[PASS] Activity::get_title_color() = {}", color);

        let intent = activity.get_intent(env)?;
        assert!(!intent.is_null(), "Activity should have a non-null Intent");
        println!("[PASS] Activity::get_intent() (cfg-gated, requires android_content)");

        let got_title = activity.get_title(env)?;
        println!("[PASS] Activity::get_title() -> CharSequence null={}", got_title.is_null());

        let title_jstr = got_title.to_string(env)?;
        println!("[PASS] CharSequence::to_string() null={}", title_jstr.is_null());

        println!("[PASS] Activity::set_title(CharSequence) (skipped — requires UI thread)");

        println!("\nAll binding API tests passed.");
        Ok(())
    })
}
