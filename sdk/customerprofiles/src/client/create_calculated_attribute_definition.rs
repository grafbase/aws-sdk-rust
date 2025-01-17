// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCalculatedAttributeDefinition`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_domain_name): <p>The unique name of the domain.</p>
    ///   - [`calculated_attribute_name(impl ::std::convert::Into<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::calculated_attribute_name) / [`set_calculated_attribute_name(Option<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_calculated_attribute_name): <p>The unique name of the calculated attribute.</p>
    ///   - [`display_name(impl ::std::convert::Into<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::display_name) / [`set_display_name(Option<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_display_name): <p>The display name of the calculated attribute.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_description): <p>The description of the calculated attribute.</p>
    ///   - [`attribute_details(AttributeDetails)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::attribute_details) / [`set_attribute_details(Option<AttributeDetails>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_attribute_details): <p>Mathematical expression and a list of attribute items specified in that expression.</p>
    ///   - [`conditions(Conditions)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::conditions) / [`set_conditions(Option<Conditions>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_conditions): <p>The conditions including range, object count, and threshold for the calculated attribute.</p>
    ///   - [`statistic(Statistic)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::statistic) / [`set_statistic(Option<Statistic>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_statistic): <p>The aggregation operation to perform for the calculated attribute.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::set_tags): <p>The tags used to organize, track, or control access for this resource.</p>
    /// - On success, responds with [`CreateCalculatedAttributeDefinitionOutput`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput) with field(s):
    ///   - [`calculated_attribute_name(Option<String>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::calculated_attribute_name): <p>The unique name of the calculated attribute.</p>
    ///   - [`display_name(Option<String>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::display_name): <p>The display name of the calculated attribute.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::description): <p>The description of the calculated attribute.</p>
    ///   - [`attribute_details(Option<AttributeDetails>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::attribute_details): <p>Mathematical expression and a list of attribute items specified in that expression.</p>
    ///   - [`conditions(Option<Conditions>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::conditions): <p>The conditions including range, object count, and threshold for the calculated attribute.</p>
    ///   - [`statistic(Option<Statistic>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::statistic): <p>The aggregation operation to perform for the calculated attribute.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::created_at): <p>The timestamp of when the calculated attribute definition was created.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::last_updated_at): <p>The timestamp of when the calculated attribute definition was most recently edited.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionOutput::tags): <p>The tags used to organize, track, or control access for this resource.</p>
    /// - On failure, responds with [`SdkError<CreateCalculatedAttributeDefinitionError>`](crate::operation::create_calculated_attribute_definition::CreateCalculatedAttributeDefinitionError)
    pub fn create_calculated_attribute_definition(
        &self,
    ) -> crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder {
        crate::operation::create_calculated_attribute_definition::builders::CreateCalculatedAttributeDefinitionFluentBuilder::new(self.handle.clone())
    }
}
