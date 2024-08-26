use crate::ParseResult;
use crate::Product;

pub(crate) fn default_parse_products<'a>(
    p_cls: u16,
    products: &'a [&'a Product],
) -> Vec<&'a Product> {
    let mut result = vec![];
    for (i, p) in products.iter().enumerate() {
        if (1 << i) & p_cls != 0 {
            result.push(*p);
        }
    }
    result
}

pub(crate) fn default_parse_product<'a>(
    p_cls: u16,
    products: &'a [&'a Product],
) -> ParseResult<&'a Product> {
    for (i, p) in products.iter().enumerate() {
        if (1 << i) == p_cls {
            return Ok(p);
        }
    }
    Err(format!("Unknown product bit: {:b}", p_cls).into())
}
