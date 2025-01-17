// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DissociatePackage`](crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`package_id(impl ::std::convert::Into<String>)`](crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder::package_id) / [`set_package_id(Option<String>)`](crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder::set_package_id): <p>Internal ID of the package that you want to associate with a domain. Use <code>DescribePackages</code> to find this value.</p>
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder::set_domain_name): <p>Name of the domain that you want to associate the package with.</p>
    /// - On success, responds with [`DissociatePackageOutput`](crate::operation::dissociate_package::DissociatePackageOutput) with field(s):
    ///   - [`domain_package_details(Option<DomainPackageDetails>)`](crate::operation::dissociate_package::DissociatePackageOutput::domain_package_details): <p><code>DomainPackageDetails</code></p>
    /// - On failure, responds with [`SdkError<DissociatePackageError>`](crate::operation::dissociate_package::DissociatePackageError)
    pub fn dissociate_package(&self) -> crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder {
        crate::operation::dissociate_package::builders::DissociatePackageFluentBuilder::new(self.handle.clone())
    }
}
