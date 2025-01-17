// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDeliveryStream`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`delivery_stream_name(impl ::std::convert::Into<String>)`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::delivery_stream_name) / [`set_delivery_stream_name(Option<String>)`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::set_delivery_stream_name): <p>The name of the delivery stream.</p>
    ///   - [`limit(i32)`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::set_limit): <p>The limit on the number of destinations to return. You can have one destination per delivery stream.</p>
    ///   - [`exclusive_start_destination_id(impl ::std::convert::Into<String>)`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::exclusive_start_destination_id) / [`set_exclusive_start_destination_id(Option<String>)`](crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::set_exclusive_start_destination_id): <p>The ID of the destination to start returning the destination information. Kinesis Data Firehose supports one destination per delivery stream.</p>
    /// - On success, responds with [`DescribeDeliveryStreamOutput`](crate::operation::describe_delivery_stream::DescribeDeliveryStreamOutput) with field(s):
    ///   - [`delivery_stream_description(Option<DeliveryStreamDescription>)`](crate::operation::describe_delivery_stream::DescribeDeliveryStreamOutput::delivery_stream_description): <p>Information about the delivery stream.</p>
    /// - On failure, responds with [`SdkError<DescribeDeliveryStreamError>`](crate::operation::describe_delivery_stream::DescribeDeliveryStreamError)
    pub fn describe_delivery_stream(&self) -> crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder {
        crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamFluentBuilder::new(self.handle.clone())
    }
}
