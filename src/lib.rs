use neon::prelude::*;

fn tlv_extractor(mut cx: FunctionContext) -> JsResult<JsObject> {
    let tlv_data = cx.argument::<JsString>(0)?.value(&mut cx);
    let tag_size = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let length_size = cx.argument::<JsNumber>(2)?.value(&mut cx);

    let tv = cx.empty_object();

    Ok(tv)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("tlvExtractor", tlv_extractor)?;
    Ok(())
}
