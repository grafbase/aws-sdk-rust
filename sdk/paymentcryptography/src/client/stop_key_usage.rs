// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopKeyUsage`](crate::operation::stop_key_usage::builders::StopKeyUsageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_identifier(impl ::std::convert::Into<String>)`](crate::operation::stop_key_usage::builders::StopKeyUsageFluentBuilder::key_identifier) / [`set_key_identifier(Option<String>)`](crate::operation::stop_key_usage::builders::StopKeyUsageFluentBuilder::set_key_identifier): <p>The <code>KeyArn</code> of the key.</p>
    /// - On success, responds with [`StopKeyUsageOutput`](crate::operation::stop_key_usage::StopKeyUsageOutput) with field(s):
    ///   - [`key(Option<Key>)`](crate::operation::stop_key_usage::StopKeyUsageOutput::key): <p>The <code>KeyARN</code> of the key.</p>
    /// - On failure, responds with [`SdkError<StopKeyUsageError>`](crate::operation::stop_key_usage::StopKeyUsageError)
    pub fn stop_key_usage(&self) -> crate::operation::stop_key_usage::builders::StopKeyUsageFluentBuilder {
        crate::operation::stop_key_usage::builders::StopKeyUsageFluentBuilder::new(self.handle.clone())
    }
}
