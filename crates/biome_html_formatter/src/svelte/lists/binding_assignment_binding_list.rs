use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::SvelteBindingAssignmentBindingList;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindingAssignmentBindingList;
impl FormatRule<SvelteBindingAssignmentBindingList> for FormatSvelteBindingAssignmentBindingList {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &SvelteBindingAssignmentBindingList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let mut is_first = true;

        for binding_assignment in node.elements() {
            if !is_first {
                write!(f, [space()])?;
            }
            is_first = false;

            // An array pattern may skip a position, as in `[, second]`. That
            // hole has no node of its own; only the separator after it says it
            // is there, so a missing node here is written as nothing rather
            // than giving up on the whole file.
            if let Ok(node) = binding_assignment.node() {
                write!(f, [node.format()])?;
            }

            if let Some(separator) = binding_assignment.trailing_separator()? {
                write!(f, [separator.format()])?;
            }
        }

        Ok(())
    }
}
