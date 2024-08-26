use crate::ParseResult;
use crate::Remark;
use crate::RemarkType;
use rcore::RemarkAssociation;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasRemark {
    r#type: Option<String>,
    txt_s: Option<String>,
    txt_n: Option<String>,
    code: Option<String>,
    jid: Option<String>,
}

pub fn default_parse_remark<F: FnOnce(&str) -> RemarkAssociation>(
    rem: HafasRemark,
    association: F,
) -> ParseResult<Remark> {
    let association = rem
        .code
        .as_ref()
        .map(|c| association(c))
        .unwrap_or_default();
    Ok(match rem.r#type.as_deref() {
        Some("M") | Some("P") => Remark {
            r#type: RemarkType::Status,
            code: rem.code.ok_or("Missing code")?,
            text: rem.txt_n.ok_or("Missing remark text")?,
            trip_id: None,
            summary: rem.txt_s,
            association,
        },
        Some("L") => Remark {
            r#type: RemarkType::Status,
            code: "alternative-trip".to_string(),
            text: rem.txt_n.ok_or("Missing remark text")?,
            trip_id: rem.jid,
            summary: None,
            association,
        },
        Some("A") | Some("I") | Some("H") => Remark {
            r#type: RemarkType::Hint,
            code: rem.code.ok_or("Missing code")?,
            text: rem.txt_n.ok_or("Missing remark text")?,
            trip_id: None,
            summary: None,
            association,
        },
        _ => Remark {
            // TODO: parse more accurately
            r#type: RemarkType::Status,
            code: rem.code.ok_or("Missing code")?,
            text: rem.txt_n.ok_or("Missing remark text")?,
            trip_id: None,
            summary: None,
            association,
        },
    })
}
