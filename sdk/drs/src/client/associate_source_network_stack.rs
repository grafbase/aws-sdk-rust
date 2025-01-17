// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateSourceNetworkStack`](crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_network_id(impl ::std::convert::Into<String>)`](crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder::source_network_id) / [`set_source_network_id(Option<String>)`](crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder::set_source_network_id): <p>The Source Network ID to associate with CloudFormation template.</p>
    ///   - [`cfn_stack_name(impl ::std::convert::Into<String>)`](crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder::cfn_stack_name) / [`set_cfn_stack_name(Option<String>)`](crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder::set_cfn_stack_name): <p>CloudFormation template to associate with a Source Network.</p>
    /// - On success, responds with [`AssociateSourceNetworkStackOutput`](crate::operation::associate_source_network_stack::AssociateSourceNetworkStackOutput) with field(s):
    ///   - [`job(Option<Job>)`](crate::operation::associate_source_network_stack::AssociateSourceNetworkStackOutput::job): <p>The Source Network association Job.</p>
    /// - On failure, responds with [`SdkError<AssociateSourceNetworkStackError>`](crate::operation::associate_source_network_stack::AssociateSourceNetworkStackError)
    pub fn associate_source_network_stack(
        &self,
    ) -> crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder {
        crate::operation::associate_source_network_stack::builders::AssociateSourceNetworkStackFluentBuilder::new(self.handle.clone())
    }
}
