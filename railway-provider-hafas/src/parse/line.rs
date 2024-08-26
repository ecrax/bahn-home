use crate::Line;
use crate::Operator;
use crate::ParseResult;
use crate::Profile;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLineProdCtx {
    num: Option<String>,
    cat_out: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLine {
    line: Option<String>,
    add_name: Option<String>,
    name: Option<String>,
    prod_ctx: Option<HafasLineProdCtx>,
    opr_x: Option<usize>,
    cls: Option<u16>,
}

pub(crate) fn default_parse_line<P: Profile + ?Sized>(
    profile: &P,
    data: HafasLine,
    operators: &[Operator],
) -> ParseResult<Line> {
    let HafasLine {
        line,
        add_name,
        name,
        prod_ctx,
        opr_x,
        cls,
    } = data;
    let product = profile.parse_product(cls.ok_or("Missing cls field")?)?;
    Ok(Line {
        name: line.or(add_name).or(name),
        fahrt_nr: prod_ctx.as_ref().and_then(|x| x.num.clone()),
        operator: opr_x.and_then(|x| operators.get(x)).cloned(),
        mode: product.mode.clone(),
        product_name: prod_ctx
            .and_then(|x| x.cat_out)
            .map(|s| s.trim().to_owned()),
        product: product.clone(),
    })
}
