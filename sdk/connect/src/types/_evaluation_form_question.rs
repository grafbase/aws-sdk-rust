// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a question from an evaluation form.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EvaluationFormQuestion {
    /// <p>The title of the question.</p>
    pub title: ::std::option::Option<::std::string::String>,
    /// <p>The instructions of the section.</p>
    pub instructions: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the question. An identifier must be unique within the evaluation form.</p>
    pub ref_id: ::std::option::Option<::std::string::String>,
    /// <p>The flag to enable not applicable answers to the question.</p>
    pub not_applicable_enabled: bool,
    /// <p>The type of the question.</p>
    pub question_type: ::std::option::Option<crate::types::EvaluationFormQuestionType>,
    /// <p>The properties of the type of question. Text questions do not have to define question type properties.</p>
    pub question_type_properties: ::std::option::Option<crate::types::EvaluationFormQuestionTypeProperties>,
    /// <p>The scoring weight of the section.</p>
    pub weight: f64,
}
impl EvaluationFormQuestion {
    /// <p>The title of the question.</p>
    pub fn title(&self) -> ::std::option::Option<&str> {
        self.title.as_deref()
    }
    /// <p>The instructions of the section.</p>
    pub fn instructions(&self) -> ::std::option::Option<&str> {
        self.instructions.as_deref()
    }
    /// <p>The identifier of the question. An identifier must be unique within the evaluation form.</p>
    pub fn ref_id(&self) -> ::std::option::Option<&str> {
        self.ref_id.as_deref()
    }
    /// <p>The flag to enable not applicable answers to the question.</p>
    pub fn not_applicable_enabled(&self) -> bool {
        self.not_applicable_enabled
    }
    /// <p>The type of the question.</p>
    pub fn question_type(&self) -> ::std::option::Option<&crate::types::EvaluationFormQuestionType> {
        self.question_type.as_ref()
    }
    /// <p>The properties of the type of question. Text questions do not have to define question type properties.</p>
    pub fn question_type_properties(&self) -> ::std::option::Option<&crate::types::EvaluationFormQuestionTypeProperties> {
        self.question_type_properties.as_ref()
    }
    /// <p>The scoring weight of the section.</p>
    pub fn weight(&self) -> f64 {
        self.weight
    }
}
impl EvaluationFormQuestion {
    /// Creates a new builder-style object to manufacture [`EvaluationFormQuestion`](crate::types::EvaluationFormQuestion).
    pub fn builder() -> crate::types::builders::EvaluationFormQuestionBuilder {
        crate::types::builders::EvaluationFormQuestionBuilder::default()
    }
}

/// A builder for [`EvaluationFormQuestion`](crate::types::EvaluationFormQuestion).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EvaluationFormQuestionBuilder {
    pub(crate) title: ::std::option::Option<::std::string::String>,
    pub(crate) instructions: ::std::option::Option<::std::string::String>,
    pub(crate) ref_id: ::std::option::Option<::std::string::String>,
    pub(crate) not_applicable_enabled: ::std::option::Option<bool>,
    pub(crate) question_type: ::std::option::Option<crate::types::EvaluationFormQuestionType>,
    pub(crate) question_type_properties: ::std::option::Option<crate::types::EvaluationFormQuestionTypeProperties>,
    pub(crate) weight: ::std::option::Option<f64>,
}
impl EvaluationFormQuestionBuilder {
    /// <p>The title of the question.</p>
    pub fn title(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.title = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The title of the question.</p>
    pub fn set_title(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.title = input;
        self
    }
    /// <p>The title of the question.</p>
    pub fn get_title(&self) -> &::std::option::Option<::std::string::String> {
        &self.title
    }
    /// <p>The instructions of the section.</p>
    pub fn instructions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instructions = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instructions of the section.</p>
    pub fn set_instructions(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instructions = input;
        self
    }
    /// <p>The instructions of the section.</p>
    pub fn get_instructions(&self) -> &::std::option::Option<::std::string::String> {
        &self.instructions
    }
    /// <p>The identifier of the question. An identifier must be unique within the evaluation form.</p>
    pub fn ref_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ref_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the question. An identifier must be unique within the evaluation form.</p>
    pub fn set_ref_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ref_id = input;
        self
    }
    /// <p>The identifier of the question. An identifier must be unique within the evaluation form.</p>
    pub fn get_ref_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ref_id
    }
    /// <p>The flag to enable not applicable answers to the question.</p>
    pub fn not_applicable_enabled(mut self, input: bool) -> Self {
        self.not_applicable_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>The flag to enable not applicable answers to the question.</p>
    pub fn set_not_applicable_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.not_applicable_enabled = input;
        self
    }
    /// <p>The flag to enable not applicable answers to the question.</p>
    pub fn get_not_applicable_enabled(&self) -> &::std::option::Option<bool> {
        &self.not_applicable_enabled
    }
    /// <p>The type of the question.</p>
    pub fn question_type(mut self, input: crate::types::EvaluationFormQuestionType) -> Self {
        self.question_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the question.</p>
    pub fn set_question_type(mut self, input: ::std::option::Option<crate::types::EvaluationFormQuestionType>) -> Self {
        self.question_type = input;
        self
    }
    /// <p>The type of the question.</p>
    pub fn get_question_type(&self) -> &::std::option::Option<crate::types::EvaluationFormQuestionType> {
        &self.question_type
    }
    /// <p>The properties of the type of question. Text questions do not have to define question type properties.</p>
    pub fn question_type_properties(mut self, input: crate::types::EvaluationFormQuestionTypeProperties) -> Self {
        self.question_type_properties = ::std::option::Option::Some(input);
        self
    }
    /// <p>The properties of the type of question. Text questions do not have to define question type properties.</p>
    pub fn set_question_type_properties(mut self, input: ::std::option::Option<crate::types::EvaluationFormQuestionTypeProperties>) -> Self {
        self.question_type_properties = input;
        self
    }
    /// <p>The properties of the type of question. Text questions do not have to define question type properties.</p>
    pub fn get_question_type_properties(&self) -> &::std::option::Option<crate::types::EvaluationFormQuestionTypeProperties> {
        &self.question_type_properties
    }
    /// <p>The scoring weight of the section.</p>
    pub fn weight(mut self, input: f64) -> Self {
        self.weight = ::std::option::Option::Some(input);
        self
    }
    /// <p>The scoring weight of the section.</p>
    pub fn set_weight(mut self, input: ::std::option::Option<f64>) -> Self {
        self.weight = input;
        self
    }
    /// <p>The scoring weight of the section.</p>
    pub fn get_weight(&self) -> &::std::option::Option<f64> {
        &self.weight
    }
    /// Consumes the builder and constructs a [`EvaluationFormQuestion`](crate::types::EvaluationFormQuestion).
    pub fn build(self) -> crate::types::EvaluationFormQuestion {
        crate::types::EvaluationFormQuestion {
            title: self.title,
            instructions: self.instructions,
            ref_id: self.ref_id,
            not_applicable_enabled: self.not_applicable_enabled.unwrap_or_default(),
            question_type: self.question_type,
            question_type_properties: self.question_type_properties,
            weight: self.weight.unwrap_or_default(),
        }
    }
}
