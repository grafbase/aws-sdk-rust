// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutConferencePreference`](crate::operation::put_conference_preference::builders::PutConferencePreferenceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`conference_preference(ConferencePreference)`](crate::operation::put_conference_preference::builders::PutConferencePreferenceFluentBuilder::conference_preference) / [`set_conference_preference(Option<ConferencePreference>)`](crate::operation::put_conference_preference::builders::PutConferencePreferenceFluentBuilder::set_conference_preference): <p>The conference preference of a specific conference provider.</p>
    /// - On success, responds with [`PutConferencePreferenceOutput`](crate::operation::put_conference_preference::PutConferencePreferenceOutput)
    /// - On failure, responds with [`SdkError<PutConferencePreferenceError>`](crate::operation::put_conference_preference::PutConferencePreferenceError)
    #[deprecated(note = "Alexa For Business is no longer supported")]
    pub fn put_conference_preference(&self) -> crate::operation::put_conference_preference::builders::PutConferencePreferenceFluentBuilder {
        crate::operation::put_conference_preference::builders::PutConferencePreferenceFluentBuilder::new(self.handle.clone())
    }
}
