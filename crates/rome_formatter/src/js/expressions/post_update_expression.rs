use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsPostUpdateExpression;

impl ToFormatElement for JsPostUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.operand().format(formatter)?,
            self.operator().format(formatter)?,
        ])
    }
}
