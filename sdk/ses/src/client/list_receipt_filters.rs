// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListReceiptFilters`](crate::operation::list_receipt_filters::builders::ListReceiptFiltersFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::list_receipt_filters::builders::ListReceiptFiltersFluentBuilder::send) it.
    /// - On success, responds with [`ListReceiptFiltersOutput`](crate::operation::list_receipt_filters::ListReceiptFiltersOutput) with field(s):
    ///   - [`filters(Option<Vec<ReceiptFilter>>)`](crate::operation::list_receipt_filters::ListReceiptFiltersOutput::filters): <p>A list of IP address filter data structures, which each consist of a name, an IP address range, and whether to allow or block mail from it.</p>
    /// - On failure, responds with [`SdkError<ListReceiptFiltersError>`](crate::operation::list_receipt_filters::ListReceiptFiltersError)
    pub fn list_receipt_filters(&self) -> crate::operation::list_receipt_filters::builders::ListReceiptFiltersFluentBuilder {
        crate::operation::list_receipt_filters::builders::ListReceiptFiltersFluentBuilder::new(self.handle.clone())
    }
}
