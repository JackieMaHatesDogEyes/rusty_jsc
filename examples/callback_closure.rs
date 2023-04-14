use rusty_jsc::{callback_closure, JSContext, JSObject, JSValue};

fn main() {
    let mut context = JSContext::default();

    let mut sum = 0;

    let binded_callback = callback_closure!(&context, move |ctx: JSContext,
                                                            function: JSObject,
                                                            this: JSObject,
                                                            args: &[JSValue]|
          -> Result<JSValue, JSValue> {
        println!(
            "hello from Rust land! len: {}, value[0]: {}, sum: {}",
            args.len(),
            args[0].to_string(&ctx),
            sum,
        );
        sum += 10;
        Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
    });

    let binded_callback_o = binded_callback.to_object(&context);
    binded_callback_o.call(
        &context,
        binded_callback_o.clone(),
        &[JSValue::number(&context, 5f64)],
    );
    println!("D");

    binded_callback_o.call(
        &context,
        binded_callback_o.clone(),
        &[
            JSValue::number(&context, 5f64),
            // JSValue::number(&context, 6f64),
        ],
    );
    println!("D");

    // global.set_property(&context, "foo".to_string(), binded_callback.unwrap());
    // println!("B");

    // let foo = global
    //     .get_property(&context, "foo".to_string())
    //     .to_object(&context);
    // println!("C");
    // let result = foo.call(
    //     &context,
    //     foo.clone(),
    //     &[
    //         // JSValue::number(&context, 5f64),
    //         // JSValue::number(&context, 6f64),
    //     ],
    // );
    // println!("D");

    // println!("direct call: {}", result.unwrap().to_string(&context));
    // match context.evaluate_script("foo(1, 2, 3)", 1) {
    //     Some(value) => {
    //         println!("{}", value.to_string(&context));
    //     }
    //     None => {
    //         println!(
    //             "Uncaught: {}",
    //             context.get_exception().unwrap().to_string(&context)
    //         )
    //     }
    // }
}