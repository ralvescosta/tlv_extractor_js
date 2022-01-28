use neon::prelude::*;
use std::str::FromStr;

fn tlv_extractor(mut cx: FunctionContext) -> JsResult<JsObject> {
    let mut tlv_data = cx.argument::<JsString>(0)?.value(&mut cx);
    let tv = cx.empty_object();

    if tlv_data.is_empty() {
        return Ok(tv);
    }

    let tag_size = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let length_size = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;

    loop {
        let next_tlv_data = tlv_data.clone();

        let tag = match next_tlv_data.get(0..tag_size) {
            Some(t) => t,
            None => break,
        };

        let value_length = match next_tlv_data.get(tag_size..length_size + tag_size) {
            Some(v) => v,
            None => break,
        };

        let value_start_position = tag_size + length_size;
        let u_value_length = value_length.parse::<usize>().expect("Error u_value_length");

        let value_end_position = value_start_position + u_value_length;
        let value = match next_tlv_data.get(value_start_position..value_end_position) {
            Some(v) => v,
            None => break,
        };

        let next_value_start_position = value_start_position + u_value_length;
        let next_data = match next_tlv_data.get(next_value_start_position..next_tlv_data.len()) {
            Some(v) => v,
            None => "",
        };
        tlv_data = String::from_str(next_data).expect("Error tlv_data conversion");

        let val = cx.string(value);
        tv.set(&mut cx, tag.clone(), val)?;
    }

    Ok(tv)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("tlvExtractor", tlv_extractor)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use neon::prelude::*;

    #[test]
    fn test_first() {}
}
