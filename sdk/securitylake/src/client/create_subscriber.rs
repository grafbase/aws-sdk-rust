// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSubscriber`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`subscriber_identity(AwsIdentity)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::subscriber_identity) / [`set_subscriber_identity(Option<AwsIdentity>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::set_subscriber_identity): <p>The AWS identity used to access your data.</p>
    ///   - [`subscriber_name(impl ::std::convert::Into<String>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::subscriber_name) / [`set_subscriber_name(Option<String>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::set_subscriber_name): <p>The name of your Security Lake subscriber account.</p>
    ///   - [`subscriber_description(impl ::std::convert::Into<String>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::subscriber_description) / [`set_subscriber_description(Option<String>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::set_subscriber_description): <p>The description for your subscriber account in Security Lake.</p>
    ///   - [`sources(Vec<LogSourceResource>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::sources) / [`set_sources(Option<Vec<LogSourceResource>>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::set_sources): <p>The supported Amazon Web Services from which logs and events are collected. Security Lake supports log and event collection for natively supported Amazon Web Services.</p>
    ///   - [`access_types(Vec<AccessType>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::access_types) / [`set_access_types(Option<Vec<AccessType>>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::set_access_types): <p>The Amazon S3 or Lake Formation access type.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::set_tags): <p>An array of objects, one for each tag to associate with the subscriber. For each tag, you must specify both a tag key and a tag value. A tag value cannot be null, but it can be an empty string.</p>
    /// - On success, responds with [`CreateSubscriberOutput`](crate::operation::create_subscriber::CreateSubscriberOutput) with field(s):
    ///   - [`subscriber(Option<SubscriberResource>)`](crate::operation::create_subscriber::CreateSubscriberOutput::subscriber): <p>Retrieve information about the subscriber created using the <code>CreateSubscriber</code> API.</p>
    /// - On failure, responds with [`SdkError<CreateSubscriberError>`](crate::operation::create_subscriber::CreateSubscriberError)
    pub fn create_subscriber(&self) -> crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder {
        crate::operation::create_subscriber::builders::CreateSubscriberFluentBuilder::new(self.handle.clone())
    }
}
