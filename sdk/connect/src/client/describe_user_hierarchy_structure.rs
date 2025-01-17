// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeUserHierarchyStructure`](crate::operation::describe_user_hierarchy_structure::builders::DescribeUserHierarchyStructureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::describe_user_hierarchy_structure::builders::DescribeUserHierarchyStructureFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::describe_user_hierarchy_structure::builders::DescribeUserHierarchyStructureFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    /// - On success, responds with [`DescribeUserHierarchyStructureOutput`](crate::operation::describe_user_hierarchy_structure::DescribeUserHierarchyStructureOutput) with field(s):
    ///   - [`hierarchy_structure(Option<HierarchyStructure>)`](crate::operation::describe_user_hierarchy_structure::DescribeUserHierarchyStructureOutput::hierarchy_structure): <p>Information about the hierarchy structure.</p>
    /// - On failure, responds with [`SdkError<DescribeUserHierarchyStructureError>`](crate::operation::describe_user_hierarchy_structure::DescribeUserHierarchyStructureError)
    pub fn describe_user_hierarchy_structure(
        &self,
    ) -> crate::operation::describe_user_hierarchy_structure::builders::DescribeUserHierarchyStructureFluentBuilder {
        crate::operation::describe_user_hierarchy_structure::builders::DescribeUserHierarchyStructureFluentBuilder::new(self.handle.clone())
    }
}
